// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Signable<'a> {
  Credential(&'a Credential),
  Presentation(&'a Presentation),
  Json(&'a serde_json::Value),
}
