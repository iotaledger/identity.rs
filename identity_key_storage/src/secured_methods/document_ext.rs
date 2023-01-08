// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::common::KeyComparable;
use identity_data_integrity::verification_material::Multikey;
use identity_data_integrity::verification_material::PublicKeyMultibase;
use identity_data_integrity::verification_material::VerificationMaterial;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodBuilder;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::identifiers::MethodId;
use crate::identity_storage::IdentityStorage;
use crate::key_generation::MultikeyOutput;
use crate::key_generation::MultikeySchema;
use crate::key_storage::KeyStorage;

use crate::storage::Storage;

use super::method_creation_error::MethodCreationError;
use super::method_removal_error::MethodRemovalError;
use super::signing_material::SigningMaterial;
use super::KeyLookupError;
use super::RemoteKey;

#[async_trait(?Send)]
/// Extension trait enabling [`CoreDocument`] to utilize
/// key material secured by a [`Storage`](crate::storage::Storage).
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
  ///
  /// This operation cannot be undone.
  ///
  /// # Behaviour
  ///
  /// If the key material corresponding to the verification method is successfully removed from the underlying
  /// [`KeyStorage`] the verification method will also be removed from the document. There is still a non-zero chance
  /// that the [`IdentityStorage`] fails to delete the metadata of the verification method in which case an error is
  /// returned. In other words the absence of an `Ok`, does not necessarily mean that the method still exists in the
  /// document.
  async fn purge_method<K, I>(
    &mut self,
    did_url: &DIDUrl<Self::D>,
    storage: &Storage<K, I>,
  ) -> Result<(), MethodRemovalError>
  where
    K: KeyStorage,
    I: IdentityStorage;

  /// Extracts the [`SigningMaterial`] corresponding to the verification method with the given `fragment`.   
  async fn signing_material<K, I, F>(
    &self,
    fragment: &str,
    scope: MethodRelationship,
    storage: &Storage<K, I>,
  ) -> Result<SigningMaterial<F>, KeyLookupError>
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
      return Err(MethodCreationError::FragmentInUse);
    }

    let did_url = {
      let mut did_url = self.id().to_url();
      did_url
        .set_fragment(Some(&fragment))
        .map_err(|_| MethodCreationError::InvalidFragmentSyntax)?;
      did_url
    };

    // Use the key storage to generate a multikey:
    let MultikeyOutput { public_key, key_id } = storage
      .key_storage()
      .generate_multikey(schema)
      .await
      .map_err(|key_storage_err| MethodCreationError::KeyGenerationFailure(key_storage_err))?;

    let method_id: MethodId = MethodId::new_from_multikey(fragment.strip_prefix('#').unwrap_or(fragment), &public_key);

    // Note: If metadata cannot be persisted, then the generated key will be "stuck" in storage.
    // TODO: Do we need to provide a way for users to manually delete the key material from storage when this occurs?
    storage
      .identity_storage()
      .store_key_id(method_id, key_id.clone())
      .await
      .map_err(|err| MethodCreationError::MetadataPersistenceFailure(err))?;

    let method = MethodBuilder::<D, U>::default()
      .controller(did_url.did().to_owned())
      .id(did_url)
      .type_(MethodType::MULTIKEY)
      // This line should be removed once we replace MethodData with VerificationMaterial
      .data(MethodData::PublicKeyMultibase(public_key.as_multibase_str().to_owned()))
      .material(VerificationMaterial::PublicKeyMultibase(PublicKeyMultibase::new(
        public_key.into_multibase_string(),
      )))
      .build()
      .expect("building a method with valid data should be fine");

    Ok(
      self
        .insert_method(method, scope)
        .expect("inserting a method with a unique identifier should be fine"),
    )
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
      .and_then(|material| match material {
        VerificationMaterial::PublicKeyMultibase(ref public_key_multibase) => Some(public_key_multibase),
        _ => None,
      })
      .ok_or_else(|| MethodRemovalError::MethodNotFound)?;

    // TODO: This only works when type is Multikey, generalize this when we start supporting publicKeyJwk as well.
    // TODO: Introduce a function for extracting a MethodId from a given verification method.
    let method_idx = MethodId::new_from_multikey(
      did_url.fragment().unwrap_or_default(),
      &Multikey::from_multibase_string(public_key_multibase.as_str().to_owned()),
    );

    let key_id = storage
      .identity_storage()
      .load_key_id(&method_idx)
      .await
      .map_err(|err| MethodRemovalError::MetadataLookupFailure(err))?;

    storage
      .key_storage()
      .delete(&key_id)
      .await
      .map_err(|err| MethodRemovalError::KeyRemovalFailure(err))?;

    // Attempt to remove metadata.
    // Note: If this operation fails redundant metadata is "stuck" in the identity storage.
    // TODO: Do we need to provide a way for users to manually delete the (method_id, key_id) from storage whenever this
    // occurs?
    let key_id_removal_result = storage
      .identity_storage()
      .delete_key_id(&method_idx)
      .await
      .map_err(|identity_storage_error| MethodRemovalError::PartialRemoval(identity_storage_error));

    // Proceed with removing the method regardless of whether the `key_id` was purged from the identity storage.
    self
      .remove_method(did_url)
      .expect("removing a method known to be contained in the document should be fine");

    key_id_removal_result
  }

  async fn signing_material<K, I, F>(
    &self,
    fragment: &str,
    scope: MethodRelationship,
    storage: &Storage<K, I>,
  ) -> Result<SigningMaterial<F>, KeyLookupError>
  where
    K: KeyStorage,
    I: IdentityStorage,
  {
    let method = self
      .resolve_method(fragment, Some(scope))
      .ok_or(KeyLookupError::MethodNotFound)?;

    // TODO: This only works when type is Multikey, generalize this when we start supporting publicKeyJwk as well.
    // TODO: Introduce a function for extracting a MethodId from a verification method.
    let (Some(method_fragment), Some(public_key_multibase)) = (
      method.id().fragment(),
       method.material().and_then(|material| match material {VerificationMaterial::PublicKeyMultibase(ref key) => Some(key), _=> None})) else {
      return Err(KeyLookupError::MethodNotFound); 
    };
    let method_id = MethodId::new_from_multikey(
      method_fragment,
      &Multikey::from_multibase_string(public_key_multibase.as_str().to_owned()),
    );

    let key_id = storage
      .identity_storage()
      .load_key_id(&method_id)
      .await
      .map_err(|err| KeyLookupError::KeyIdRetrievalFailure(err))?;

    todo!();
  }
}

