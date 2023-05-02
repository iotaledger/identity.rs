// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::key_id_storage::KeyIdStorage;
use crate::KeyIdStorageResult;

use crate::key_id_storage::method_digest::MethodDigest;
use crate::key_id_storage::KeyIdStorageError;
use crate::key_id_storage::KeyIdStorageErrorKind;
use crate::key_storage::KeyId;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::utils::BaseEncoding;
use identity_did::CoreDID;
use identity_verification::VerificationMethod;

pub(crate) async fn test_storage_operations(storage: impl KeyIdStorage) {
  // Create a Verification Method.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let did: CoreDID = CoreDID::parse(format!("did:example:{}", BaseEncoding::encode_base58(keypair.public()))).unwrap();
  let verification_method: VerificationMethod =
    VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "frag_1").unwrap();

  // Test insertion.
  let key_id_1 = KeyId::new("keyid");
  let method_digest: MethodDigest = MethodDigest::new(&verification_method).unwrap();
  storage
    .insert_key_id(method_digest.clone(), key_id_1.clone())
    .await
    .expect("inserting into memstore failed");

  // Double insertion.
  let insertion_result = storage.insert_key_id(method_digest.clone(), key_id_1.clone()).await;
  let _expected_error: KeyIdStorageError = KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdAlreadyExists);
  assert!(matches!(insertion_result.unwrap_err(), _expected_error));

  // Test retrieving.
  let key_id: KeyId = storage.get_key_id(&method_digest).await.unwrap();
  assert_eq!(key_id_1, key_id);

  // Test deletion.
  storage.delete_key_id(&method_digest).await.expect("deletion failed");

  let retrieval_result: KeyIdStorageResult<KeyId> = storage.get_key_id(&method_digest).await;
  let _expected_error: KeyIdStorageError = KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound);
  assert!(matches!(retrieval_result.unwrap_err(), _expected_error));

  // Double Deletion.
  let repeat_deletion_result: KeyIdStorageResult<()> = storage.delete_key_id(&method_digest).await;
  let _expected_error: KeyIdStorageError = KeyIdStorageError::new(KeyIdStorageErrorKind::KeyIdNotFound);
  assert!(matches!(repeat_deletion_result.unwrap_err(), _expected_error));
}
