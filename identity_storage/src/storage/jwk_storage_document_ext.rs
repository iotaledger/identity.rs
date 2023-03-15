// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::KeyIdStorage;
use crate::key_id_storage::KeyIdStorageResult;
use crate::key_id_storage::MethodDigest;
use crate::key_storage::JwkGenOutput;
use crate::key_storage::JwkStorage;
use crate::key_storage::KeyId;
use crate::key_storage::KeyStorageResult;
use crate::key_storage::KeyType;

use super::JwkStorageDocumentError as Error;
use super::Storage;

use async_trait::async_trait;
// use identity_credential::credential::Credential;
// use identity_credential::presentation::Presentation;
use identity_did::DIDUrl;
use identity_document::document::CoreDocument;
use identity_jose::jws::JwsAlgorithm;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
// use serde::Serialize;
pub type StorageResult<T> = Result<T, Error>;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwkStorageDocumentExt {
  /// Generate new key material in the given `storage` and insert a new verification method with the corresponding
  /// public key material into the DID document.
  // TODO: Also make it possible to set the value of `kid`. This will require changes to the `JwkStorage`.
  async fn generate_method<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: JwsAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  /// Remove the method identified by the given fragment from the document and delete the corresponding key material in
  /// the given `storage`.
  //
  // TODO: Should we take fragment instead of id? id is consistent with CoreDocument::remove_method, but if we expect
  // the fragment to often be the same as a JWK's kid it could be convenient to pass in a kid value here.
  async fn purge_method<K, I>(&mut self, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  async fn sign_bytes<K, I>(&self, storage: &Storage<K, I>, fragment: &str, data: Vec<u8>) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  /* TODO: Add and implement these methods later. This requires some more work on converting Credentials & Presentations to JWT which
  should be addressed in a separate PR.

  async fn create_presentation_jwt<K, I, T, U>(
    &self,
    fragment: &str,
    presentation: &Presentation<T, U>,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize,
    U: Serialize
  ;

  async fn create_credential_jwt<K, I, T>(&self, credential: &Credential<T>, fragment: &str) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize
  ;
  */
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwkStorageDocumentExt for CoreDocument {
  async fn generate_method<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: JwsAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    let JwkGenOutput { key_id, jwk } = <K as JwkStorage>::generate(&storage.key_storage(), key_type, alg)
      .await
      .map_err(Error::KeyStorageError)?;

    // Produce a new verification method containing the generated JWK. If this operation fails we handle the error
    // by attempting to revert key generation before returning an error.
    let method: VerificationMethod = {
      match VerificationMethod::new_from_jwk(self.id(), jwk, fragment)
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

    // Insert method into document and handle error upon failure.
    if let Err(error) = CoreDocument::insert_method(&mut self, method, scope).map_err(|_| Error::FragmentAlreadyExists)
    {
      return Err(try_undo_key_generation(storage, &key_id, error).await);
    };

    // Insert the generated `KeyId` into storage under the computed method digest and handle the error if the operation
    // fails.
    if let Err(error) = <I as KeyIdStorage>::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
      .await
      .map_err(Error::KeyIdStorageError)
    {
      // Remove the method from the document as it can no longer be used.
      let _ = self.remove_method(&method_id);
      return Err(try_undo_key_generation(storage, &key_id, error).await);
    }

    Ok(())
  }

  async fn purge_method<K, I>(&mut self, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    let (method, scope) = self.remove_method_get_scope(id).ok_or(Error::MethodNotFound)?;

    // Obtain method digest and handle error if this operation fails.
    let method_digest: MethodDigest = match MethodDigest::new(&method).map_err(Error::MethodDigestConstructionError) {
      Ok(digest) => digest,
      Err(error) => {
        // Revert state by reinserting the method before returning the error.
        let _ = self.insert_method(method, scope);
        return Err(error);
      }
    };

    // Obtain key id and handle error upon failure.
    let key_id: KeyId = match <I as KeyIdStorage>::get_key_id(&storage.key_id_storage(), &method_digest)
      .await
      .map_err(Error::KeyIdStorageError)
    {
      Ok(key_id) => key_id,
      Err(error) => {
        // Reinsert method before returning.
        let _ = self.insert_method(method, scope);
        return Err(error);
      }
    };

    // Delete key and key id concurrently.
    let key_deletion_fut = <K as JwkStorage>::delete(&storage.key_storage(), &key_id);
    let key_id_deletion_fut = <I as KeyIdStorage>::delete_key_id(&storage.key_id_storage(), &method_digest);
    let (key_deletion_result, key_id_deletion_result): (KeyStorageResult<()>, KeyIdStorageResult<()>) =
      futures::join!(key_deletion_fut, key_id_deletion_fut);
    // Check for any errors that may have occurred. Unfortunately this is somewhat involved.
    match (key_deletion_result, key_id_deletion_result) {
      (Ok(_), Ok(_)) => Ok(()),
      (Ok(_), Err(key_id_deletion_error)) => {
        // Cannot attempt to revert this operation as the JwkStorage may not return the same KeyId when
        // JwkStorage::insert is called.
        Err(Error::UndoOperationFailed {
          message: format!(
            "cannot undo key deletion. This results in a stray key id stored under packed method digest: {:?}",
            &method_digest.pack()
          ),
          source: Box::new(Error::KeyIdStorageError(key_id_deletion_error)),
          undo_error: None,
        })
      }
      (Err(key_deletion_error), Ok(_)) => {
        // Attempt to revert: Reinsert key id and method if possible.
        if let Err(key_id_insertion_error) =
          <I as KeyIdStorage>::insert_key_id(&storage.key_id_storage(), (&method_digest).clone(), key_id.clone())
            .await
            .map_err(Error::KeyIdStorageError)
        {
          Err(Error::UndoOperationFailed {
            message: format!("cannot revert key id deletion. This results in stray key with key id: {key_id}"),
            source: Box::new(Error::KeyStorageError(key_deletion_error)),
            undo_error: Some(Box::new(key_id_insertion_error)),
          })
        } else {
          // KeyId reinsertion succeeded. Now reinsert method.
          let _ = self.insert_method(method, scope);
          Err(Error::KeyStorageError(key_deletion_error))
        }
      }
      (Err(_key_deletion_error), Err(key_id_deletion_error)) => {
        // We assume this means nothing got deleted. Reinsert the method and return one of the errors (perhaps
        // key_id_deletion_error as we really expect the key id storage to work as expected at this point).
        let _ = self.insert_method(method, scope);
        Err(Error::KeyIdStorageError(key_id_deletion_error))
      }
    }
  }

  async fn sign_bytes<K, I>(&self, storage: &Storage<K, I>, fragment: &str, data: Vec<u8>) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    todo!()
  }
}

/// Attempt to revert key generation if this succeeds the original `source_error` is returned,
/// otherwise [`JwkStorageDocumentError::UndoOperationFailed`] is returned with the `source_error` attached as
/// `source`.
async fn try_undo_key_generation<K, I>(storage: &Storage<K, I>, key_id: &KeyId, source_error: Error) -> Error
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  // Undo key generation
  if let Err(err) = <K as JwkStorage>::delete(&storage.key_storage(), &key_id).await {
    return Error::UndoOperationFailed {
      message: format!("unable to delete stray key with id: {}", &key_id),
      source: Box::new(source_error),
      undo_error: Some(Box::new(Error::KeyStorageError(err))),
    };
  } else {
    return source_error;
  }
}
