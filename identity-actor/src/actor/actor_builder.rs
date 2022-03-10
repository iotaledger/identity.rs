// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::future::Future;
use std::iter;
use std::marker::PhantomData;
use std::time::Duration;

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
use crate::AsynchronousInvocationStrategy;
use crate::Endpoint;
use crate::Error;
use crate::Handler;
use crate::HandlerObject;
use crate::RequestContext;
use crate::RequestMode;
use crate::Result;
use crate::SyncMode;
use crate::Synchronous;
use crate::SynchronousInvocationStrategy;
use futures::channel::mpsc;
use futures::AsyncRead;
use futures::AsyncWrite;
use futures::FutureExt;
use libp2p::core::transport::upgrade;
use libp2p::core::Executor;
use libp2p::core::Transport;
use libp2p::dns::TokioDnsConfig;
use libp2p::identity::Keypair;
use libp2p::noise::Keypair as NoiseKeypair;
use libp2p::noise::NoiseConfig;
use libp2p::noise::X25519Spec;
use libp2p::request_response::ProtocolSupport;
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseConfig;
use libp2p::swarm::SwarmBuilder;
use libp2p::tcp::TokioTcpConfig;
use libp2p::websocket::WsConfig;
use libp2p::yamux::YamuxConfig;
use libp2p::Multiaddr;
use libp2p::Swarm;
use uuid::Uuid;

use super::actor::HandlerMap;
use super::actor::ObjectId;
use super::actor::ObjectMap;

/// An [`Actor`] builder for easy configuration and building of handler and hook functions.
pub struct ActorBuilder {
  listening_addresses: Vec<Multiaddr>,
  keypair: Option<Keypair>,
  config: ActorConfig,
  handler_map: HandlerMap,
  object_map: ObjectMap,
}

impl ActorBuilder {
  /// Creates a new `ActorBuilder`.
  pub fn new() -> Self {
    Self {
      listening_addresses: vec![],
      keypair: None,
      config: ActorConfig::default(),
      handler_map: HashMap::new(),
      object_map: HashMap::new(),
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

  /// Add a new shared state object and returns a [`HandlerBuilder`] which can be used to
  /// attach handlers and hooks that operate on that object.
  pub fn add_state<MOD: SyncMode, OBJ>(&mut self, state_object: OBJ) -> HandlerBuilder<MOD, OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
  {
    let object_id: ObjectId = Uuid::new_v4();
    self.object_map.insert(object_id, Box::new(state_object));
    HandlerBuilder {
      object_id,
      handler_map: &mut self.handler_map,
      _marker_obj: PhantomData,
      _marker_mod: PhantomData,
    }
  }

  /// Build the actor with a default transport which supports DNS, TCP and WebSocket capabilities.
  pub async fn build(self) -> Result<Actor> {
    let dns_transport = TokioDnsConfig::system(TokioTcpConfig::new())
      .map_err(|err| Error::TransportError(libp2p::TransportError::Other(err)))?;
    let transport = dns_transport.clone().or_transport(WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  /// Build the actor with a custom transport.
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> Result<Actor>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let (noise_keypair, peer_id) = {
      let keypair = self.keypair.unwrap_or_else(Keypair::generate_ed25519);
      let noise_keypair = NoiseKeypair::<X25519Spec>::new().into_authentic(&keypair).unwrap();
      let peer_id = keypair.public().to_peer_id();
      (noise_keypair, peer_id)
    };

    let executor = Box::new(|fut| {
      tokio::spawn(fut);
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

    for addr in self.listening_addresses {
      swarm.listen_on(addr).map_err(Error::TransportError)?;
    }

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver);
    let swarm_commander = NetCommander::new(cmd_sender);

    let actor = Actor::from_builder(
      swarm_commander.clone(),
      self.handler_map,
      self.object_map,
      peer_id,
      self.config,
    )
    .await?;

    let actor_clone = actor.clone();

    let event_handler = move |event: InboundRequest| {
      let actor = actor_clone.clone();

      if event.request_mode == RequestMode::Asynchronous {
        actor.handle_request::<AsynchronousInvocationStrategy>(event);
      } else {
        actor.handle_request::<SynchronousInvocationStrategy>(event);
      };
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
pub struct HandlerBuilder<'builder, MOD: SyncMode, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
  MOD: 'static,
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
  pub fn add_sync_handler<REQ, FUT, FUN>(self, endpoint: &'static str, handler: FUN) -> Result<Self>
  where
    REQ: ActorRequest<Synchronous> + Sync,
    REQ::Response: Send,
    FUT: Future<Output = REQ::Response> + Send + 'static,
    FUN: 'static + Send + Sync + Fn(OBJ, Actor, RequestContext<REQ>) -> FUT,
  {
    let handler = Handler::new(handler);
    self.handler_map.insert(
      Endpoint::new(endpoint)?,
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
  pub fn add_async_handler<REQ, FUT, FUN>(self, endpoint: &'static str, handler: FUN) -> Result<Self>
  where
    REQ: ActorRequest<Asynchronous> + Sync,
    FUT: Future<Output = ()> + Send + 'static,
    FUN: 'static + Send + Sync + Fn(OBJ, Actor, RequestContext<DidCommPlaintextMessage<REQ>>) -> FUT,
  {
    let handler = Handler::new(handler);
    self.handler_map.insert(
      Endpoint::new(endpoint)?,
      HandlerObject::new(self.object_id, Box::new(handler)),
    );
    Ok(self)
  }
}
