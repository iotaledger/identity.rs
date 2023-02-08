// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use std::collections::HashMap;
use std::fmt::Display;

use async_trait::async_trait;
use crypto::signatures::ed25519::SecretKey;
use identity_jose::jwk::EdCurve;
use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkType;
use identity_jose::jws::JwsAlgorithm;
use rand::distributions::DistString;
use shared::Shared;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::key_gen::JwkGenOutput;
use super::KeyId;
use super::KeyStorageError;
use super::KeyStorageErrorKind;
use super::KeyStorageResult;
use super::KeyType;
use super::SignatureAlgorithm;
use crate::key_storage::KeyStorage;

/// The map from key ids to JWKs.
type JwkKeyStore = HashMap<KeyId, Jwk>;

/// An insecure, in-memory [`Storage`] implementation that serves as an example and is used in tests.
#[derive(Debug)]
pub struct MemKeyStore {
  jwk_store: Shared<JwkKeyStore>,
}

impl MemKeyStore {
  /// Creates a new, empty `MemStore` instance.
  pub fn new() -> Self {
    Self {
      jwk_store: Shared::new(HashMap::new()),
    }
  }
}

// Refer to the `Storage` interface docs for high-level documentation of the individual methods.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl KeyStorage for MemKeyStore {
  async fn generate_jwk(&self, key_type: KeyType) -> KeyStorageResult<JwkGenOutput> {
    let mut store: RwLockWriteGuard<'_, JwkKeyStore> = self.jwk_store.write().await;

    let key_type: MemStoreStorageKeyType = MemStoreStorageKeyType::try_from(&key_type)?;

    let (private_key, public_key) = match key_type {
      MemStoreStorageKeyType::Ed25519 => {
        let private_key = SecretKey::generate()
          .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::RetryableIOFailure).with_source(err))?;
        let public_key = private_key.public_key();
        (private_key, public_key)
      }
    };

    let kid: KeyId = random_key_id();

    let jwk: Jwk = ed25519::encode_jwk(&private_key, &public_key);
    let public_jwk: Jwk = jwk.to_public();

    store.insert(kid.clone(), jwk);

    Ok(JwkGenOutput::new(kid, public_jwk))
  }

  async fn insert_jwk(&self, jwk: Jwk) -> KeyStorageResult<KeyId> {
    let _ = MemStoreStorageKeyType::try_from(&jwk)?;

    if !jwk.is_private() {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("expected a Jwk with all private key components set"),
      );
    }

    let key_id: KeyId = random_key_id();

    let mut jwk_store: RwLockWriteGuard<'_, JwkKeyStore> = self.jwk_store.write().await;

    jwk_store.insert(key_id.clone(), jwk);

    Ok(key_id)
  }

  async fn sign(&self, key_id: &KeyId, algorithm: SignatureAlgorithm, data: Vec<u8>) -> KeyStorageResult<Vec<u8>> {
    let jwk_store: RwLockReadGuard<'_, JwkKeyStore> = self.jwk_store.read().await;

    let jwk: &Jwk = jwk_store
      .get(key_id)
      .ok_or_else(|| KeyStorageError::new(KeyStorageErrorKind::KeyNotFound))?;

    let signature: Vec<u8> = match MemStoreSigningAlgorithm::try_from(&algorithm)? {
      sig_alg @ MemStoreSigningAlgorithm::Ed25519 => {
        jwk
          .check_alg(JwsAlgorithm::EdDSA.name())
          .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedSignatureAlgorithm).with_source(err))?;

        let okp_params = jwk.try_okp_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedSigningKey)
            .with_custom_message(format!(
              "expected a Jwk with Okp params in order to sign with {sig_alg}"
            ))
            .with_source(err)
        })?;
        if okp_params.crv != EdCurve::Ed25519.name() {
          return Err(
            KeyStorageError::new(KeyStorageErrorKind::UnsupportedSigningKey).with_custom_message(format!(
              "expected Jwk with Okp {} crv in order to sign with {sig_alg}",
              EdCurve::Ed25519
            )),
          );
        }

        let secret_key: _ = ed25519::expand_secret_jwk(jwk)?;
        secret_key.sign(&data).to_bytes().to_vec()
      }
    };

    Ok(signature)
  }

  async fn public_jwk(&self, key_id: &KeyId) -> KeyStorageResult<Jwk> {
    let jwk_store: RwLockReadGuard<'_, JwkKeyStore> = self.jwk_store.read().await;
    let jwk: &Jwk = jwk_store
      .get(key_id)
      .ok_or_else(|| KeyStorageError::new(KeyStorageErrorKind::KeyNotFound))?;
    Ok(jwk.to_public())
  }

  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<bool> {
    let mut jwk_store: RwLockWriteGuard<'_, JwkKeyStore> = self.jwk_store.write().await;

    let key: Option<Jwk> = jwk_store.remove(key_id);

    Ok(key.is_some())
  }

  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool> {
    let jwk_store: RwLockReadGuard<'_, JwkKeyStore> = self.jwk_store.read().await;
    Ok(jwk_store.contains_key(key_id))
  }
}

