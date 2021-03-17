// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

use crate::types::Metadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialMetadata {
  pub(crate) id: String,
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
}

impl CredentialMetadata {
  pub fn new() -> Self {
    Self {
      id: String::new(),
      created_at: Timestamp::now(),
      updated_at: Timestamp::now(),
    }
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn created_at(&self) -> Timestamp {
    self.created_at
  }

  pub fn updated_at(&self) -> Timestamp {
    self.updated_at
  }
}

impl Metadata for CredentialMetadata {
  fn tag(&self) -> &str {
    &self.id
  }

  fn ident(&self) -> &str {
    ""
  }

  fn index(&self) -> u32 {
    0
  }
}
