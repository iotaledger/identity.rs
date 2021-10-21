// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
use std::convert::TryInto;

use identity_core::crypto::KeyType;
use identity_iota::tangle::NetworkName;

use crate::types::MethodSecret;

/// Configuration used to create a new Identity.
#[derive(Debug, Clone)]
pub(crate) struct IdentityCreate {
  pub(crate) key_type: KeyType,
  pub(crate) network: Option<NetworkName>,
  pub(crate) method_secret: Option<MethodSecret>,
}

impl IdentityCreate {
  pub(crate) fn new() -> Self {
    Self {
      key_type: KeyType::Ed25519,
      network: None,
      method_secret: None,
    }
  }
}

#[cfg(test)]
impl IdentityCreate {
  #[must_use]
  pub(crate) fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  #[allow(clippy::double_must_use)]
  #[must_use]
  pub(crate) fn network<T>(mut self, value: T) -> crate::Result<Self>
  where
    T: TryInto<NetworkName>,
  {
    self.network = Some(value.try_into().map_err(|_| identity_iota::Error::InvalidNetworkName)?);
    Ok(self)
  }

  #[must_use]
  pub(crate) fn method_secret(mut self, value: MethodSecret) -> Self {
    self.method_secret = Some(value);
    self
  }
}

impl Default for IdentityCreate {
  fn default() -> Self {
    Self::new()
  }
}
