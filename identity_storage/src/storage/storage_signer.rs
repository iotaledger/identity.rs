use async_trait::async_trait;
use identity_sui_name_tbd::client::IotaKeySignature;
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
pub struct StorageSigner<'a, K, I> {
  key_id: KeyId,
  public_key: Jwk,
  storage: &'a Storage<K, I>,
}

impl<'a, K, I> Clone for StorageSigner<'a, K, I> {
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
  pub fn new(storage: &'a Storage<K, I>, key_id: KeyId, public_key: Jwk) -> Self {
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
  pub fn storage(&self) -> &Storage<K, I> {
    self.storage
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<'a, K, I> Signer<IotaKeySignature> for StorageSigner<'a, K, I>
where
  K: JwkStorage + Sync,
  I: KeyIdStorage + Sync,
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
