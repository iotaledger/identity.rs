// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

/// Configuration options for an `Actor`.
#[derive(Debug, Clone)]
pub(crate) struct ActorConfig {
  pub(crate) timeout: Duration,
}

impl Default for ActorConfig {
  fn default() -> Self {
    Self {
      timeout: Duration::from_secs(30),
    }
  }
}
