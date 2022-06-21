// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::agent::ActorMap;
use crate::agent::AgentConfig;
use crate::agent::AgentId;

/// The internal state of a `Agent`.
#[derive(Debug)]
pub(crate) struct AgentState {
  pub(crate) agent_id: AgentId,
  pub(crate) config: AgentConfig,
  pub(crate) actors: ActorMap,
}
