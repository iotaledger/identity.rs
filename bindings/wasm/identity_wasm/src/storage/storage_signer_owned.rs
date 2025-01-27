// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_iota::iota_interaction::IotaKeySignature;
use identity_iota::verification::jwk::Jwk;
use identity_iota::verification::jwk::JwkParams;
use identity_iota::verification::jwu;
use secret_storage::Error as SecretStorageError;
use secret_storage::SignatureScheme;
use secret_storage::Signer;
use identity_iota::storage::JwkStorage;
use identity_iota::storage::KeyId;
use identity_iota::storage::KeyIdStorage;
use identity_iota::storage::KeyStorageErrorKind;
use identity_iota::storage::Storage;

use crate::jose::WasmJwk;

use super::WasmStorage;

/// Signer that offers signing capabilities for `Signer` trait from `secret_storage`.
/// `Storage` is used to sign.
pub struct StorageSignerOwned {
  key_id: KeyId,
  public_key: Jwk,
  // storage: WasmStorage,
}

// impl<K, I> Clone for StorageSignerOwned<K, I> {
//   fn clone(&self) -> Self {
//     StorageSignerOwned {
//       key_id: self.key_id.clone(),
//       public_key: self.public_key.clone(),
//       storage: self.storage,
//     }
//   }
// }

impl StorageSignerOwned {
  /// Creates new `StorageSignerOwned` with reference to a `Storage` instance.
  pub fn new(storage: WasmStorage, key_id: KeyId, public_key: Jwk) -> Self {
    Self {
      key_id,
      public_key,
      // storage,
    }
  }

  /// Returns a reference to the [`KeyId`] of the key used by this [`Signer`].
  pub fn key_id(&self) -> &KeyId {
    &self.key_id
  }

  /// Returns this [`Signer`]'s public key as [`Jwk`].
  pub fn public_key(&self) -> &Jwk {
    &self.public_key
  }

  // /// Returns a reference to this [`Signer`]'s [`Storage`].
  // pub fn storage(&self) -> &WasmStorage {
  //   &self.storage
  // }
}

#[async_trait(?Send)]
impl Signer<IotaKeySignature> for StorageSignerOwned {
  type KeyId = KeyId;
  fn key_id(&self) -> &KeyId {
    &self.key_id
  }
  async fn public_key(&self) -> Result<<IotaKeySignature as SignatureScheme>::PublicKey, SecretStorageError> {
    match self.public_key.params() {
      JwkParams::Okp(params) => jwu::decode_b64(&params.x)
        .map_err(|e| SecretStorageError::Other(anyhow::anyhow!("could not base64 decode key {}; {e}", self.key_id()))),
      _ => todo!("add support for other key types"),
    }
  }
  async fn sign(&self, data: &[u8]) -> Result<<IotaKeySignature as SignatureScheme>::Signature, SecretStorageError> {
    // let key_storage = self
    //   .storage
    //   .key_storage();
    // // call trait's sign impl for `WasmJwkStorage` instead of `WasmJwkStorage`s own impl
    // JwkStorage::sign(&key_storage, &self.key_id, data, &self.public_key)
    //   .await
    //   .map_err(|e| match e.kind() {
    //     KeyStorageErrorKind::KeyNotFound => SecretStorageError::KeyNotFound(e.to_string()),
    //     KeyStorageErrorKind::RetryableIOFailure => SecretStorageError::StoreDisconnected(e.to_string()),
    //     _ => SecretStorageError::Other(anyhow::anyhow!(e)),
    //   })
    todo!("pointer test");
  }
}
