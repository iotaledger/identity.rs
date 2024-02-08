use identity_document::document::CoreDocument;
use identity_verification::{jws::JwsAlgorithmPQ, MethodScope};
use async_trait::async_trait;
use crate::{JwkStoragePQ, KeyIdStorage, KeyType, Storage, StorageResult};
use crate::JwkGenOutput;
use crate::key_id_storage::MethodDigest;
use super::JwkStorageDocumentError as Error;
use identity_did::DIDUrl;
use identity_verification::VerificationMethod;
use crate::try_undo_key_generation;

///New trait to handle JWP-based operations on DID Documents
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwsDocumentExtPQC {

  /// Generate new key material in the given `storage` and insert a new verification method with the corresponding
  /// public key material into the DID document. This support BBS+ keys.
  async fn generate_method_pqc<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: JwsAlgorithmPQ,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStoragePQ,
    I: KeyIdStorage;

}




// ====================================================================================================================
// CoreDocument
// ====================================================================================================================


generate_method_for_document_type!(CoreDocument, JwsAlgorithmPQ, JwkStoragePQ, JwkStoragePQ::generate_pq_key, generate_method_core_document);


#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwsDocumentExtPQC for CoreDocument {
  async fn generate_method_pqc<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: JwsAlgorithmPQ,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStoragePQ,
    I: KeyIdStorage,
  {
    // todo!()
    generate_method_core_document(self, storage, key_type, alg, fragment, scope).await
  }

}




// ====================================================================================================================
// IotaDocument
// ====================================================================================================================
#[cfg(feature = "iota-document")]
mod iota_document {
  use crate::JwkStorage;

use super::*;
  use identity_iota_core::IotaDocument;

  generate_method_for_document_type!(IotaDocument, JwsAlgorithmPQ, JwkStoragePQ, JwkStoragePQ::generate_pq_key, generate_method_iota_document);

  #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
  #[cfg_attr(feature = "send-sync-storage", async_trait)]
  impl JwsDocumentExtPQC for IotaDocument {
    async fn generate_method_pqc<K, I>(
      &mut self,
      storage: &Storage<K, I>,
      key_type: KeyType,
      alg: JwsAlgorithmPQ,
      fragment: Option<&str>,
      scope: MethodScope,
    ) -> StorageResult<String>
    where
      K: JwkStoragePQ,
      I: KeyIdStorage,
    {
        // todo!()
      generate_method_iota_document(self, storage, key_type, alg, fragment, scope).await
    }

  }
}