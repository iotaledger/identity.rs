// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::future::Future;
use std::iter;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use crate::actor::Actor;
use crate::actor::ActorConfig;
use crate::actor::Error;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActorRequest;
use crate::actor::SyncHandler;
use crate::actor::SyncHandlerObject;
use crate::p2p::ActorProtocol;
use crate::p2p::ActorRequestResponseCodec;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use futures::channel::mpsc;
use futures::AsyncRead;
use futures::AsyncWrite;
use futures::FutureExt;
use libp2p::core::transport::upgrade;
use libp2p::core::Executor;
use libp2p::core::Transport;
use libp2p::identity::Keypair;
use libp2p::noise::Keypair as NoiseKeypair;
use libp2p::noise::NoiseConfig;
use libp2p::noise::X25519Spec;
use libp2p::request_response::ProtocolSupport;
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseConfig;
use libp2p::swarm::SwarmBuilder;
use libp2p::yamux::YamuxConfig;
use libp2p::Multiaddr;
use libp2p::Swarm;
use uuid::Uuid;

use super::actor::ObjectId;
use super::actor::ObjectMap;
use super::actor::SyncHandlerMap;
use super::ActorState;

/// An [`Actor`] builder for easy configuration and building of handler and hook functions.
pub struct ActorBuilder {
  pub(crate) listening_addresses: Vec<Multiaddr>,
  pub(crate) keypair: Option<Keypair>,
  pub(crate) config: ActorConfig,
  pub(crate) handlers: SyncHandlerMap,
  pub(crate) objects: ObjectMap,
}

impl ActorBuilder {
  /// Creates a new `ActorBuilder`.
  pub fn new() -> ActorBuilder {
    Self {
      listening_addresses: vec![],
      keypair: None,

      config: ActorConfig::default(),
      handlers: HashMap::new(),
      objects: HashMap::new(),
    }
  }

  /// Set the keypair from which the `PeerId` of the actor is derived.
  ///
  /// If unset, a new keypair is generated.
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.keypair = Some(keypair);
    self
  }

  /// Add a [`Multiaddr`] to listen on. This can be called multiple times.
  #[must_use]
  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.listening_addresses.push(address);
    self
  }

  /// Sets the timeout for the underlying libp2p [`RequestResponse`] protocol.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.config.timeout = timeout;
    self
  }

  /// Grants low-level access to the handler map for use in bindings.
  #[cfg(feature = "primitives")]
  pub fn handlers(&mut self) -> &mut SyncHandlerMap {
    &mut self.handlers
  }

  /// Grants low-level access to the object map for use in bindings.
  #[cfg(feature = "primitives")]
  pub fn objects(&mut self) -> &mut ObjectMap {
    &mut self.objects
  }

  /// Add a new shared state object and returns a [`ActorHandlerBuilder`] which can be used to
  /// attach handlers and hooks that operate on that object.
  pub fn add_state<OBJ>(&mut self, state_object: OBJ) -> ActorHandlerBuilder<OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
  {
    let object_id: ObjectId = Uuid::new_v4();
    self.objects.insert(object_id, Box::new(state_object));
    ActorHandlerBuilder {
      object_id,
      handler_map: &mut self.handlers,
      _marker_obj: PhantomData,
    }
  }

  /// Build the actor with a default transport which supports DNS, TCP and WebSocket capabilities.
  #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
  pub async fn build(self) -> ActorResult<Actor> {
    let dns_transport =
      libp2p::dns::TokioDnsConfig::system(libp2p::tcp::TokioTcpConfig::new()).map_err(|err| Error::TransportError {
        context: "unable to build transport",
        source: libp2p::TransportError::Other(err),
      })?;

    let transport = dns_transport
      .clone()
      .or_transport(libp2p::websocket::WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  /// Build the actor with a custom transport.
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> ActorResult<Actor>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let executor = Box::new(|fut| {
      cfg_if::cfg_if! {
        if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
          tokio::spawn(fut);
        } else {
          wasm_bindgen_futures::spawn_local(fut);
        }
      }
    });

    let (event_loop, actor_state, net_commander): (EventLoop, ActorState, NetCommander) =
      self.build_actor_constituents(transport, executor.clone()).await?;

    let actor: Actor = Actor::new(net_commander, Arc::new(actor_state));
    let actor_clone: Actor = actor.clone();

    let event_handler = move |event: InboundRequest| {
      actor_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(actor)
  }

  /// Build the actor constituents with a custom transport and custom executor.
  pub(crate) async fn build_actor_constituents<TRA>(
    self,
    transport: TRA,
    executor: Box<dyn Executor + Send>,
  ) -> ActorResult<(EventLoop, ActorState, NetCommander)>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let (noise_keypair, peer_id) = {
      let keypair: Keypair = self.keypair.unwrap_or_else(Keypair::generate_ed25519);
      let noise_keypair = NoiseKeypair::<X25519Spec>::new().into_authentic(&keypair).unwrap();
      let peer_id = keypair.public().to_peer_id();
      (noise_keypair, peer_id)
    };

    let mut swarm: Swarm<RequestResponse<ActorRequestResponseCodec>> = {
      let mut config: RequestResponseConfig = RequestResponseConfig::default();
      config.set_request_timeout(self.config.timeout);

      let behaviour = RequestResponse::new(
        ActorRequestResponseCodec(),
        iter::once((ActorProtocol(), ProtocolSupport::Full)),
        config,
      );

      let transport = transport
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(noise_keypair).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

      SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(executor)
        .build()
    };

    for addr in self.listening_addresses {
      swarm.listen_on(addr).map_err(|err| Error::TransportError {
        context: "unable to start listening",
        source: err,
      })?;
    }

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver);
    let net_commander = NetCommander::new(cmd_sender);

    let actor_state: ActorState = ActorState {
      handlers: self.handlers,
      objects: self.objects,
      peer_id,
      config: self.config,
    };

    Ok((event_loop, actor_state, net_commander))
  }
}

impl Default for ActorBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Used to attach handlers and hooks to an [`ActorBuilder`].
pub struct ActorHandlerBuilder<'builder, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  pub(crate) object_id: ObjectId,
  pub(crate) handler_map: &'builder mut SyncHandlerMap,
  pub(crate) _marker_obj: PhantomData<&'static OBJ>,
}

impl<'builder, OBJ> ActorHandlerBuilder<'builder, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  /// Add a synchronous handler function that operates on a shared state object and a
  /// [`SyncActorRequest`]. The function will be called if the actor receives a request
  /// on the given `endpoint` and can deserialize it into `REQ`. The handler is expected
  /// to return an instance of `REQ::Response`.
  pub fn add_sync_handler<REQ, FUT>(self, handler: fn(OBJ, Actor, RequestContext<REQ>) -> FUT) -> Self
  where
    REQ: SyncActorRequest + Sync,
    REQ::Response: Send,
    FUT: Future<Output = REQ::Response> + Send + 'static,
  {
    let handler = SyncHandler::new(handler);
    self.handler_map.insert(
      REQ::endpoint(),
      SyncHandlerObject::new(self.object_id, Box::new(handler)),
    );
    self
  }
}
