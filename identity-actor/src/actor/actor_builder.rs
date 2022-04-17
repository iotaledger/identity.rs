// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::future::Future;
use std::iter;
use std::marker::PhantomData;
use std::time::Duration;

use crate::didcomm::didcomm_actor::DidCommActor;
use crate::didcomm::message::DidCommPlaintextMessage;
use crate::p2p::ActorProtocol;
use crate::p2p::ActorRequestResponseCodec;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::Actor;
use crate::ActorConfig;
use crate::ActorRequest;
use crate::Asynchronous;
use crate::Endpoint;
use crate::Error;
use crate::GenericActor;
use crate::Handler;
use crate::HandlerObject;
use crate::RequestContext;
use crate::Result;
use crate::SyncMode;
use crate::Synchronous;
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

use super::actor::HandlerMap;
use super::actor::ObjectId;
use super::actor::ObjectMap;

/// An [`Actor`] builder for easy configuration and building of handler and hook functions.
pub struct ActorBuilder {
  pub(crate) listening_addresses: Vec<Multiaddr>,
  pub(crate) keypair: Option<Keypair>,
  pub(crate) config: ActorConfig,
  pub(crate) handlers: HandlerMap,
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

  /// Sets the timeout for [`Actor::await_message`] and the underlying libp2p protocol timeout.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.config.timeout = timeout;
    self
  }

  /// Grants low-level access to the handler map for use in bindings.
  #[cfg(feature = "primitives")]
  pub fn handlers(&mut self) -> &mut HandlerMap {
    &mut self.handlers
  }

  /// Grants low-level access to the object map for use in bindings.
  #[cfg(feature = "primitives")]
  pub fn objects(&mut self) -> &mut ObjectMap {
    &mut self.objects
  }

  /// Add a new shared state object and returns a [`HandlerBuilder`] which can be used to
  /// attach handlers and hooks that operate on that object.
  pub fn add_state<MOD, OBJ>(&mut self, state_object: OBJ) -> HandlerBuilder<MOD, OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
    MOD: SyncMode,
  {
    let object_id: ObjectId = Uuid::new_v4();
    self.objects.insert(object_id, Box::new(state_object));
    HandlerBuilder {
      object_id,
      handler_map: &mut self.handlers,
      _marker_obj: PhantomData,
      _marker_mod: PhantomData,
    }
  }

  /// Build the actor with a default transport which supports DNS, TCP and WebSocket capabilities.
  #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
  pub async fn build<ACT: GenericActor>(self) -> Result<ACT> {
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
  pub async fn build_with_transport<ACT, TRA>(mut self, transport: TRA) -> Result<ACT>
  where
    ACT: GenericActor,
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    // TODO: (maybe), quick hack to avoid moving keypair out of the builder.
    let gen_keypair = Keypair::generate_ed25519();
    let (noise_keypair, peer_id) = {
      let keypair: &Keypair = self.keypair.as_ref().unwrap_or(&gen_keypair);
      let noise_keypair = NoiseKeypair::<X25519Spec>::new().into_authentic(keypair).unwrap();
      let peer_id = keypair.public().to_peer_id();
      (noise_keypair, peer_id)
    };

    let executor = Box::new(|fut| {
      cfg_if::cfg_if! {
        if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
          tokio::spawn(fut);
        } else {
          wasm_bindgen_futures::spawn_local(fut);
        }
      }
    });

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
        .executor(executor.clone())
        .build()
    };

    for addr in std::mem::take(&mut self.listening_addresses) {
      swarm.listen_on(addr).map_err(|err| Error::TransportError {
        context: "unable to start listening",
        source: err,
      })?;
    }

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver);
    let net_commander = NetCommander::new(cmd_sender);

    let actor = ACT::from_actor_builder(self, peer_id, net_commander)?;

    let actor_clone = actor.clone();

    let event_handler = move |event: InboundRequest| {
      let actor = actor_clone.clone();

      // TODO: Should be GenericActor::handle_request
      actor.handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(actor)
  }
}

impl Default for ActorBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Used to attach handlers and hooks to an [`ActorBuilder`].
pub struct HandlerBuilder<'builder, MOD, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
  MOD: SyncMode + 'static,
{
  pub(crate) object_id: ObjectId,
  pub(crate) handler_map: &'builder mut HandlerMap,
  _marker_obj: PhantomData<&'static OBJ>,
  _marker_mod: PhantomData<&'static MOD>,
}

impl<'builder, OBJ> HandlerBuilder<'builder, Synchronous, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  /// Add a synchronous handler function that operates on a shared state object and some
  /// [`ActorRequest`]. The function will be called if the actor receives a request
  /// on the given `endpoint` and can deserialize it into `REQ`. The handler is expected
  /// to return an instance of `REQ::Response`.
  pub fn add_sync_handler<REQ, FUT>(self, handler: fn(OBJ, Actor, RequestContext<REQ>) -> FUT) -> Result<Self>
  where
    REQ: ActorRequest<Synchronous> + Sync,
    REQ::Response: Send,
    FUT: Future<Output = REQ::Response> + Send + 'static,
  {
    let handler = Handler::new(handler);
    self.handler_map.insert(
      Endpoint::new(REQ::endpoint())?,
      HandlerObject::new(self.object_id, Box::new(handler)),
    );
    Ok(self)
  }
}

impl<'builder, OBJ> HandlerBuilder<'builder, Asynchronous, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  /// Add an asynchronous handler function that operates on a shared state object and some
  /// [`ActorRequest`]. The function will be called if the actor receives a request
  /// on the given `endpoint` and can deserialize it into `DidCommPlaintextMessage<REQ>`.
  /// The handler is not expected to return anything.
  pub fn add_async_handler<REQ, FUT>(
    self,
    handler: fn(OBJ, DidCommActor, RequestContext<DidCommPlaintextMessage<REQ>>) -> FUT,
  ) -> Result<Self>
  where
    REQ: ActorRequest<Asynchronous> + Sync,
    FUT: Future<Output = ()> + Send + 'static,
  {
    let handler = Handler::new(handler);
    self.handler_map.insert(
      Endpoint::new(REQ::endpoint())?,
      HandlerObject::new(self.object_id, Box::new(handler)),
    );
    Ok(self)
  }
}
