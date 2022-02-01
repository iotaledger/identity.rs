// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::iter;

use crate::p2p::behaviour::DidCommCodec;
use crate::p2p::behaviour::DidCommProtocol;
use crate::p2p::event_loop::EventLoop;
use crate::p2p::net_commander::NetCommander;
use crate::Actor;
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

pub struct ActorBuilder {
  // receiver: mpsc::Receiver<ReceiveRequest<RequestMessage, ResponseMessage>>,
  // comm_builder: StrongholdP2pBuilder<RequestMessage, ResponseMessage>,
  // commander: EventLoopInstructor,
  listening_addresses: Vec<Multiaddr>,
  keypair: Option<Keypair>,
}

impl ActorBuilder {
  pub fn new() -> Self {
    Self {
      listening_addresses: vec![],
      keypair: None,
    }
  }

  pub async fn build(self) -> Result<Actor> {
    let dns_transport = TokioDnsConfig::system(TokioTcpConfig::new())?;
    let transport = dns_transport.clone().or_transport(WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  // pub async fn build_with_transport_and_executor<TRA, EXE>(self, transport: TRA, executor: EXE) -> Result<Actor>
  // where
  //   TRA: Transport + Sized + Clone + Send + Sync + 'static,
  //   TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
  //   TRA::Dial: Send + 'static,
  //   TRA::Listener: Send + 'static,
  //   TRA::ListenerUpgrade: Send + 'static,
  //   TRA::Error: Send + Sync,
  //   EXE: Executor + Send + 'static + Clone,
  // {
  //   let comm = self.comm_builder.build_with_transport(transport, executor).await;
  //   let handlers = DashMap::new();
  //   let objects = DashMap::new();
  //   Actor::from_builder(self.receiver, comm, handlers, objects, self.listening_addresses).await
  // }

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
      let keypair = self.keypair.unwrap_or_else(|| Keypair::generate_ed25519());
      let noise_keypair = NoiseKeypair::<X25519Spec>::new().into_authentic(&keypair).unwrap();
      let peer_id = keypair.public().to_peer_id();
      (noise_keypair, peer_id)
    };

    let executor = Box::new(|fut| {
      tokio::spawn(fut);
    });

    let swarm: Swarm<RequestResponse<DidCommCodec>> = {
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

    let (cmd_sender, cmd_receiver) = mpsc::channel(10);
    let (event_sender, event_receiver) = mpsc::channel(10);

    let event_loop = EventLoop::new(swarm, cmd_receiver, event_sender);
    let swarm_commander = NetCommander::new(cmd_sender);

    executor.exec(event_loop.run().boxed());

    let handlers = DashMap::new();
    let objects = DashMap::new();

    Actor::from_builder(
      event_receiver,
      swarm_commander,
      handlers,
      objects,
      self.listening_addresses,
    )
    .await
  }

  pub fn keypair(mut self, keys: Keypair) -> Self {
    self.keypair = Some(keys);
    self
  }

  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.listening_addresses.push(address);
    self
  }
}

impl Default for ActorBuilder {
  fn default() -> Self {
    Self::new()
  }
}
