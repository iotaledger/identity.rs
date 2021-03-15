// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

use crate::types::Metadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialMetadata {
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
}

impl CredentialMetadata {
  pub fn new() -> Self {
    Self {
      created_at: Timestamp::now(),
      updated_at: Timestamp::now(),
    }
  }
}

impl Metadata for CredentialMetadata {
  fn tag(&self) -> &str {
    ""
  }

  fn ident(&self) -> &str {
    ""
  }

  fn index(&self) -> u32 {
    0
  }
}
