// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod actor;
mod comm;
mod storage;

use libp2p::tcp::TcpConfig;
use libp2p::Multiaddr;
use libp2p::PeerId;
use p2p::InitKeypair;
use p2p::Keypair;

use crate::actor_builder::ActorBuilder;
use crate::Actor;

async fn default_listening_actor() -> (Actor, Multiaddr, PeerId) {
  let id_keys = Keypair::generate_ed25519();
  let transport = TcpConfig::new().nodelay(true);

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();

  let mut listening_actor = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .listen_on(addr.clone())
    .build_with_transport(transport)
    .await
    .unwrap();

  let addr = listening_actor.addrs().await.pop().unwrap();
  let peer_id = listening_actor.peer_id();

  (listening_actor, addr, peer_id)
}

async fn default_sending_actor() -> Actor {
  let id_keys = Keypair::generate_ed25519();

  ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .build()
    .await
    .unwrap()
}
