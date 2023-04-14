// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_storage::KeyStorageError;
use crate::key_storage::KeyStorageErrorKind;
use crate::key_storage::KeyType;
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkType;
use std::fmt::Display;
const ED25519_KEY_TYPE_STR: &str = "Ed25519";
pub const ED25519_KEY_TYPE: KeyType = KeyType::from_static_str(ED25519_KEY_TYPE_STR);

#[derive(Debug, Copy, Clone)]
pub enum StrongholdKeyType {
  Ed25519,
}

impl StrongholdKeyType {
  pub const fn name(&self) -> &'static str {
    match self {
      StrongholdKeyType::Ed25519 => "Ed25519",
    }
  }
}

impl Display for StrongholdKeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

impl TryFrom<&KeyType> for StrongholdKeyType {
  type Error = KeyStorageError;

  fn try_from(value: &KeyType) -> Result<Self, Self::Error> {
    match value.as_str() {
      ED25519_KEY_TYPE_STR => Ok(StrongholdKeyType::Ed25519),
      _ => Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    }
  }
}

impl TryFrom<&Jwk> for StrongholdKeyType {
  type Error = KeyStorageError;

  fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
    match jwk.kty() {
      JwkType::Okp => {
        let okp_params = jwk.try_okp_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("expected Okp parameters for a JWK with `kty` Okp")
            .with_source(err)
        })?;
        match okp_params.try_ed_curve().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("only Ed curves are supported for signing")
            .with_source(err)
        })? {
          EdCurve::Ed25519 => Ok(StrongholdKeyType::Ed25519),
          curve => Err(
            KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
              .with_custom_message(format!("{curve} not supported")),
          ),
        }
      }
      other => Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
          .with_custom_message(format!("Jwk `kty` {other} not supported")),
      ),
    }
  }
}
