// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod actor;
mod didcomm;

use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::Actor;
use crate::ActorBuilder;

async fn default_listening_actor() -> (Actor, Multiaddr, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();

  let mut listening_actor = ActorBuilder::new()
    .keypair(id_keys)
    .listen_on(addr.clone())
    .build()
    .await
    .unwrap();

  let addr = listening_actor.addresses().await.pop().unwrap();
  let peer_id = listening_actor.peer_id().await;

  (listening_actor, addr, peer_id)
}

async fn default_sending_actor() -> Actor {
  let keypair = Keypair::generate_ed25519();
  ActorBuilder::new().keypair(keypair).build().await.unwrap()
}
