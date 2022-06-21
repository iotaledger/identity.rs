// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::iter;
use std::sync::Arc;
use std::time::Duration;

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
use libp2p::Swarm;

use crate::actor::AbstractActor;
use crate::actor::Actor;
use crate::actor::ActorMap;
use crate::actor::ActorRequest;
use crate::actor::ActorWrapper;
use crate::actor::Error;
use crate::actor::Result as AgentResult;
use crate::actor::System;
use crate::actor::SystemConfig;
use crate::actor::SystemState;
use crate::p2p::AgentProtocol;
use crate::p2p::AgentRequestResponseCodec;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

/// A builder for [`System`]s to customize its configuration and attach actors.
pub struct SystemBuilder {
  pub(crate) keypair: Option<Keypair>,
  pub(crate) config: SystemConfig,
  pub(crate) actors: ActorMap,
}

impl SystemBuilder {
  /// Create a new builder with the default configuration.
  pub fn new() -> SystemBuilder {
    Self {
      keypair: None,
      config: SystemConfig::default(),
      actors: HashMap::new(),
    }
  }

  /// Set the keypair from which the `AgentId` of the system is derived.
  ///
  /// If unset, a new keypair is generated.
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.keypair = Some(keypair);
    self
  }

  /// Sets the timeout for the underlying libp2p [`RequestResponse`] protocol.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.config.timeout = timeout;
    self
  }

  /// Attaches an [`Actor`] to this system.
  ///
  /// This means that when the system receives a request of type `REQ`, it will invoke this actor.
  ///
  /// Calling this method with a `REQ` type whose endpoint is already attached to an actor
  /// will overwrite the previous attachment.
  pub fn attach<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: Actor<REQ> + Send + Sync,
    REQ: ActorRequest + Send + Sync,
    REQ::Response: Send,
  {
    self.actors.insert(
      REQ::endpoint(),
      Box::new(ActorWrapper::new(actor)) as Box<dyn AbstractActor>,
    );
  }

  /// Build the actor with a default transport which supports DNS, TCP and WebSocket capabilities.
  pub async fn build(self) -> AgentResult<System> {
    let transport: _ = {
      let dns_tcp_transport: TokioDnsConfig<_> = TokioDnsConfig::system(TokioTcpConfig::new().nodelay(true))
        .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?;
      let ws_transport: WsConfig<_> = WsConfig::new(
        TokioDnsConfig::system(TokioTcpConfig::new().nodelay(true))
          .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?,
      );
      dns_tcp_transport.or_transport(ws_transport)
    };

    self.build_with_transport(transport).await
  }

  /// Build the system with a custom transport.
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> AgentResult<System>
  where
    TRA: Transport + Sized + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let executor = Box::new(|fut| {
      tokio::spawn(fut);
    });

    let (event_loop, actor_state, net_commander): (EventLoop, SystemState, NetCommander) =
      self.build_constituents(transport, executor.clone()).await?;

    let system: System = System::new(net_commander, Arc::new(actor_state));
    let system_clone: System = system.clone();

    let event_handler = move |event: InboundRequest| {
      system_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(system)
  }

  /// Build the system constituents with a custom transport and custom executor.
  pub(crate) async fn build_constituents<TRA>(
    self,
    transport: TRA,
    executor: Box<dyn Executor + Send>,
  ) -> AgentResult<(EventLoop, SystemState, NetCommander)>
  where
    TRA: Transport + Sized + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let (noise_keypair, agent_id) = {
      let keypair: Keypair = self.keypair.unwrap_or_else(Keypair::generate_ed25519);
      let noise_keypair = NoiseKeypair::<X25519Spec>::new()
        .into_authentic(&keypair)
        .expect("ed25519 keypair should be convertible into x25519");
      let agent_id = keypair.public().to_peer_id();
      (noise_keypair, agent_id)
    };

    let swarm: Swarm<RequestResponse<AgentRequestResponseCodec>> = {
      let mut config: RequestResponseConfig = RequestResponseConfig::default();
      config.set_request_timeout(self.config.timeout);

      let behaviour = RequestResponse::new(
        AgentRequestResponseCodec(),
        iter::once((AgentProtocol(), ProtocolSupport::Full)),
        config,
      );

      let transport = transport
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(noise_keypair).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

      SwarmBuilder::new(transport, behaviour, agent_id)
        .executor(executor)
        .build()
    };

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver);
    let net_commander = NetCommander::new(cmd_sender);

    let system_state: SystemState = SystemState {
      agent_id,
      config: self.config,
      actors: self.actors,
    };

    Ok((event_loop, system_state, net_commander))
  }
}

impl Default for SystemBuilder {
  fn default() -> Self {
    Self::new()
  }
}