pub(crate) mod ed25519 {
  use crypto::signatures::ed25519::PublicKey;
  use crypto::signatures::ed25519::SecretKey;
  use crypto::signatures::ed25519::{self};
  use identity_jose::jwk::EdCurve;
  use identity_jose::jwk::Jwk;
  use identity_jose::jwk::JwkParamsOkp;
  use identity_jose::jwu;

  use crate::key_storage::KeyStorageError;
  use crate::key_storage::KeyStorageErrorKind;
  use crate::key_storage::KeyStorageResult;

  pub(crate) fn expand_secret_jwk(jwk: &Jwk) -> KeyStorageResult<SecretKey> {
    let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

    if params
      .try_ed_curve()
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_source(err))?
      != EdCurve::Ed25519
    {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
          .with_custom_message(format!("expected an {} key", EdCurve::Ed25519.name())),
      );
    }

    let sk: [u8; ed25519::SECRET_KEY_LENGTH] = params
      .d
      .as_deref()
      .map(jwu::decode_b64)
      .ok_or_else(|| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("expected Jwk `d` param to be present")
      })?
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("unable to decode `d` param")
          .with_source(err)
      })?
      .try_into()
      .map_err(|_| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message(format!("expected key of length {}", ed25519::SECRET_KEY_LENGTH))
      })?;

    Ok(SecretKey::from_bytes(sk))
  }

  pub(crate) fn encode_jwk(private_key: &SecretKey, public_key: &PublicKey) -> Jwk {
    let x = jwu::encode_b64(public_key.as_ref());
    let d = jwu::encode_b64(private_key.to_bytes().as_ref());
    let mut params = JwkParamsOkp::new();
    params.x = x;
    params.d = Some(d);
    params.crv = EdCurve::Ed25519.name().to_owned();
    Jwk::from_params(params)
  }
}

const ED25519_SIGNING_ALG_STR: &str = "Ed25519";
pub const ED25519_SIGNING_ALG: SignatureAlgorithm = SignatureAlgorithm::from_static_str(ED25519_SIGNING_ALG_STR);

pub enum MemStoreSigningAlgorithm {
  Ed25519,
}

impl Display for MemStoreSigningAlgorithm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MemStoreSigningAlgorithm::Ed25519 => f.write_str("Ed25519"),
    }
  }
}

impl TryFrom<&SignatureAlgorithm> for MemStoreSigningAlgorithm {
  type Error = KeyStorageError;

  fn try_from(value: &SignatureAlgorithm) -> Result<Self, Self::Error> {
    match value.as_str() {
      ED25519_SIGNING_ALG_STR => Ok(MemStoreSigningAlgorithm::Ed25519),
      other => Err(
        KeyStorageError::new(crate::key_storage::KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
          .with_custom_message(format!("`{other}` not supported")),
      ),
    }
  }
}

const ED25519_KEY_TYPE_STR: &str = "Ed25519";
pub const ED25519_KEY_TYPE: KeyType = KeyType::from_static_str(ED25519_KEY_TYPE_STR);

pub enum MemStoreStorageKeyType {
  Ed25519,
}

impl TryFrom<&KeyType> for MemStoreStorageKeyType {
  type Error = KeyStorageError;

