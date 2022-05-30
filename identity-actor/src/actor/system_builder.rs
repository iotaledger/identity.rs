// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::iter;
use std::sync::Arc;
use std::time::Duration;

use crate::actor::ActorConfig;
use crate::actor::Error;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActorRequest;
use crate::actor::System;
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

use super::sync_actor::SyncActor;
use super::system::ObjectMap;
use super::system::SyncActorMap;
use super::AbstractSyncActor;
use super::SyncActorWrapper;
use super::SystemState;

/// An [`Actor`] builder for easy configuration and building of handler and hook functions.
pub struct SystemBuilder {
  pub(crate) listening_addresses: Vec<Multiaddr>,
  pub(crate) keypair: Option<Keypair>,
  pub(crate) config: ActorConfig,
  pub(crate) objects: ObjectMap,
  pub(crate) actors: SyncActorMap,
}

impl SystemBuilder {
  /// Creates a new `ActorBuilder`.
  pub fn new() -> SystemBuilder {
    Self {
      listening_addresses: vec![],
      keypair: None,
      config: ActorConfig::default(),
      objects: HashMap::new(),
      actors: HashMap::new(),
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

  /// Grants low-level access to the object map for use in bindings.
  #[cfg(feature = "primitives")]
  pub fn objects(&mut self) -> &mut ObjectMap {
    &mut self.objects
  }

  pub fn attach<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: SyncActor<REQ> + Send + Sync,
    REQ: SyncActorRequest + Send + Sync,
    REQ::Response: Send,
  {
    self.actors.insert(
      REQ::endpoint(),
      Box::new(SyncActorWrapper::new(actor)) as Box<dyn AbstractSyncActor>,
    );
  }

  /// Build the actor with a default transport which supports DNS, TCP and WebSocket capabilities.
  #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
  pub async fn build(self) -> ActorResult<System> {
    let dns_transport = libp2p::dns::TokioDnsConfig::system(libp2p::tcp::TokioTcpConfig::new())
      .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?;

    let transport = dns_transport
      .clone()
      .or_transport(libp2p::websocket::WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  /// Build the actor with a custom transport.
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> ActorResult<System>
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

    let (event_loop, actor_state, net_commander): (EventLoop, SystemState, NetCommander) =
      self.build_actor_constituents(transport, executor.clone()).await?;

    let actor: System = System::new(net_commander, Arc::new(actor_state));
    let actor_clone: System = actor.clone();

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
  ) -> ActorResult<(EventLoop, SystemState, NetCommander)>
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
      let noise_keypair = NoiseKeypair::<X25519Spec>::new()
        .into_authentic(&keypair)
        .expect("ed25519 keypair should be convertible into x25519");
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
      swarm
        .listen_on(addr)
        .map_err(|err| Error::TransportError("start listening", err))?;
    }

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver);
    let net_commander = NetCommander::new(cmd_sender);

    let actor_state: SystemState = SystemState {
      objects: self.objects,
      peer_id,
      config: self.config,
      actors: self.actors,
    };

    Ok((event_loop, actor_state, net_commander))
  }
}

impl Default for SystemBuilder {
  fn default() -> Self {
    Self::new()
  }
}
