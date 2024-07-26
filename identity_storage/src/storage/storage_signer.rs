use async_trait::async_trait;
use identity_sui_name_tbd::client::IotaKeySignature;
use identity_verification::jwk::Jwk;
use secret_storage::key_signature_set::KeySignatureTypes;
use secret_storage::signer::Signer;

use crate::JwkStorage;
use crate::KeyId;
use crate::KeyIdStorage;
use crate::Storage;

/// Signer that offers signing capabilities for `Signer` trait from `secret_storage`.
/// `Storage` is used to sign.
pub struct StorageSigner<'a, K, I> {
  key_id: KeyId,
  public_key: Jwk,
  storage: &'a Storage<K, I>,
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
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<'a, K: JwkStorage + Sync, I: KeyIdStorage + Sync> Signer<IotaKeySignature> for StorageSigner<'a, K, I> {
  async fn sign(
    &self,
    hash: impl AsRef<[u8]> + Send,
  ) -> Result<<IotaKeySignature as KeySignatureTypes>::Signature, anyhow::Error> {
    Ok(
      self
        .storage
        .key_storage()
        .sign(&self.key_id, hash.as_ref(), &self.public_key)
        .await
        .unwrap(),
    )
  }
}
