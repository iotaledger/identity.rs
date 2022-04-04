// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_iota_core::did::IotaDID;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;
use iota_stronghold::Location;

use crate::storage::test_util::random_did;
use crate::storage::test_util::random_key_location;
use crate::storage::test_util::random_password;
use crate::storage::test_util::random_temporary_path;
use crate::stronghold::ClientPath;
use crate::stronghold::Stronghold;
use crate::types::KeyLocation;

async fn test_stronghold() -> Stronghold {
  Stronghold::new(&random_temporary_path(), random_password(), None)
    .await
    .unwrap()
}

#[tokio::test]
async fn test_mutate_client_persists_client_into_snapshot() {
  let path: String = random_temporary_path();
  let password: String = random_password();

  let stronghold: Stronghold = Stronghold::new(&path, password.clone(), None).await.unwrap();

  let did: IotaDID = random_did();
  let location: &KeyLocation = &random_key_location();

  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();

  stronghold
    .mutate_client(&did, |client| async move {
      let vault: ClientVault = client.vault(Location::generic(b"vault".to_vec(), Vec::new()));

      vault
        .write_secret(location.into(), keypair.private().as_ref().to_vec())
        .unwrap();

      Ok(())
    })
    .await
    .unwrap();

  let client: Client = stronghold.client(&ClientPath::from(&did)).await.unwrap();
  assert!(client.record_exists(location.into()).await.unwrap());

  stronghold.persist_snapshot().await.unwrap();

  let stronghold: Stronghold = Stronghold::new(&path, password, None).await.unwrap();

  let client: Client = stronghold.client(&ClientPath::from(&did)).await.unwrap();
  assert!(client.record_exists(location.into()).await.unwrap());
}

#[tokio::test]
async fn test_stronghold_did_create() {
  let stronghold: Stronghold = test_stronghold().await;

  crate::storage::tests::storage_did_create_test(Box::new(stronghold))
    .await
    .unwrap();
}

#[tokio::test]
async fn test_stronghold_key_generate() {
  let stronghold: Stronghold = test_stronghold().await;

  crate::storage::tests::storage_key_generate_test(Box::new(stronghold))
    .await
    .unwrap();
}

#[tokio::test]
async fn test_stronghold_key_delete() {
  let stronghold: Stronghold = test_stronghold().await;

  crate::storage::tests::storage_key_delete_test(Box::new(stronghold))
    .await
    .unwrap();
}
