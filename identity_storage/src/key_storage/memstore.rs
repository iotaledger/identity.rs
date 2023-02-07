// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use std::collections::HashMap;

use async_trait::async_trait;
use crypto::signatures::ed25519::SecretKey;
use identity_jose::jwk::EdCurve;
use identity_jose::jwk::Jwk;
use identity_jose::jws::JwsAlgorithm;
use rand::distributions::DistString;
use shared::Shared;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use self::signing_algorithm::MemStoreSigningAlgorithm;
use self::key_type::MemStoreStorageKeyType;
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

    let key_type: MemStoreStorageKeyType = MemStoreStorageKeyType::try_from(&key_type)
      .map_err(|_err: ()| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType))?;

    let (private_key, public_key) = match key_type {
      MemStoreStorageKeyType::Ed25519 => {
        let private_key = SecretKey::generate().expect("TODO");
        let public_key = private_key.public_key();
        (private_key, public_key)
      }
    };

    let random_string: String = rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let kid: KeyId = KeyId::new(random_string);

    let jwk: Jwk = ed25519::encode_jwk(&private_key, &public_key);
    let public_jwk: Jwk = jwk.to_public();

    store.insert(kid.clone(), jwk);

    Ok(JwkGenOutput::new(kid, public_jwk))
  }

  async fn sign(&self, key_id: &KeyId, algorithm: SignatureAlgorithm, data: Vec<u8>) -> KeyStorageResult<Vec<u8>> {
    let jwk_store: RwLockReadGuard<'_, JwkKeyStore> = self.jwk_store.read().await;

    let jwk: &Jwk = jwk_store
      .get(key_id)
      .ok_or_else(|| KeyStorageError::new(KeyStorageErrorKind::KeyNotFound))?;

    match MemStoreSigningAlgorithm::try_from(&algorithm)? {
      MemStoreSigningAlgorithm::Ed25519 => (),
    }

    jwk
      .check_alg(JwsAlgorithm::EdDSA.name())
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedSigningAlgorithm).with_source(err))?;

    let okp_params = jwk.try_okp_params().map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("expected Okp params")
        .with_source(err)
    })?;
    if okp_params.crv != EdCurve::Ed25519.name() {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message(format!("expected Jwk with Okp {} crv", EdCurve::Ed25519)),
      );
    }

    let secret_key: _ = ed25519::expand_secret_jwk(&jwk)?;

    Ok(secret_key.sign(&data).to_bytes().to_vec())
  }

  async fn public_jwk(&self, key_id: &KeyId) -> KeyStorageResult<Jwk> {
    let jwk_store: RwLockReadGuard<'_, JwkKeyStore> = self.jwk_store.read().await;
    let jwk: &Jwk = jwk_store
      .get(key_id)
      .ok_or_else(|| KeyStorageError::new(KeyStorageErrorKind::KeyNotFound))?;
    Ok(jwk.to_public())
  }

  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()> {
    let mut store: RwLockWriteGuard<'_, JwkKeyStore> = self.jwk_store.write().await;

    store.remove(key_id);

    Ok(())
  }
}

pub(crate) mod ed25519 {
  use crypto::signatures::ed25519::{self, PublicKey, SecretKey};
  use identity_jose::{
    jwk::{EdCurve, Jwk, JwkParamsOkp},
    jwu,
  };

  use crate::key_storage::{KeyStorageError, KeyStorageErrorKind, KeyStorageResult};

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

pub mod signing_algorithm {
  use crate::key_storage::{KeyStorageError, SignatureAlgorithm};

  const ED25519_SIGNING_ALG_STR: &str = "Ed25519";
  pub const ED25519_SIGNING_ALG: SignatureAlgorithm = SignatureAlgorithm::from_static_str(ED25519_SIGNING_ALG_STR);

  pub enum MemStoreSigningAlgorithm {
    Ed25519,
  }

  impl TryFrom<&SignatureAlgorithm> for MemStoreSigningAlgorithm {
    type Error = KeyStorageError;

    fn try_from(value: &SignatureAlgorithm) -> Result<Self, Self::Error> {
      match value.as_str() {
        ED25519_SIGNING_ALG_STR => Ok(MemStoreSigningAlgorithm::Ed25519),
        other => Err(
          KeyStorageError::new(crate::key_storage::KeyStorageErrorKind::UnsupportedSigningAlgorithm)
            .with_custom_message(format!("`{other}` not supported")),
        ),
      }
    }
  }
}

pub mod key_type {
  use crate::key_storage::KeyType;

  const ED25519_KEY_TYPE_STR: &str = "Ed25519";
  pub const ED25519_KEY_TYPE: KeyType = KeyType::from_static_str(ED25519_KEY_TYPE_STR);

  pub enum MemStoreStorageKeyType {
    Ed25519,
  }

  impl TryFrom<&KeyType> for MemStoreStorageKeyType {
    // TODO: Use a better type for the error.
    type Error = ();

    fn try_from(value: &KeyType) -> Result<Self, Self::Error> {
      match value.as_str() {
        ED25519_KEY_TYPE_STR => Ok(MemStoreStorageKeyType::Ed25519),
        _ => Err(()),
      }
    }
  }

  // impl TryFrom<&Jwk> for MemStoreStorageKeyType {
  //   // TODO: Use a better type for the error.
  //   type Error = KeyStorageError;

  //   fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
  //     match jwk.kty() {
  //       JwkType::Ec => todo!(),
  //       JwkType::Rsa => todo!(),
  //       JwkType::Oct => todo!(),
  //       JwkType::Okp => {
  //         let okp_params = jwk
  //           .try_okp_params()
  //           .expect("unlikely, but return error anyway because set_params_unchecked exists");
  //         match okp_params
  //           .try_ed_curve()
  //           .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_source(err))?
  //         {
  //           EdCurve::Ed25519 => Ok(MemStoreStorageKeyType::Ed25519),
  //           curve => Err(
  //             KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
  //               .with_custom_message(format!("{curve} not supported")),
  //           ),
  //         }
  //       }
  //     }
  //   }
  // }
}

impl Default for MemKeyStore {
  fn default() -> Self {
    Self::new()
  }
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
  use crypto::signatures::ed25519::{self, PublicKey, Signature};
  use identity_jose::{jwk::JwkParamsOkp, jwu};

  use super::*;

  #[tokio::test]
  async fn generate_sign() {
    let test_msg = b"test";
    let store = MemKeyStore::new();

    let JwkGenOutput { key_id, jwk } = store.generate_jwk(key_type::ED25519_KEY_TYPE).await.unwrap();

    let signature = store
      .sign(&key_id, signing_algorithm::ED25519_SIGNING_ALG, test_msg.to_vec())
      .await
      .unwrap();

    let pubkey = expand_public_jwk(&jwk);

    let signature = Signature::from_bytes(signature.try_into().unwrap());

    assert!(pubkey.verify(&signature, test_msg));
  }

  pub(crate) fn expand_public_jwk(jwk: &Jwk) -> PublicKey {
    let params: &JwkParamsOkp = jwk.try_okp_params().unwrap();

    if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
      panic!("expected an ed25519 jwk");
    }

    let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

    PublicKey::try_from(pk).unwrap()
  }
}
