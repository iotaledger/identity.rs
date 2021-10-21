// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_iota::tangle::NetworkName;
use std::convert::TryInto;

use crate::account::Account;
use crate::error::Result;
use crate::identity::IdentityCreate;
use crate::types::MethodSecret;

use super::AccountBuilder;

pub struct IdentityBuilder<'account_builder> {
  builder: &'account_builder mut AccountBuilder,
  key_type: KeyType,
  network: Option<NetworkName>,
  method_secret: Option<MethodSecret>,
}

impl<'account_builder> IdentityBuilder<'account_builder> {
  /// Creates a new [`IdentityBuilder`] to customize the creation of an identity.
  ///
  /// Typically created by [`AccountBuilder::create_identity`].
  pub fn new(builder: &'account_builder mut AccountBuilder) -> Self {
    Self {
      builder,
      key_type: KeyType::Ed25519,
      network: None,
      method_secret: None,
    }
  }

  /// Sets the key type of the initial authentication method.
  #[must_use]
  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  /// Sets the IOTA Tangle network of the Identity DID.
  #[allow(clippy::double_must_use)]
  #[must_use]
  pub fn network<T>(mut self, value: T) -> Result<Self>
  where
    T: TryInto<NetworkName>,
  {
    self.network = Some(value.try_into().map_err(|_| identity_iota::Error::InvalidNetworkName)?);
    Ok(self)
  }

  /// Sets the [`MethodSecret`] for the Identity creation.
  ///
  /// Note that only [`MethodSecret::Ed25519`] is currently supported.
  #[must_use]
  pub fn method_secret(mut self, value: MethodSecret) -> Self {
    self.method_secret = Some(value);
    self
  }

  pub async fn build(self) -> Result<Account> {
    let builder = self.builder;

    let id_create = IdentityCreate {
      key_type: self.key_type,
      network: self.network,
      method_secret: self.method_secret,
    };

    builder.build_identity(id_create).await
  }
}
