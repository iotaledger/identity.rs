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

use super::JwsSignatureOptions;
use async_trait::async_trait;
use identity_credential::credential::Credential;
use identity_did::DIDUrl;
use identity_document::document::CoreDocument;
use identity_verification::jose::jws::CompactJwsEncoder;
use identity_verification::jose::jws::CompactJwsEncodingOptions;
use identity_verification::jose::jws::JwsAlgorithm;
use identity_verification::jose::jws::JwsHeader;
use identity_verification::jws::CharSet;
use identity_verification::MethodData;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub type StorageResult<T> = Result<T, Error>;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait JwkStorageDocumentExt: private::Sealed {
  /// Generate new key material in the given `storage` and insert a new verification method with the corresponding
  /// public key material into the DID document. The `kid` of the generated Jwk is returned if it is set.
  // TODO: Also make it possible to set the value of `kid`. This will require changes to the `JwkStorage`.
  async fn generate_method<K, I>(
    &mut self,
    storage: &Storage<K, I>,
    key_type: KeyType,
    alg: JwsAlgorithm,
    fragment: Option<&str>,
    scope: MethodScope,
  ) -> StorageResult<Option<String>>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  /// Remove the method identified by the given fragment from the document and delete the corresponding key material in
  /// the given `storage`.
  async fn purge_method<K, I>(&mut self, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  /// Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
  /// material in the verification method identified by the given `fragment.
  ///
  /// Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
  /// See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).   
  async fn sign_bytes<K, I>(
    &self,
    storage: &Storage<K, I>,
    fragment: &str,
    payload: &[u8],
    options: &JwsSignatureOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage;

  /// Produces a JWS where the payload is produced from the given `credential`
  /// in accordance with [VC-JWT version 1.1.](https://w3c.github.io/vc-jwt/#version-1.1).
  ///
  /// The `kid` in the protected header is the `id` of the method identified by `fragment` and the JWS signature will be
  /// produced by the corresponding private key backed by the `storage` in accordance with the passed `options`.
  async fn sign_credential<K, I, T>(
    &self,
    credential: &Credential<T>,
    storage: &Storage<K, I>,
    fragment: &str,
    options: &JwsSignatureOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: ToOwned + Serialize + Sync,
    <T as ToOwned>::Owned: DeserializeOwned;
}

mod private {
  pub trait Sealed {}
  impl Sealed for identity_document::document::CoreDocument {}
  #[cfg(feature = "iota-document")]
  impl Sealed for identity_iota_core::IotaDocument {}
}

// ====================================================================================================================
// Implementation
// ====================================================================================================================

// We want to implement this trait both for CoreDocument and IotaDocument, but the methods that take `&mut self` cannot
// be implemented in terms of &mut CoreDocument for IotaDocument. To work around this limitation we use macros to avoid
// copious amounts of repetition.
// NOTE: If such use of macros becomes very common it is probably better to use the duplicate crate: https://docs.rs/duplicate/latest/duplicate/
macro_rules! generate_method_for_document_type {
  ($t:ty, $name:ident) => {
    async fn $name<K, I>(
      document: &mut $t,
      storage: &Storage<K, I>,
      key_type: KeyType,
      alg: JwsAlgorithm,
      fragment: Option<&str>,
      scope: MethodScope,
    ) -> StorageResult<Option<String>>
    where
      K: JwkStorage,
      I: KeyIdStorage,
    {
      let JwkGenOutput { key_id, jwk } = <K as JwkStorage>::generate(&storage.key_storage(), key_type, alg)
        .await
        .map_err(Error::KeyStorageError)?;
      let kid = jwk.kid().map(ToOwned::to_owned);

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

      Ok(kid)
    }
  };
}

macro_rules! purge_method_for_document_type {
  ($t:ty, $name:ident) => {
    async fn $name<K, I>(document: &mut $t, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
    where
      K: JwkStorage,
      I: KeyIdStorage,
    {
      let (method, scope) = document.remove_method_get_scope(id).ok_or(Error::MethodNotFound)?;

      // Obtain method digest and handle error if this operation fails.
      let method_digest: MethodDigest = match MethodDigest::new(&method).map_err(Error::MethodDigestConstructionError) {
        Ok(digest) => digest,
        Err(error) => {
          // Revert state by reinserting the method before returning the error.
          let _ = document.insert_method(method, scope);
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
          let _ = document.insert_method(method, scope);
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
            let _ = document.insert_method(method, scope);
            Err(Error::KeyStorageError(key_deletion_error))
          }
        }
        (Err(_key_deletion_error), Err(key_id_deletion_error)) => {
          // We assume this means nothing got deleted. Reinsert the method and return one of the errors (perhaps
          // key_id_deletion_error as we really expect the key id storage to work as expected at this point).
          let _ = document.insert_method(method, scope);
          Err(Error::KeyIdStorageError(key_id_deletion_error))
        }
      }
    }
  };
}

// ====================================================================================================================
// CoreDocument
// ====================================================================================================================

generate_method_for_document_type!(CoreDocument, generate_method_core_document);
purge_method_for_document_type!(CoreDocument, purge_method_core_document);

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
  ) -> StorageResult<Option<String>>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    generate_method_core_document(self, storage, key_type, alg, fragment, scope).await
  }

  async fn purge_method<K, I>(&mut self, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    purge_method_core_document(self, storage, id).await
  }

  async fn sign_bytes<K, I>(
    &self,
    storage: &Storage<K, I>,
    fragment: &str,
    payload: &[u8],
    options: &JwsSignatureOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
    // Obtain the method corresponding to the given fragment.
    let method: &VerificationMethod = self.resolve_method(fragment, None).ok_or(Error::MethodNotFound)?;
    let MethodData::PublicKeyJwk(ref jwk) = method.data() else {
      return Err(Error::NotPublicKeyJwk)
    };
    // Extract JwsAlgorithm
    let alg: JwsAlgorithm = jwk
      .alg()
      .unwrap_or("")
      .parse()
      .map_err(|_| Error::InvalidJwsAlgorithm)?;

    // create JWS header in accordance with options
    let header: JwsHeader = {
      let mut header = JwsHeader::new();

      header.set_alg(alg);

      header.set_kid(method.id().to_string());

      if options.attach_jwk {
        header.set_jwk(jwk.clone())
      };

      if let Some(b64) = options.b64 {
        header.set_b64(b64)
      };

      if let Some(typ) = &options.typ {
        header.set_typ(typ.clone())
      };

      if let Some(cty) = &options.cty {
        header.set_cty(cty.clone())
      };

      if let Some(crit) = &options.crit {
        header.set_crit(crit.iter().cloned())
      };

      if let Some(url) = &options.url {
        header.set_url(url.clone())
      };

      if let Some(nonce) = &options.nonce {
        header.set_nonce(nonce.clone())
      };
      header
    };

    // Get the key identifier corresponding to the given method from the KeyId storage.
    let method_digest: MethodDigest = MethodDigest::new(method).map_err(Error::MethodDigestConstructionError)?;
    let key_id = <I as KeyIdStorage>::get_key_id(storage.key_id_storage(), &method_digest)
      .await
      .map_err(Error::KeyIdStorageError)?;

    // Extract Compact JWS encoding options.
    let encoding_options: CompactJwsEncodingOptions = if !options.detached_payload {
      // We use this as a default and don't provide the extra UrlSafe check for now.
      // Applications that require such checks can easily do so after JWS creation.
      CompactJwsEncodingOptions::NonDetached {
        charset_requirements: CharSet::Default,
      }
    } else {
      CompactJwsEncodingOptions::Detached
    };

    let jws_encoder: CompactJwsEncoder = CompactJwsEncoder::new_with_options(payload, &header, encoding_options)
      .map_err(|err| Error::EncodingError(err.into()))?;
    let signature = <K as JwkStorage>::sign(storage.key_storage(), &key_id, jws_encoder.signing_input(), jwk)
      .await
      .map_err(Error::KeyStorageError)?;
    Ok(jws_encoder.into_jws(&signature))
  }

  async fn sign_credential<K, I, T>(
    &self,
    credential: &Credential<T>,
    storage: &Storage<K, I>,
    fragment: &str,
    options: &JwsSignatureOptions,
  ) -> StorageResult<String>
  where
    K: JwkStorage,
    I: KeyIdStorage,
    T: ToOwned + Serialize + Sync,
    <T as ToOwned>::Owned: DeserializeOwned,
  {
    let payload = credential
      .serialize_jwt()
      .map_err(Error::ClaimsSerializationError)?;
    self.sign_bytes(storage, fragment, payload.as_bytes(), options).await
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
  if let Err(err) = <K as JwkStorage>::delete(storage.key_storage(), key_id).await {
    Error::UndoOperationFailed {
      message: format!("unable to delete stray key with id: {}", &key_id),
      source: Box::new(source_error),
      undo_error: Some(Box::new(Error::KeyStorageError(err))),
    }
  } else {
    source_error
  }
}

// ====================================================================================================================
// IotaDocument
// ====================================================================================================================
#[cfg(feature = "iota-document")]
mod iota_document {
  use super::*;
  use identity_iota_core::IotaDocument;
  generate_method_for_document_type!(IotaDocument, generate_method_iota_document);
  purge_method_for_document_type!(IotaDocument, purge_method_iota_document);

  #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
  #[cfg_attr(feature = "send-sync-storage", async_trait)]
  impl JwkStorageDocumentExt for IotaDocument {
    async fn generate_method<K, I>(
      &mut self,
      storage: &Storage<K, I>,
      key_type: KeyType,
      alg: JwsAlgorithm,
      fragment: Option<&str>,
      scope: MethodScope,
    ) -> StorageResult<Option<String>>
    where
      K: JwkStorage,
      I: KeyIdStorage,
    {
      generate_method_iota_document(self, storage, key_type, alg, fragment, scope).await
    }

    async fn purge_method<K, I>(&mut self, storage: &Storage<K, I>, id: &DIDUrl) -> StorageResult<()>
    where
      K: JwkStorage,
      I: KeyIdStorage,
    {
      purge_method_iota_document(self, storage, id).await
    }

    async fn sign_bytes<K, I>(
      &self,
      storage: &Storage<K, I>,
      fragment: &str,
      payload: &[u8],
      options: &JwsSignatureOptions,
    ) -> StorageResult<String>
    where
      K: JwkStorage,
      I: KeyIdStorage,
    {
      self
        .core_document()
        .sign_bytes(storage, fragment, payload, options)
        .await
    }

    async fn sign_credential<K, I, T>(
      &self,
      credential: &Credential<T>,
      storage: &Storage<K, I>,
      fragment: &str,
      options: &JwsSignatureOptions,
    ) -> StorageResult<String>
    where
      K: JwkStorage,
      I: KeyIdStorage,
      T: ToOwned + Serialize + Sync,
      <T as ToOwned>::Owned: DeserializeOwned,
    {
      self
        .core_document()
        .sign_credential(credential, storage, fragment, options)
        .await
    }
  }
}
