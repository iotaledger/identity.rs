//TODO:: JwpDocumentExt

use identity_document::document::CoreDocument;
use crate::Storage;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use crate::KeyType;
use identity_verification::MethodScope;
use crate::StorageResult;
use crate::JwkStorageExt;
use crate::KeyIdStorage;
use crate::JwkGenOutput;

use crate::key_id_storage::KeyIdStorageResult;
use crate::key_id_storage::MethodDigest;
use crate::key_storage::JwkStorage;
use crate::key_storage::KeyId;
use crate::key_storage::KeyStorageResult;
use super::JwkStorageDocumentError as Error;
use super::JwsSignatureOptions;

use async_trait::async_trait;
use identity_core::common::Object;
use identity_credential::credential::Credential;
use identity_credential::credential::Jws;
use identity_credential::credential::Jwt;
use identity_credential::presentation::JwtPresentationOptions;
use identity_credential::presentation::Presentation;
use identity_did::DIDUrl;
use identity_verification::jose::jws::CompactJwsEncoder;
use identity_verification::jose::jws::CompactJwsEncodingOptions;
use identity_verification::jose::jws::JwsAlgorithm;
use identity_verification::jose::jws::JwsHeader;
use identity_verification::jws::CharSet;
use identity_verification::MethodData;
use identity_verification::VerificationMethod;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::try_undo_key_generation;

macro_rules! generate_method_for_document_type {
    ($t:ty, $name:ident) => {
      async fn $name<K, I>(
        document: &mut $t,
        storage: &Storage<K, I>,
        key_type: KeyType,
        alg: ProofAlgorithm,
        fragment: Option<&str>,
        scope: MethodScope,
      ) -> StorageResult<String>
      where
        K: JwkStorageExt,
        I: KeyIdStorage,
      {
        let JwkGenOutput { key_id, jwk } = <K as JwkStorageExt>::generate_bbs_key(&storage.key_storage(), key_type, alg)
          .await
          .map_err(Error::KeyStorageError)?;
  
        // Produce a new verification method containing the generated JWK. If this operation fails we handle the error
        // by attempting to revert key generation before returning an error.
        let method: VerificationMethod = {
          match VerificationMethod::new_from_jwk(document.id().clone(), jwk, fragment)
            .map_err(Error::VerificationMethodConstructionError)
          {
            Ok(method) => method,
            Err(source) => {
              return Err(try_undo_key_generation(storage, &key_id, source).await);
            }
          }
        };
  
        // Extract data from method before inserting it into the DID document.
        let method_digest: MethodDigest = MethodDigest::new(&method).map_err(Error::MethodDigestConstructionError)?;
        let method_id: DIDUrl = method.id().clone();
  
        // The fragment is always set on a method, so this error will never occur.
        let fragment: String = method_id
          .fragment()
          .ok_or(identity_verification::Error::MissingIdFragment)
          .map_err(Error::VerificationMethodConstructionError)?
          .to_owned();
  
        // Insert method into document and handle error upon failure.
        if let Err(error) = document
          .insert_method(method, scope)
          .map_err(|_| Error::FragmentAlreadyExists)
        {
          return Err(try_undo_key_generation(storage, &key_id, error).await);
        };
  
        // Insert the generated `KeyId` into storage under the computed method digest and handle the error if the
        // operation fails.
        if let Err(error) = <I as KeyIdStorage>::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
          .await
          .map_err(Error::KeyIdStorageError)
        {
          // Remove the method from the document as it can no longer be used.
          let _ = document.remove_method(&method_id);
          return Err(try_undo_key_generation(storage, &key_id, error).await);
        }
  
        Ok(fragment)
      }
    };
}



/// Extension trait for JWK-based operations on DID documents.
///
/// This trait is deliberately sealed and cannot be implemented by external crates.
/// The trait only exists as an extension of existing DID documents implemented in
/// dependent crates. Because those crates cannot also depend on this crate,
/// the extension trait is necessary. External crates however should simply wrap the methods
/// on the trait if they wish to reexport them on their DID document type.
/// This also allows them to use their own error type on those methods.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwpDocumentExt {
  /// Generate new key material in the given `storage` and insert a new verification method with the corresponding
  /// public key material into the DID document.
  ///
  /// - If no fragment is given the `kid` of the generated JWK is used, if it is set, otherwise an error is returned.
  /// - The `key_type` must be compatible with the given `storage`. [`Storage`]s are expected to export key type
  ///   constants for that use case.
  ///
  /// The fragment of the generated method is returned.
  async fn generate_method<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: ProofAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage;
}


// ====================================================================================================================
// CoreDocument
// ====================================================================================================================


generate_method_for_document_type!(CoreDocument, generate_method_core_document);


#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwpDocumentExt for CoreDocument {
  async fn generate_method<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: ProofAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<String>
  where
    K: JwkStorageExt,
    I: KeyIdStorage,
  {
    generate_method_core_document(self, storage, key_type, alg, fragment, scope).await
  }

}





// ====================================================================================================================
// IotaDocument
// ====================================================================================================================
#[cfg(feature = "iota-document")]
mod iota_document {
  use super::*;
  use identity_credential::credential::Jwt;
  use identity_iota_core::IotaDocument;

  generate_method_for_document_type!(IotaDocument, generate_method_iota_document);

  #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
  #[cfg_attr(feature = "send-sync-storage", async_trait)]
  impl JwpDocumentExt for IotaDocument {
    async fn generate_method<K, I>(
      &mut self,
      storage: &Storage<K, I>,
      key_type: KeyType,
      alg: ProofAlgorithm,
      fragment: Option<&str>,
      scope: MethodScope,
    ) -> StorageResult<String>
    where
      K: JwkStorageExt,
      I: KeyIdStorage,
    {
      generate_method_iota_document(self, storage, key_type, alg, fragment, scope).await
    }

}
}