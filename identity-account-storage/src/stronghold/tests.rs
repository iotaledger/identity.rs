// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_iota_core::did::IotaDID;
use iota_stronghold::procedures;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;

use crate::storage::Storage;
use crate::storage::StorageTestSuite;
use crate::stronghold::test_util::random_did;
use crate::stronghold::test_util::random_key_location;
use crate::stronghold::test_util::random_string;
use crate::stronghold::test_util::random_temporary_path;
use crate::stronghold::ClientPath;
use crate::stronghold::Stronghold;
use crate::types::KeyLocation;

#[tokio::test]
async fn test_mutate_client_persists_client_into_snapshot() {
  let path: String = random_temporary_path();
  let password: String = random_string();

  let stronghold: Stronghold = Stronghold::new(&path, password.clone(), Some(true)).await.unwrap();

  let did: IotaDID = random_did();
  let location: &KeyLocation = &random_key_location();

  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();

  stronghold
    .mutate_client(&did, |client| {
      let vault: ClientVault = client.vault(b"vault");

      vault
        .write_secret(location.into(), keypair.private().as_ref().to_vec())
        .unwrap();

      Ok(())
    })
    .unwrap();

  let client: Client = stronghold.client(&ClientPath::from(&did)).unwrap();
  assert!(client.record_exists(&location.into()).unwrap());

  // Persists the snapshot, because dropsave = true.
  std::mem::drop(stronghold);

  let stronghold: Stronghold = Stronghold::new(&path, password, Some(false)).await.unwrap();

  let client: Client = stronghold.client(&ClientPath::from(&did)).unwrap();
  assert!(client.record_exists(&location.into()).unwrap());
}

#[tokio::test]
async fn test_incorrect_password_returns_error() {
  let path: String = random_temporary_path();
  let password: String = random_string();

  let stronghold: Stronghold = Stronghold::new(&path, password, Some(false)).await.unwrap();

  let did: IotaDID = random_did();
  let location: &KeyLocation = &random_key_location();

  stronghold
    .mutate_client(&did, |client| {
      client
        .execute_procedure(GenerateKey {
          ty: procedures::KeyType::Ed25519,
          output: location.into(),
        })
        .unwrap();

      Ok(())
    })
    .unwrap();

  stronghold.persist_snapshot().await.unwrap();
  std::mem::drop(stronghold);

  let err = Stronghold::new(&path, "not-the-original-password".to_owned(), Some(false))
    .await
    .unwrap_err();

  assert!(matches!(
    err,
    crate::error::Error::StrongholdError(crate::stronghold::error::StrongholdError::Snapshot(
      crate::stronghold::error::SnapshotOperation::Read,
      _,
      _
    ))
  ));
}

async fn test_stronghold() -> impl Storage {
  Stronghold::new(&random_temporary_path(), random_string(), Some(false))
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_did_create_with_private_key() {
  StorageTestSuite::did_create_private_key_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_did_create_generate_key() {
  StorageTestSuite::did_create_generate_key_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_key_generate() {
  StorageTestSuite::key_generate_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_key_delete() {
  StorageTestSuite::key_delete_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_did_list() {
  StorageTestSuite::did_list_test(test_stronghold().await).await.unwrap()
}

#[tokio::test]
async fn test_stronghold_key_insert() {
  StorageTestSuite::key_insert_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_key_sign_ed25519() {
  StorageTestSuite::key_sign_ed25519_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_key_value_store() {
  StorageTestSuite::key_value_store_test(test_stronghold().await)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_stronghold_did_purge() {
  StorageTestSuite::did_purge_test(test_stronghold().await).await.unwrap()
}
