// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::ActorMap;
use crate::actor::AgentId;
use crate::actor::SystemConfig;

/// The internal state of a `System`.
#[derive(Debug)]
pub(crate) struct SystemState {
  pub(crate) agent_id: AgentId,
  pub(crate) config: SystemConfig,
  pub(crate) actors: ActorMap,
}