#[cfg(test)]
mod tests {
  use identity_data_integrity::verification_material::Multicodec;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::verification::MethodRelationship;
  use identity_did::verification::MethodScope;

  use crate::identity_storage::MemIdentityStore;
  use crate::key_generation::MultikeySchema;
  use crate::key_storage::MemKeyStore;
  use crate::storage::Storage;

  use super::CoreDocumentExt;

  #[tokio::test]
  async fn test_method_creation_deletion() {
    let fragment = "#multikey";
    let key_storage = MemKeyStore::new();
    let blob_storage = MemIdentityStore::new();
    let storage = Storage::new(key_storage, blob_storage);

    let did = CoreDID::parse("did:iota:0x0000").unwrap();
    let mut document: CoreDocument = CoreDocument::builder(Default::default()).id(did).build().unwrap();

    document
      .create_multikey_method(
        fragment,
        &MultikeySchema::new(Multicodec::ED25519_PUB),
        &storage,
        MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod),
      )
      .await
      .unwrap();

    assert!(document
      .resolve_method(
        fragment,
        Some(MethodScope::VerificationRelationship(
          MethodRelationship::AssertionMethod
        )),
      )
      .is_some());

    let did_url = document.id().to_url().join(fragment).unwrap();
    document.purge_method(&did_url, &storage).await.unwrap();

    assert!(document
      .resolve_method(
        fragment,
        Some(MethodScope::VerificationRelationship(
          MethodRelationship::AssertionMethod
        )),
      )
      .is_none());
  }
}
