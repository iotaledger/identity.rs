// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

/// Configuration options for a [`System`](crate::actor::System).
#[derive(Debug, Clone)]
pub(crate) struct SystemConfig {
  pub(crate) timeout: Duration,
}

impl Default for SystemConfig {
  fn default() -> Self {
    Self {
      timeout: Duration::from_secs(30),
    }
  }
}
