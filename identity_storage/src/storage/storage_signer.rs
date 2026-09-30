// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use identity_verification::jwk::ToJwk as _;
use iota_sdk::types::Address;
use iota_sdk::types::PublicKey;
use iota_sdk::types::Transaction;
use iota_sdk::types::UserSignature;
use secret_storage::iota::TransactionSigner;
use secret_storage::Error as SecretStorageError;

use crate::JwkStorage;
use crate::KeyId;
use crate::KeyIdStorage;
use crate::KeyStorageErrorKind;
use crate::Storage;

/// Signer that offers signing capabilities for `Signer` trait from `secret_storage`.
/// `Storage` is used to sign.
pub struct StorageSigner<'a, K, I> {
  key_id: KeyId,
  public_key: PublicKey,
  storage: &'a Storage<K, I>,
}

impl<K, I> Clone for StorageSigner<'_, K, I> {
  fn clone(&self) -> Self {
    StorageSigner {
      key_id: self.key_id.clone(),
      public_key: self.public_key.clone(),
      storage: self.storage,
    }
  }
}

impl<'a, K, I> StorageSigner<'a, K, I> {
  /// Creates new `StorageSigner` with reference to a `Storage` instance.
  pub fn new(storage: &'a Storage<K, I>, key_id: KeyId, public_key: PublicKey) -> Self {
    Self {
      key_id,
      public_key,
      storage,
    }
  }

  /// Returns a reference to the [`KeyId`] of the key used by this [`Signer`].
  pub fn key_id(&self) -> &KeyId {
    &self.key_id
  }

  /// Returns this [`Signer`]'s public key.
  pub fn public_key(&self) -> &PublicKey {
    &self.public_key
  }

  /// Returns a reference to this [`Signer`]'s [`Storage`].
  pub fn storage(&self) -> &Storage<K, I> {
    self.storage
  }
}

impl<K, I> iota_sdk::transaction_builder::TransactionSigner for StorageSigner<'_, K, I>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  type Error = SecretStorageError;
  async fn sign(&self, transaction: &Transaction) -> Result<UserSignature, Self::Error> {
    let data = transaction.intent_message().signing_digest();
    let public_jwk = self
      .public_key
      .clone()
      .to_jwk()
      .map_err(|e| SecretStorageError::Other(anyhow!("failed to convert public key to JWK: {e}")))?;
    let signature_bytes = self
      .storage
      .key_storage()
      .sign(&self.key_id, &data, &public_jwk)
      .await
      .map_err(|e| match e.kind() {
        KeyStorageErrorKind::KeyNotFound => SecretStorageError::KeyNotFound(e.to_string()),
        KeyStorageErrorKind::RetryableIOFailure => SecretStorageError::StoreDisconnected(e.to_string()),
        _ => SecretStorageError::Other(anyhow::anyhow!(e)),
      })?;

    user_signature_from_bytes(&self.public_key, &signature_bytes)
  }
}

impl<K, I> TransactionSigner for StorageSigner<'_, K, I>
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  fn address(&self) -> Address {
    Address::from(&self.public_key)
  }
}

fn user_signature_from_bytes(
  public_key: &PublicKey,
  signature_bytes: &[u8],
) -> Result<UserSignature, SecretStorageError> {
  let iota_signature_bytes = [
    [public_key.scheme().to_u8()].as_slice(),
    signature_bytes,
    public_key.as_ref(),
  ]
  .concat();
  UserSignature::from_bytes(&iota_signature_bytes)
    .map_err(|e| SecretStorageError::Other(anyhow!("failed to create valid IOTA signature: {e}")))
}
