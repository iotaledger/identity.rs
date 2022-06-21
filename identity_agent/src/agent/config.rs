// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

/// Configuration options for a [`Agent`](crate::actor::Agent).
#[derive(Debug, Clone)]
pub(crate) struct AgentConfig {
  pub(crate) timeout: Duration,
}

impl Default for AgentConfig {
  fn default() -> Self {
    Self {
      timeout: Duration::from_secs(30),
    }
  }
}
