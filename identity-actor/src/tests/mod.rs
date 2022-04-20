// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod actor;
mod didcomm;
#[cfg(feature = "account")]
mod remote_account;

use std::collections::HashMap;

use identity_core::crypto::KeyPair;
use identity_iota_core::document::IotaDocument;
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::actor::Actor;
use crate::actor::ActorBuilder;
use crate::didcomm::ActorIdentity;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommActorBuilder;

fn try_init_logger() {
  let _ = pretty_env_logger::try_init();
}

async fn default_listening_actor(f: impl FnOnce(ActorBuilder) -> ActorBuilder) -> (Actor, Vec<Multiaddr>, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = ActorBuilder::new().keypair(id_keys);

  builder = f(builder);

  let mut listening_actor: Actor = builder.build().await.unwrap();

  let _ = listening_actor.start_listening(addr).await.unwrap();
  let addrs = listening_actor.addresses().await.unwrap();

  let peer_id = listening_actor.peer_id();

  (listening_actor, addrs, peer_id)
}

async fn default_sending_actor(f: impl FnOnce(ActorBuilder) -> ActorBuilder) -> Actor {
  let mut builder = ActorBuilder::new();

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_sending_didcomm_actor(f: impl FnOnce(DidCommActorBuilder) -> DidCommActorBuilder) -> DidCommActor {
  let mut builder = DidCommActorBuilder::new().identity(default_identity());

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_listening_didcomm_actor(
  f: impl FnOnce(DidCommActorBuilder) -> DidCommActorBuilder,
) -> (DidCommActor, Vec<Multiaddr>, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = DidCommActorBuilder::new().keypair(id_keys).identity(default_identity());

  builder = f(builder);

  let mut listening_actor: DidCommActor = builder.build().await.unwrap();

  let _ = listening_actor.start_listening(addr).await.unwrap();
  let addrs = listening_actor.addresses().await.unwrap();

  let peer_id = listening_actor.peer_id();

  (listening_actor, addrs, peer_id)
}

fn default_identity() -> ActorIdentity {
  let keypair: KeyPair = KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();

  ActorIdentity {
    doc: IotaDocument::new(&keypair).unwrap(),
    keypairs: HashMap::new(),
  }
}
