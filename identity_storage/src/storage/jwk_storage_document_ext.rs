// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::KeyIdStorage;
use crate::key_id_storage::MethodDigest;
use crate::key_storage::JwkGenOutput;
use crate::key_storage::JwkStorage;
use crate::key_storage::KeyId;
use crate::key_storage::KeyType;

use super::JwkStorageDocumentError;
use super::Storage;

use async_trait::async_trait;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::DIDUrl;
use identity_document::document::CoreDocument;
use identity_jose::jws::JwsAlgorithm;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use serde::Serialize;
pub type StorageResult<T> = Result<T, JwkStorageDocumentError>;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwkStorageDocumentExt {
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

  async fn remove_method<K, I>(&mut self, storage: &Storage<K, I>, fragment: &str) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  async fn sign_bytes<K, I>(&self, storage: &Storage<K, I>, fragment: &str, data: Vec<u8>) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  async fn create_presentation_jwt<K, I, T, U>(
    &self,
    fragment: &str,
    presentation: &Presentation<T, U>,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize,
    U: Serialize,
  {
    todo!()
  }

  async fn create_credential_jwt<K, I, T>(&self, credential: &Credential<T>, fragment: &str) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize,
  {
    todo!()
  }
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
      .map_err(JwkStorageDocumentError::KeyStorageError)?;

    let method: VerificationMethod = {
      match VerificationMethod::new_from_jwk(self.id(), jwk, fragment)
        .map_err(|err| JwkStorageDocumentError::VerificationMethodConstructionError(err))
      {
        Ok(method) => method,
        Err(source) => {
          return Err(undo_key_generation(storage, &key_id, source).await);
        }
      }
    };

    let method_digest: MethodDigest = MethodDigest::new(&method);
    let method_id: DIDUrl = method.id().clone();

    if let Err(error) =
      CoreDocument::insert_method(&mut self, method, scope).map_err(|_| JwkStorageDocumentError::FragmentAlreadyExists)
    {
      return Err(undo_key_generation(storage, &key_id, error).await);
    };

    if let Err(error) = <I as KeyIdStorage>::insert_key_id(&storage.key_id_storage(), method_digest, key_id.clone())
      .await
      .map_err(JwkStorageDocumentError::KeyIdStorageError)
    {
      // Remove method from document
      let _ = self.remove_method(&method_id);
      return Err(undo_key_generation(storage, &key_id, error).await);
    }

    Ok(())
  }

  async fn remove_method<K, I>(&mut self, storage: &Storage<K, I>, fragment: &str) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    todo!()
  }

  async fn sign_bytes<K, I>(&self, storage: &Storage<K, I>, fragment: &str, data: Vec<u8>) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    todo!()
  }

  async fn create_presentation_jwt<K, I, T, U>(
    &self,
    fragment: &str,
    presentation: &Presentation<T, U>,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize,
    U: Serialize,
  {
    todo!()
  }

  async fn create_credential_jwt<K, I, T>(&self, credential: &Credential<T>, fragment: &str) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: Serialize,
  {
    todo!()
  }
}

async fn undo_key_generation<K, I>(
  storage: &Storage<K, I>,
  key_id: &KeyId,
  source_error: JwkStorageDocumentError,
) -> JwkStorageDocumentError
where
  K: JwkStorage,
  I: KeyIdStorage,
{
  // Undo key generation
  if let Err(err) = <K as JwkStorage>::delete(&storage.key_storage(), &key_id).await {
    return JwkStorageDocumentError::UndoOperationFailed {
      message: format!("unable to delete stray key with id: {}", &key_id),
      source: Box::new(source_error),
      undo_errors: vec![JwkStorageDocumentError::KeyStorageError(err)],
    };
  } else {
    return source_error;
  }
}
