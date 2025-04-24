// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::jwk::Jwk;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsEc;
use crate::jwu;
use anyhow::anyhow;
use anyhow::Context as _;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::secp256k1::Secp256k1KeyPair;
use fastcrypto::secp256r1::Secp256r1KeyPair;
use fastcrypto::traits::KeyPair;
use identity_iota_interaction::types::crypto::IotaKeyPair;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::SignatureScheme as IotaSignatureScheme;

use super::ed25519;
use super::secp256k1;
use super::secp256r1;

/// Helper trait to convert implementing conversion to `Jwk`.
pub trait ToJwk {
  /// Error type used
  type Error;

  /// Converts instance to `Jwk`.
  fn to_jwk(&self) -> Result<Jwk, Self::Error>;
}

impl ToJwk for PublicKey {
  type Error = Error;

  fn to_jwk(&self) -> Result<Jwk, Self::Error> {
    let jwk = match self {
      PublicKey::Ed25519(pk) => ed25519::pk_to_jwk(pk),
      PublicKey::Secp256r1(pk) => secp256r1::pk_to_jwk(pk),
      PublicKey::Secp256k1(pk) => secp256k1::pk_to_jwk(pk),
      _ => return Err(Error::KeyConversion("unsupported key type".to_string())),
    };

    Ok(jwk)
  }
}

impl ToJwk for Ed25519KeyPair {
  type Error = Error;

  fn to_jwk(&self) -> Result<Jwk, Self::Error> {
    Ok(ed25519::encode_jwk(self.copy()))
  }
}

/// Helper trait to convert implementing conversion from `Jwk`.
pub trait FromJwk {
  /// Error type used
  type Error;

  /// Create instance from `Jwk`.
  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error>
  where
    Self: Sized;
}

impl FromJwk for IotaKeyPair {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    let maybe_ed22519 = Ed25519KeyPair::from_jwk(jwk).map(IotaKeyPair::from);
    let maybe_secp256r1 = Secp256r1KeyPair::from_jwk(jwk).map(IotaKeyPair::from);
    let maybe_secp256k1 = Secp256k1KeyPair::from_jwk(jwk).map(IotaKeyPair::from);

    maybe_ed22519
      .or(maybe_secp256k1)
      .or(maybe_secp256r1)
      .map_err(|err| Error::KeyConversion(err.to_string()))
  }
}

impl FromJwk for PublicKey {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    match jwk.params() {
      JwkParams::Okp(params) => {
        if params.crv != "Ed25519" {
          return Err(Error::KeyConversion(format!("unsupported key type {}", params.crv)));
        }

        jwu::decode_b64(&params.x)
          .context("failed to base64 decode key")
          .and_then(|pk_bytes| {
            PublicKey::try_from_bytes(IotaSignatureScheme::ED25519, &pk_bytes).map_err(|e| anyhow!("{e}"))
          })
          .map_err(|err| Error::KeyConversion(err.to_string()))
      }
      JwkParams::Ec(JwkParamsEc { crv, x, y, .. }) => {
        let pk_bytes = {
          let mut decoded_x_bytes =
            jwu::decode_b64(x).map_err(|e| Error::KeyConversion(format!("failed to decode b64 x parameter: {e}")))?;
          let mut decoded_y_bytes =
            jwu::decode_b64(y).map_err(|e| Error::KeyConversion(format!("failed to decode b64 y parameter: {e}")))?;
          decoded_x_bytes.append(&mut decoded_y_bytes);

          decoded_x_bytes
        };

        if jwk.alg() == Some("ES256") || crv == "P-256" {
          PublicKey::try_from_bytes(IotaSignatureScheme::Secp256r1, &pk_bytes)
            .map_err(|e| Error::KeyConversion(format!("not a secp256r1 key: {e}")))
        } else if jwk.alg() == Some("ES256K") || crv == "K-256" {
          PublicKey::try_from_bytes(IotaSignatureScheme::Secp256k1, &pk_bytes)
            .map_err(|e| Error::KeyConversion(format!("not a secp256k1 key: {e}")))
        } else {
          Err(Error::KeyError("invalid EC key"))
        }
      }
      _ => Err(Error::KeyConversion("unsupported key".to_string())),
    }
  }
}

impl FromJwk for Ed25519KeyPair {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    ed25519::jwk_to_keypair(jwk).map_err(|err| Error::KeyConversion(err.to_string()))
  }
}

impl FromJwk for Secp256r1KeyPair {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    secp256r1::jwk_to_keypair(jwk).map_err(|err| Error::KeyConversion(err.to_string()))
  }
}

impl FromJwk for Secp256k1KeyPair {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    secp256k1::jwk_to_keypair(jwk).map_err(|err| Error::KeyConversion(err.to_string()))
  }
}

impl FromJwk for Ed25519PublicKey {
  type Error = Error;

  fn from_jwk(jwk: &Jwk) -> Result<Self, Self::Error> {
    ed25519::from_public_jwk(jwk).map_err(|err| Error::KeyConversion(err.to_string()))
  }
}
