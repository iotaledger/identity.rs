use std::sync::Arc;

use async_trait::async_trait;
use identity_sui_name_tbd::iota_sdk_abstraction::IotaKeySignature;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParams;
use identity_verification::jwu;
use secret_storage::Error as SecretStorageError;
use secret_storage::SignatureScheme;
use secret_storage::Signer;

use crate::JwkStorage;
use crate::KeyId;
use crate::KeyIdStorage;
use crate::KeyStorageErrorKind;
use crate::Storage;

/// Signer that offers signing capabilities for `Signer` trait from `secret_storage`.
/// `Storage` is used to sign.
pub struct StorageSigner<K, I> {
  key_id: KeyId,
  public_key: Jwk,
  storage: Arc<Storage<K, I>>,
}

impl<K, I> Clone for StorageSigner<K, I> {
  fn clone(&self) -> Self {
    StorageSigner {
      key_id: self.key_id.clone(),
      public_key: self.public_key.clone(),
      storage: self.storage.clone(),
    }
  }
}

impl<K, I> StorageSigner<K, I> {
  /// Creates new `StorageSigner` with reference to a `Storage` instance.
  pub fn new(storage: Storage<K, I>, key_id: KeyId, public_key: Jwk) -> Self {
    Self::new_with_shared_storage(Arc::new(storage), key_id, public_key)
  }

  /// Creates new `StorageSigner` with reference to an existing `Arc<Storage>`.
  pub fn new_with_shared_storage(storage: Arc<Storage<K, I>>, key_id: KeyId, public_key: Jwk) -> Self {
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

  /// Returns this [`Signer`]'s public key as [`Jwk`].
  pub fn public_key(&self) -> &Jwk {
    &self.public_key
  }

  /// Returns a reference to this [`Signer`]'s [`Storage`].
  pub fn storage(&self) -> Arc<Storage<K, I>> {
    self.storage.clone()
  }
}

cfg_if::cfg_if! {
  if #[cfg(feature = "send-sync-storage")] {
    /// Trait alias for a `JwkStorage` version that is `Sync`.
    trait JwkStorageGeneric: JwkStorage + Sync {}
    impl<T> JwkStorageGeneric for T where T: JwkStorage + Sync {}
    /// Trait alias for a `KeyIdStorage` version that is `Sync`.
    trait KeyIdStorageGeneric: KeyIdStorage + Sync {}
    impl<T> KeyIdStorageGeneric for T where T: KeyIdStorage + Sync {}
  } else {
    /// Trait alias for a `JwkStorage` version that is not explicitly `Sync`.
    trait JwkStorageGeneric: JwkStorage {}
    impl<T> JwkStorageGeneric for T where T: JwkStorage {}
    /// Trait alias for a `KeyIdStorage` version that is not explicitly `Sync`.
    trait KeyIdStorageGeneric: KeyIdStorage {}
    impl<T> KeyIdStorageGeneric for T where T: KeyIdStorage {}
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K, I> Signer<IotaKeySignature> for StorageSigner<K, I>
where
  K: JwkStorageGeneric,
  I: KeyIdStorageGeneric,
{
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
    self
      .storage
      .key_storage()
      .sign(&self.key_id, data, &self.public_key)
      .await
      .map_err(|e| match e.kind() {
        KeyStorageErrorKind::KeyNotFound => SecretStorageError::KeyNotFound(e.to_string()),
        KeyStorageErrorKind::RetryableIOFailure => SecretStorageError::StoreDisconnected(e.to_string()),
        _ => SecretStorageError::Other(anyhow::anyhow!(e)),
      })
  }
}
