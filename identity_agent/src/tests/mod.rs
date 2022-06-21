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

use crate::actor::AgentId;
use crate::actor::System;
use crate::actor::SystemBuilder;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommSystemBuilder;
use crate::didcomm::DidCommSystemIdentity;

fn try_init_logger() {
  let _ = pretty_env_logger::try_init();
}

async fn default_listening_system(f: impl FnOnce(SystemBuilder) -> SystemBuilder) -> (System, Vec<Multiaddr>, AgentId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = SystemBuilder::new().keypair(id_keys);

  builder = f(builder);

  let mut listening_system: System = builder.build().await.unwrap();

  let _ = listening_system.start_listening(addr).await.unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let agent_id = listening_system.agent_id();

  (listening_system, addrs, agent_id)
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
) -> (DidCommSystem, Vec<Multiaddr>, AgentId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = DidCommSystemBuilder::new()
    .keypair(id_keys)
    .identity(default_identity());

  builder = f(builder);

  let mut listening_system: DidCommSystem = builder.build().await.unwrap();

  let _ = listening_system.start_listening(addr).await.unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let agent_id = listening_system.agent_id();

  (listening_system, addrs, agent_id)
}

fn default_identity() -> DidCommSystemIdentity {
  let keypair: KeyPair = KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();

  DidCommSystemIdentity {
    document: IotaDocument::new(&keypair).unwrap(),
  }
}
