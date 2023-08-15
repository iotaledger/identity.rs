// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::utils::test_storage_operations;
use crate::key_id_storage::method_digest::MethodDigest;
use crate::key_id_storage::KeyIdStorage;
use crate::key_id_storage::KeyIdStorageErrorKind;
use crate::key_storage::KeyId;
use crate::storage::tests::test_utils::create_verification_method;
use crate::test_utils::stronghold_test_utils::create_temp_file;
use crate::StrongholdStorage;
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
  test_storage_operations(stronghold_storage).await;
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
