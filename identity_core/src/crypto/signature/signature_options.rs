// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::Url;

//TODO: Should this module perhaps be moved to `identity_verification`? 
// Ideally it would be in `identity_storage`, but the test-only JWS signing functionality in `identity_document` 
// requires this type and that crate cannot depend on `identity_storage`. 

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JwsSignatureOptions {
  pub attach_jwk: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub b64: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub typ: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cty: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub crit: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  pub detached_payload: bool,
}

impl JwsSignatureOptions {
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

  pub fn detached_payload(mut self, value: bool) -> Self {
    self.detached_payload = value;
    self
  }
}
