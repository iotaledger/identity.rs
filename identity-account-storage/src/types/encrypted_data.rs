// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
 
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  tag: Vec<u8>,
  cypher_text: Vec<u8>,
}

impl EncryptedData {
  pub fn new(tag: Vec<u8>, cypher_text: Vec<u8>) -> Self {
    Self { tag, cypher_text }
  }

  pub fn cypher_text(&self) -> &[u8] {
    &self.cypher_text
  }

  pub fn tag(&self) -> &[u8] {
    &self.tag
  }
}
