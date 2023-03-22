// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

//TODO: Add documentation to this module.

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct JwkStorageDocumentSignatureOptions {
  pub attach_jwk: bool,
  pub b64: Option<bool>,
  pub typ: Option<String>,
  pub cty: Option<String>,
  pub crit: Option<Vec<String>>,
  pub url: Option<Url>,
  pub nonce: Option<String>,
}

impl JwkStorageDocumentSignatureOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn attach_jwk_to_header(mut self, value: bool) -> Self {
    self.attach_jwk = value;
    self
  }

  pub fn b64(mut self, value: bool) -> Self {
    self.b64 = Some(value);
    self
  }

  pub fn typ(mut self, value: String) -> Self {
    self.typ = Some(value);
    self
  }

  pub fn cty(mut self, value: String) -> Self {
    self.cty = Some(value);
    self
  }

  pub fn add_crit(mut self, value: String) -> Self {
    let mut crits = self.crit.unwrap_or_default();
    crits.push(value);
    self.crit = Some(crits);
    self
  }

  pub fn url(mut self, value: Url) -> Self {
    self.url = Some(value);
    self
  }

  pub fn nonce(mut self, value: String) -> Self {
    self.nonce = Some(value);
    self
  }
}
