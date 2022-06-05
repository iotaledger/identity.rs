// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod actor;
mod didcomm;
mod presentation;
mod remote_account;

use identity_core::crypto::KeyPair;
use identity_iota_core::document::IotaDocument;
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::actor::System;
use crate::actor::SystemBuilder;
use crate::didcomm::ActorIdentity;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommSystemBuilder;

fn try_init_logger() {
  let _ = pretty_env_logger::try_init();
}

async fn default_listening_system(f: impl FnOnce(SystemBuilder) -> SystemBuilder) -> (System, Vec<Multiaddr>, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = SystemBuilder::new().keypair(id_keys);

  builder = f(builder);

  let mut listening_system: System = builder.build().await.unwrap();

  let _ = listening_system.start_listening(addr).await.unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let peer_id = listening_system.peer_id();

  (listening_system, addrs, peer_id)
}

async fn default_sending_system(f: impl FnOnce(SystemBuilder) -> SystemBuilder) -> System {
  let mut builder = SystemBuilder::new();

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_sending_didcomm_system(f: impl FnOnce(DidCommSystemBuilder) -> DidCommSystemBuilder) -> DidCommSystem {
  let mut builder = DidCommSystemBuilder::new().identity(default_identity());

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_listening_didcomm_system(
  f: impl FnOnce(DidCommSystemBuilder) -> DidCommSystemBuilder,
) -> (DidCommSystem, Vec<Multiaddr>, PeerId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = DidCommSystemBuilder::new()
    .keypair(id_keys)
    .identity(default_identity());

  builder = f(builder);

  let mut listening_actor: DidCommSystem = builder.build().await.unwrap();

  let _ = listening_actor.start_listening(addr).await.unwrap();
  let addrs = listening_actor.addresses().await.unwrap();

  let peer_id = listening_actor.peer_id();

  (listening_actor, addrs, peer_id)
}

fn default_identity() -> ActorIdentity {
  let keypair: KeyPair = KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();

  ActorIdentity {
    document: IotaDocument::new(&keypair).unwrap(),
  }
}
