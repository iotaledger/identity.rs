// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::iter;

use crate::invocation::AsynchronousInvocationStrategy;
use crate::invocation::SynchronousInvocationStrategy;
use crate::p2p::behaviour::DidCommCodec;
use crate::p2p::behaviour::DidCommProtocol;
use crate::p2p::event_loop::EventLoop;
use crate::p2p::event_loop::InboundRequest;
use crate::p2p::net_commander::NetCommander;
use crate::Actor;
use crate::Error;
use crate::RequestMode;
use crate::Result;
use dashmap::DashMap;
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
use libp2p::swarm::SwarmBuilder;
use libp2p::tcp::TokioTcpConfig;
use libp2p::websocket::WsConfig;
use libp2p::yamux::YamuxConfig;
use libp2p::Multiaddr;
use libp2p::Swarm;

/// An [`Actor`] builder for easy configuration.
pub struct ActorBuilder {
  listening_addresses: Vec<Multiaddr>,
  keypair: Option<Keypair>,
}

impl ActorBuilder {
  /// Create a new `ActorBuilder`.
  pub fn new() -> Self {
    Self {
      listening_addresses: vec![],
      keypair: None,
    }
  }

  /// Set the keypair from which the `PeerId` of the actor is derived.
  ///
  /// If unset, a new keypair is generated.
  #[must_use]
  pub fn keypair(mut self, keys: Keypair) -> Self {
    self.keypair = Some(keys);
    self
  }

  /// Add a [`Multiaddr`] to listen on. This can be called multiple times.
  #[must_use]
  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.listening_addresses.push(address);
    self
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

    let mut swarm: Swarm<RequestResponse<DidCommCodec>> = {
      let behaviour = RequestResponse::new(
        DidCommCodec(),
        iter::once((DidCommProtocol(), ProtocolSupport::Full)),
        Default::default(),
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

    let handlers = DashMap::new();
    let objects = DashMap::new();

    let actor = Actor::from_builder(swarm_commander.clone(), handlers, objects, peer_id).await?;

    let actor_clone = actor.clone();

    let event_handler = move |event: InboundRequest| {
      let actor = actor_clone.clone();

      if event.request_mode == RequestMode::Asynchronous {
        actor.handle_request(AsynchronousInvocationStrategy::new(), event);
      } else {
        actor.handle_request(SynchronousInvocationStrategy::new(), event);
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
