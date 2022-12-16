// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::common::KeyComparable;
use identity_data_integrity::verification_material::VerificationMaterial;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodBuilder;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::identifiers::MethodIdx;
use crate::identity_storage::IdentityStorage;
use crate::identity_storage::IdentityStorageErrorKind;
use crate::key_generation::MultikeyOutput;
use crate::key_generation::MultikeySchema;
use crate::key_storage::KeyStorage;
use crate::key_storage::KeyStorageErrorKind;
use crate::storage::Storage;

use super::method_creation_error::MethodCreationError;
use super::method_removal_error::MethodRemovalError;
use super::storage_error::StorageError;
use super::MethodCreationErrorKind;
use super::MethodRemovalErrorKind;

#[async_trait(?Send)]
pub trait CoreDocumentExt: private::Sealed {
  type D: DID + KeyComparable;
  /// Create a new verification method of type `Multikey`
  /// whose key material is backed by a [`Storage`](crate::storage::Storage).  
  ///
  /// The `schema` parameter declares what kind of key you wish the
  /// storage to generate, while `fragment` defines the fragment of the created method's relative DIDUrl and `scope`
  /// declares what purpose the created method can be used for. See the section on verification relationships in [the data integrity specification](https://w3c.github.io/vc-data-integrity/#verification-relationships).
  async fn create_multikey_method<K, I>(
    &mut self,
    fragment: &str,
    schema: &MultikeySchema,
    storage: &Storage<K, I>,
    scope: MethodScope,
  ) -> Result<(), MethodCreationError>
  where
    K: KeyStorage,
    I: IdentityStorage;

  /// Remove the method from the document and delete
  /// the corresponding keys and metadata from the [`Storage`](crate::storage::Storage).
  ///
  /// # Warning
  /// This operation cannot be undone.
  async fn purge_method<K, I>(
    &mut self,
    did_url: &DIDUrl<Self::D>,
    storage: &Storage<K, I>,
  ) -> Result<(), MethodRemovalError>
  where
    K: KeyStorage,
    I: IdentityStorage;
}

mod private {
  use identity_core::common::KeyComparable;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;

  pub trait Sealed {}

  impl<D, T, U, V> Sealed for CoreDocument<D, T, U, V> where D: DID + KeyComparable {}
}

