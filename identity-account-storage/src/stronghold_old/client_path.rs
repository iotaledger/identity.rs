// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;

/// A helper type to ensure a consistently generated client path, for DIDs and strings
/// to avoid having `Vec<u8>` everywhere.
#[derive(Debug, Clone)]
pub struct ClientPath(pub String);

impl AsRef<[u8]> for ClientPath {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

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
