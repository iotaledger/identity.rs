// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;

#[derive(Debug, Clone)]
pub struct ClientPath(pub String);

impl From<&IotaDID> for ClientPath {
  fn from(did: &IotaDID) -> Self {
    Self(did.to_string())
  }
}

impl From<&str> for ClientPath {
  fn from(string: &str) -> Self {
    Self(string.to_owned())
  }
}
