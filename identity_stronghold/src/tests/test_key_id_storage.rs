// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::utils::create_temp_file;
use super::utils::create_verification_method;
use crate::StrongholdStorage;
use identity_storage::key_id_storage::KeyIdStorageErrorKind;
use identity_storage::key_id_storage::MethodDigest;
use identity_storage::key_storage::KeyId;
use identity_storage::KeyIdStorage;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::Password;
use std::path::PathBuf;

const PASS: &str = "secure_password";

#[tokio::test]
async fn test_stronghold() {
  iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
  let file: PathBuf = create_temp_file();
  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  key_id_tests::test_storage_operations(stronghold_storage).await;
}

#[tokio::test]
async fn write_to_disk() {
  iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
  let file: PathBuf = create_temp_file();
  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();

  let verification_method = create_verification_method();

  let key_id_1 = KeyId::new("keyid");
  let method_digest: MethodDigest = MethodDigest::new(&verification_method).unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  stronghold_storage
    .insert_key_id(method_digest.clone(), key_id_1.clone())
    .await
    .expect("inserting into memstore failed");

  drop(stronghold_storage);

  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);

  let key_id: KeyId = stronghold_storage.get_key_id(&method_digest).await.unwrap();
  assert_eq!(key_id_1, key_id);

  stronghold_storage
    .delete_key_id(&method_digest)
    .await
    .expect("deletion failed");

  drop(stronghold_storage);

  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  let error_kind: KeyIdStorageErrorKind = stronghold_storage
    .get_key_id(&method_digest)
    .await
    .unwrap_err()
    .kind()
    .clone();

  assert!(matches!(error_kind, KeyIdStorageErrorKind::KeyIdNotFound));
}

mod key_id_tests {
  // Copy of test in identity_storage.

  use identity_storage::key_id_storage::KeyIdStorage;
  use identity_storage::key_id_storage::KeyIdStorageError;
  use identity_storage::key_id_storage::KeyIdStorageErrorKind;
  use identity_storage::key_id_storage::KeyIdStorageResult;
  use identity_storage::key_id_storage::MethodDigest;
  use identity_storage::key_storage::KeyId;

  use crate::tests::utils::create_verification_method;

  pub(crate) async fn test_storage_operations(storage: impl KeyIdStorage) {
    // Create a Verification Method.
    let verification_method = create_verification_method();

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
}
