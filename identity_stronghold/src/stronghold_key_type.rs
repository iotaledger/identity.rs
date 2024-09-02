// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use identity_storage::KeyStorageError;
use identity_storage::KeyStorageErrorKind;
use identity_storage::KeyType;
use identity_verification::jwk::BlsCurve;
use identity_verification::jwk::EcCurve;
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkType;

pub const ED25519_KEY_TYPE_STR: &str = "Ed25519";
/// The Ed25519 key type.
pub const ED25519_KEY_TYPE: KeyType = KeyType::from_static_str(ED25519_KEY_TYPE_STR);
pub const BLS12381G2_KEY_TYPE_STR: &str = "BLS12381G2";
/// The BLS12381G2 key type
pub const BLS12381G2_KEY_TYPE: KeyType = KeyType::from_static_str(BLS12381G2_KEY_TYPE_STR);

/// Key Types supported by the stronghold storage implementation.
#[derive(Debug, Copy, Clone)]
pub enum StrongholdKeyType {
  Ed25519,
  Bls12381G2,
  Es256,
  Es256k,
}

impl StrongholdKeyType {
  /// String representation of the key type.
  const fn name(&self) -> &'static str {
    match self {
      Self::Ed25519 => ED25519_KEY_TYPE_STR,
      Self::Bls12381G2 => BLS12381G2_KEY_TYPE_STR,
      Self::Es256 => "ES256",
      Self::Es256k => "ES256K",
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
      BLS12381G2_KEY_TYPE_STR => Ok(StrongholdKeyType::Bls12381G2),
      "ES256" => Ok(Self::Es256),
      "ES256K" => Ok(Self::Es256k),
      _ => Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    }
  }
}

impl From<StrongholdKeyType> for KeyType {
  fn from(key_type: StrongholdKeyType) -> KeyType {
    KeyType::from_static_str(key_type.name())
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
      JwkType::Ec => {
        let ec_params = jwk.try_ec_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("expected EC parameters for a JWK with `kty` Ec")
            .with_source(err)
        })?;
        if let Ok(bls_curve) = ec_params.try_bls_curve() {
          match bls_curve {
            BlsCurve::BLS12381G2 => Ok(StrongholdKeyType::Bls12381G2),
            curve => Err(
              KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
                .with_custom_message(format!("{curve} not supported")),
            ),
          }
        } else if let Ok(ec_curve) = ec_params.try_ec_curve() {
          match ec_curve {
            EcCurve::P256 => Ok(StrongholdKeyType::Es256),
            EcCurve::Secp256K1 => Ok(StrongholdKeyType::Es256k),
            curve => Err(
              KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
                .with_custom_message(format!("unsupported EC curve \"{curve}\"")),
            ),
          }
        } else {
          Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_custom_message("invalid EC params"))
        }
      }
      other => Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
          .with_custom_message(format!("Jwk `kty` {other} not supported")),
      ),
    }
  }
}