  fn try_from(value: &KeyType) -> Result<Self, Self::Error> {
    match value.as_str() {
      ED25519_KEY_TYPE_STR => Ok(MemStoreStorageKeyType::Ed25519),
      _ => Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    }
  }
}

impl TryFrom<&Jwk> for MemStoreStorageKeyType {
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
          EdCurve::Ed25519 => Ok(MemStoreStorageKeyType::Ed25519),
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

impl Default for MemKeyStore {
  fn default() -> Self {
    Self::new()
  }
}

/// Generate a random alphanumeric string of len 32.
fn random_key_id() -> KeyId {
  KeyId::new(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32))
}

pub(crate) mod shared {
  use core::fmt::Debug;
  use core::fmt::Formatter;
  use tokio::sync::RwLock;
  use tokio::sync::RwLockReadGuard;
  use tokio::sync::RwLockWriteGuard;

  #[derive(Default)]
  pub struct Shared<T>(RwLock<T>);

  impl<T> Shared<T> {
    pub fn new(data: T) -> Self {
      Self(RwLock::new(data))
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
      self.0.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
      self.0.write().await
    }
  }

  impl<T: Debug> Debug for Shared<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
      Debug::fmt(&self.0, f)
    }
  }
}

#[cfg(test)]
mod tests {
  use crypto::signatures::ed25519::PublicKey;
  use crypto::signatures::ed25519::Signature;
  use crypto::signatures::ed25519::{self};
  use identity_jose::jwk::EcCurve;
  use identity_jose::jwk::JwkParamsEc;
  use identity_jose::jwk::JwkParamsOkp;
  use identity_jose::jwu;

  use super::*;

  #[tokio::test]
  async fn generate_and_sign() {
    let test_msg: &[u8] = b"test";
    let store: MemKeyStore = MemKeyStore::new();

    let JwkGenOutput { key_id, jwk } = store.generate_jwk(ED25519_KEY_TYPE).await.unwrap();

    let signature = store
      .sign(&key_id, ED25519_SIGNING_ALG, test_msg.to_vec())
      .await
      .unwrap();

    let public_key: PublicKey = expand_public_jwk(&jwk);
    let signature: Signature = Signature::from_bytes(signature.try_into().unwrap());

    assert!(public_key.verify(&signature, test_msg));
    assert!(store.exists(&key_id).await.unwrap());
    assert!(store.delete(&key_id).await.unwrap());
  }

  #[tokio::test]
  async fn insert() {
    let store: MemKeyStore = MemKeyStore::new();

    let (private_key, public_key) = generate_ed25519();
    let jwk: Jwk = crate::key_storage::ed25519::encode_jwk(&private_key, &public_key);

    // VALID: Inserting a Jwk with all private key components set should succeed.
    store.insert_jwk(jwk.clone()).await.unwrap();

    // INVALID: Inserting a Jwk with all private key components unset should fail.
    let err = store.insert_jwk(jwk.to_public()).await.unwrap_err();
    assert!(matches!(err.kind(), KeyStorageErrorKind::Unspecified))
  }

  #[tokio::test]
  async fn exists() {
    let store: MemKeyStore = MemKeyStore::new();
    assert!(!store.exists(&KeyId::new("non-existent-id")).await.unwrap());
  }

  #[tokio::test]
  async fn incompatible_signing_key() {
    let store: MemKeyStore = MemKeyStore::new();

    let mut ec_params = JwkParamsEc::new();
    ec_params.crv = EcCurve::P256.name().to_owned();
    ec_params.x = "".to_owned();
    ec_params.y = "".to_owned();
    ec_params.d = Some("".to_owned());
    let jwk_ec = Jwk::from_params(ec_params);

    let key_id = store.insert_jwk(jwk_ec).await.unwrap();

    let err: _ = store
      .sign(&key_id, ED25519_SIGNING_ALG, b"test".to_vec())
      .await
      .unwrap_err();
    assert!(matches!(err.kind(), KeyStorageErrorKind::UnsupportedSigningKey));
  }

  pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
    let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

    if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
      panic!("expected an ed25519 jwk");
    }

    let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

    PublicKey::try_from(pk).unwrap()
  }

  fn generate_ed25519() -> (SecretKey, PublicKey) {
    let private_key = SecretKey::generate().unwrap();
    let public_key = private_key.public_key();
    (private_key, public_key)
  }
}
