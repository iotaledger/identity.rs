// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::common::KeyComparable;
use identity_data_integrity::verification_material::VerificationMaterial;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodBuilder;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;

use crate::identifiers::MethodIdx;
use crate::identity_storage::IdentityStorage;
use crate::identity_storage::IdentityStorageErrorKind;
use crate::key_generation::MultikeyOutput;
use crate::key_generation::MultikeySchema;
use crate::key_storage::KeyStorage;
use crate::key_storage::KeyStorageErrorKind;
use crate::storage::Storage;

use super::method_creation_error::MethodCreationError;
use super::storage_error::StorageError;
use super::MethodCreationErrorKind;

#[async_trait(?Send)]
pub trait CoreDocumentExt: private::Sealed {
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

  /*
  async fn purge_method<K,I>(
    &mut self,
    did_url: &DIDUrl<D>,
    storage: &Storage<K, I>,
  ) -> Result<(), MethodRemovalError>
  */

  /*
  async fn purge_method<K,I>(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>
  where
      K: KeyStorage,
      I: IdentityStorage;
  */
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

  /*
  async fn purge_method(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>
  {
      todo!()
  }
  */
}