#[async_trait(?Send)]
impl<D: DID + KeyComparable, T, U, V> CoreDocumentExt for CoreDocument<D, T, U, V>
where
  U: Default,
{
  type D = D;
  async fn create_multikey_method<K, I>(
    &mut self,
    fragment: &str,
    schema: &MultikeySchema,
    storage: &Storage<K, I>,
    scope: MethodScope,
  ) -> Result<(), MethodCreationError>
  where
    K: KeyStorage,
    I: IdentityStorage,
  {
    // Check if the fragment already exists
    if self.refers_to_sub_resource(fragment) {
      return Err(MethodCreationError::from_kind(MethodCreationErrorKind::FragmentInUse));
    }

    let did_url = {
      let mut did_url = self.id().to_url();
      did_url
        .set_fragment(Some(&fragment))
        .map_err(|_| MethodCreationError::from_kind(MethodCreationErrorKind::InvalidFragmentSyntax))?;
      did_url
    };

    // Use the key storage to generate a multikey:

    let MultikeyOutput { public_key, key_id } = match storage.key_storage().generate_multikey(schema).await {
      Ok(output) => output,
      Err(key_storage_error) => {
        let error_kind: MethodCreationErrorKind = match key_storage_error.kind() {
          KeyStorageErrorKind::CouldNotAuthenticate => MethodCreationErrorKind::KeyStorageAuthenticationFailure,
          KeyStorageErrorKind::RetryableIOFailure => MethodCreationErrorKind::RetryableIOFailure,
          KeyStorageErrorKind::UnavailableKeyStorage => MethodCreationErrorKind::UnavailableStorage,
          KeyStorageErrorKind::UnsupportedMultikeySchema => MethodCreationErrorKind::UnsupportedMultikeySchema,
          KeyStorageErrorKind::Unspecified => MethodCreationErrorKind::UnspecifiedStorageFailure,
          // The other variants should not be relevant for this operation
          KeyStorageErrorKind::KeyNotFound | KeyStorageErrorKind::UnsupportedSigningKey => {
            MethodCreationErrorKind::UnspecifiedStorageFailure
          }
        };
        return Err(MethodCreationError::new(
          error_kind,
          StorageError::KeyStorage(key_storage_error),
        ));
      }
    };

    let method_idx: MethodIdx = MethodIdx::new_from_multikey(fragment, &public_key);

    if let Err(identity_storage_error) = storage.identity_storage().store_key_id(method_idx, &key_id).await {
      // Attempt to rollback key generation
      if let Err(key_storage_error) = storage.key_storage().delete(&key_id).await {
        let error_kind: MethodCreationErrorKind = MethodCreationErrorKind::TransactionRollbackFailure;
        return Err(MethodCreationError::new(
          error_kind,
          StorageError::Both(Box::new((key_storage_error, identity_storage_error))),
        ));
      } else {
        // Rollback succeeded, only need to report the identity storage error
        let error_kind = match identity_storage_error.kind() {
          IdentityStorageErrorKind::CouldNotAuthenticate => {
            MethodCreationErrorKind::IdentityStorageAuthenticationFailure
          }
          IdentityStorageErrorKind::MethodIdxAlreadyExists => MethodCreationErrorKind::MethodMetadataAlreadyStored,
          IdentityStorageErrorKind::RetryableIOFailure => MethodCreationErrorKind::RetryableIOFailure,
          IdentityStorageErrorKind::UnavailableIdentityStorage => MethodCreationErrorKind::UnavailableStorage,
          IdentityStorageErrorKind::Unspecified => MethodCreationErrorKind::UnspecifiedStorageFailure,
          // The other variants should not be relevant for this operation
          IdentityStorageErrorKind::MethodIdxNotFound => MethodCreationErrorKind::UnspecifiedStorageFailure,
        };
        return Err(MethodCreationError::new(
          error_kind,
          StorageError::IdentityStorage(identity_storage_error),
        ));
      }
    }

    let method = MethodBuilder::<D, U>::default()
      .id(did_url)
      .type_(MethodType::MULTIKEY)
      // This line should be removed once we replace MethodData with VerificationMaterial
      .data(MethodData::PublicKeyMultibase(public_key.as_str().to_owned()))
      .material(VerificationMaterial::PublicKeyMultibase(public_key))
      .build()
      .expect("building a method with valid data should be fine");

    self
      .insert_method(method, scope)
      .map_err(|_| MethodCreationError::from_kind(MethodCreationErrorKind::FragmentInUse))
  }

  async fn purge_method<K, I>(&mut self, did_url: &DIDUrl<D>, storage: &Storage<K, I>) -> Result<(), MethodRemovalError>
  where
    K: KeyStorage,
    I: IdentityStorage,
  {
    // TODO: What to do about the VerificationMaterial::Multikey variant?
    let public_key_multibase = self
      .resolve_method(did_url, None)
      .and_then(VerificationMethod::material)
      .map(|material| match material {
        VerificationMaterial::PublicKeyMultibase(ref public_key_multibase) => public_key_multibase,
      })
      .ok_or_else(|| MethodRemovalError::from_kind(MethodRemovalErrorKind::MethodNotFound))?;

    let method_idx = MethodIdx::new_from_multikey(did_url.fragment().unwrap_or_default(), public_key_multibase);

    match storage.identity_storage().get_key_id(method_idx).await {
      Ok(key_id) => {
        if let Err(key_storage_error) = storage.key_storage().delete(&key_id).await {
          let error_kind = match key_storage_error.kind() {
            KeyStorageErrorKind::KeyNotFound => MethodRemovalErrorKind::KeyNotFound,
            KeyStorageErrorKind::CouldNotAuthenticate => MethodRemovalErrorKind::KeyStorageAuthenticationFailure,
            KeyStorageErrorKind::RetryableIOFailure => MethodRemovalErrorKind::RetryableIOFailure,
            KeyStorageErrorKind::UnavailableKeyStorage => MethodRemovalErrorKind::UnavailableStorage,
            KeyStorageErrorKind::Unspecified => MethodRemovalErrorKind::UnspecifiedStorageFailure,
            // Other variants are irrelevant
            KeyStorageErrorKind::UnsupportedMultikeySchema | KeyStorageErrorKind::UnsupportedSigningKey => {
              MethodRemovalErrorKind::UnspecifiedStorageFailure
            }
          };
          return Err(MethodRemovalError::new(error_kind, key_storage_error.into()));
        } else {
          // The key material has been removed
          //TODO: Also delete from IdentityStorage
          self
            .remove_method(did_url)
            .ok_or_else(|| MethodRemovalError::from_kind(MethodRemovalErrorKind::MethodNotFound))?;
          Ok(())
        }
      }
      Err(identity_storage_error) => {
        let error_kind = match identity_storage_error.kind() {
          IdentityStorageErrorKind::CouldNotAuthenticate => {
            MethodRemovalErrorKind::IdentityStorageAuthenticationFailure
          }
          IdentityStorageErrorKind::MethodIdxNotFound => MethodRemovalErrorKind::MethodMetadataNotFound,
          IdentityStorageErrorKind::RetryableIOFailure => MethodRemovalErrorKind::RetryableIOFailure,
          IdentityStorageErrorKind::UnavailableIdentityStorage => MethodRemovalErrorKind::UnavailableStorage,
          IdentityStorageErrorKind::Unspecified => MethodRemovalErrorKind::UnspecifiedStorageFailure,
          // Other variants are irrelevant
          IdentityStorageErrorKind::MethodIdxAlreadyExists => MethodRemovalErrorKind::UnspecifiedStorageFailure,
        };
        Err(MethodRemovalError::new(error_kind, identity_storage_error.into()))
      }
    }
  }
}
