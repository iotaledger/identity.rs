use identity_sui_name_tbd::client::KinesisKeySignature;
use identity_verification::jwk::Jwk;
use secret_storage::key_signature_set::KeySignatureTypes;
use secret_storage::signer::Signer;

use crate::JwkStorage;
use crate::KeyId;
use crate::KeyIdStorage;
use crate::Storage;

pub struct StorageSigner<'a, K, I> {
  key_id: KeyId,
  public_key: Jwk,
  storage: &'a Storage<K, I>,
}

impl<'a, K, I> StorageSigner<'a, K, I> {
  pub fn new(storage: &'a Storage<K, I>, key_id: KeyId, public_key: Jwk) -> Self {
    Self {
      key_id,
      public_key,
      storage,
    }
  }
}

impl<'a, K: JwkStorage, I: KeyIdStorage> Signer<KinesisKeySignature> for StorageSigner<'a, K, I> {
  async fn sign(
    &self,
    hash: impl AsRef<[u8]>,
  ) -> Result<<KinesisKeySignature as KeySignatureTypes>::Signature, anyhow::Error> {
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
