// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod actor;
mod didcomm;
#[cfg(feature = "account")]
mod remote_account;

use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::Actor;
use crate::ActorBuilder;

fn try_init_logger() {
  let _ = pretty_env_logger::try_init();
}

async fn default_listening_actor(f: impl FnOnce(&mut ActorBuilder)) -> (Actor, Multiaddr, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = ActorBuilder::new().keypair(id_keys);

  f(&mut builder);

  let mut listening_actor: Actor = builder.build().await.unwrap();

  let addr = listening_actor.start_listening(addr).await.unwrap();
  let peer_id = listening_actor.peer_id();

  (listening_actor, addr, peer_id)
}

async fn default_sending_actor(f: impl FnOnce(&mut ActorBuilder)) -> Actor {
  let mut builder = ActorBuilder::new();

  f(&mut builder);

  builder.build().await.unwrap()
}
