// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// The structure returned after encrypting data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  nonce: Vec<u8>,
  tag: Vec<u8>,
  cypher_text: Vec<u8>,
}

impl EncryptedData {
  pub fn new(nonce: Vec<u8>, tag: Vec<u8>, cypher_text: Vec<u8>) -> Self {
    Self {
      nonce,
      tag,
      cypher_text,
    }
  }

  pub fn cypher_text(&self) -> &[u8] {
    &self.cypher_text
  }

  pub fn tag(&self) -> &[u8] {
    &self.tag
  }

  pub fn nonce(&self) -> &[u8] {
    &self.nonce
  }
}
