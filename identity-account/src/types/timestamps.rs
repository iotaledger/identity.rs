// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timestamps {
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
}

impl Timestamps {
  pub fn now() -> Self {
    let now: Timestamp = Timestamp::now();

    Self {
      created_at: now,
      updated_at: now,
    }
  }

  pub const fn created_at(&self) -> Timestamp {
    self.created_at
  }

  pub const fn updated_at(&self) -> Timestamp {
    self.updated_at
  }
}
