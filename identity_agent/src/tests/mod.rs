// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod didcomm;
mod handler;
mod presentation;
mod remote_account;

use libp2p::identity::Keypair;
use libp2p::Multiaddr;

use crate::agent::Agent;
use crate::agent::AgentBuilder;
use crate::agent::AgentId;
use crate::didcomm::DidCommAgent;
use crate::didcomm::DidCommAgentBuilder;
use crate::didcomm::DidCommAgentIdentity;

fn try_init_logger() {
  let _ = pretty_env_logger::try_init();
}

async fn default_listening_agent(f: impl FnOnce(AgentBuilder) -> AgentBuilder) -> (Agent, Vec<Multiaddr>, AgentId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = AgentBuilder::new().keypair(id_keys);

  builder = f(builder);

  let mut listening_agent: Agent = builder.build().await.unwrap();

  let _ = listening_agent.start_listening(addr).await.unwrap();
  let addrs = listening_agent.addresses().await.unwrap();

  let agent_id = listening_agent.agent_id();

  (listening_agent, addrs, agent_id)
}

async fn default_sending_agent(f: impl FnOnce(AgentBuilder) -> AgentBuilder) -> Agent {
  let mut builder = AgentBuilder::new();

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_sending_didcomm_agent(f: impl FnOnce(DidCommAgentBuilder) -> DidCommAgentBuilder) -> DidCommAgent {
  let mut builder = DidCommAgentBuilder::new().identity(default_identity());

  builder = f(builder);

  builder.build().await.unwrap()
}

async fn default_listening_didcomm_agent(
  f: impl FnOnce(DidCommAgentBuilder) -> DidCommAgentBuilder,
) -> (DidCommAgent, Vec<Multiaddr>, AgentId) {
  let id_keys = Keypair::generate_ed25519();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = DidCommAgentBuilder::new().keypair(id_keys).identity(default_identity());

  builder = f(builder);

  let mut listening_agent: DidCommAgent = builder.build().await.unwrap();

  let _ = listening_agent.start_listening(addr).await.unwrap();
  let addrs = listening_agent.addresses().await.unwrap();

  let agent_id = listening_agent.agent_id();

  (listening_agent, addrs, agent_id)
}

fn default_identity() -> DidCommAgentIdentity {
  DidCommAgentIdentity::new()
}
