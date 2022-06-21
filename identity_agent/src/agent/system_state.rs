// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::agent::ActorMap;
use crate::agent::AgentId;
use crate::agent::SystemConfig;

/// The internal state of a `System`.
#[derive(Debug)]
pub(crate) struct SystemState {
  pub(crate) agent_id: AgentId,
  pub(crate) config: SystemConfig,
  pub(crate) actors: ActorMap,
}
