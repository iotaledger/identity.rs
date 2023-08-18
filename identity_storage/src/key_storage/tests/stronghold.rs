// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use identity_verification::jwk::Jwk;
use identity_verification::jws::JwsAlgorithm;
use iota_sdk::client::Password;

use crate::key_storage::tests::utils::generate_ed25519;
use crate::key_storage::JwkStorage;
use crate::key_storage::KeyType;
use crate::test_utils::stronghold_test_utils::create_stronghold_secret_manager;
use crate::test_utils::stronghold_test_utils::create_temp_file;
use crate::StrongholdStorage;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;

use super::utils::test_generate_and_sign;
use super::utils::test_incompatible_key_alg;
use super::utils::test_incompatible_key_type;
use super::utils::test_insertion;
use super::utils::test_key_exists;

#[tokio::test]
async fn insert() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  let stronghold_storage = StrongholdStorage::new(stronghold_secret_manager);
  test_insertion(stronghold_storage).await;
}

#[tokio::test]
async fn incompatible_key_alg() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  let stronghold_storage = StrongholdStorage::new(stronghold_secret_manager);
  test_incompatible_key_alg(stronghold_storage).await;
}

#[tokio::test]
async fn incompatible_key_types() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  let stronghold_storage = StrongholdStorage::new(stronghold_secret_manager);
  test_incompatible_key_type(stronghold_storage).await;
}

#[tokio::test]
async fn generate_and_sign() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  let stronghold_storage = StrongholdStorage::new(stronghold_secret_manager);
  test_generate_and_sign(stronghold_storage).await;
}

#[tokio::test]
async fn key_exists() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  let stronghold_storage = StrongholdStorage::new(stronghold_secret_manager);
  test_key_exists(stronghold_storage).await;
}

// Tests the cases that require persisting to disk, generate, insert and delete.
#[tokio::test]
async fn write_to_disk() {
  const PASS: &str = "secure_password";
  let file: PathBuf = create_temp_file();
  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(file.clone())
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);

  let generate = stronghold_storage
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await
    .unwrap();
  let key_id = &generate.key_id;

  drop(stronghold_storage);

  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  let exists = stronghold_storage.exists(key_id).await.unwrap();
  assert!(exists);
  stronghold_storage.delete(key_id).await.unwrap();

  drop(stronghold_storage);

  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  let exists = stronghold_storage.exists(key_id).await.unwrap();
  assert!(!exists);

  let (private_key, public_key) = generate_ed25519();
  let mut jwk: Jwk = crate::key_storage::ed25519::encode_jwk(&private_key, &public_key);
  jwk.set_alg(JwsAlgorithm::EdDSA.name());
  let key_id = stronghold_storage.insert(jwk).await.unwrap();

  drop(stronghold_storage);

  let secret_manager = StrongholdSecretManager::builder()
    .password(Password::from(PASS.to_owned()))
    .build(&file)
    .unwrap();
  let stronghold_storage = StrongholdStorage::new(secret_manager);
  let exists = stronghold_storage.exists(&key_id).await.unwrap();
  assert!(exists);
}
