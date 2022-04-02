// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_iota_core::did::IotaDID;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;
use iota_stronghold::Location;
use rand::Rng;

use crate::storage::Storage;
use crate::types::KeyLocation;

use super::ClientPath;
use super::Stronghold;

fn random_temporary_path() -> String {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(
    rand::thread_rng()
      .sample_iter(rand::distributions::Alphanumeric)
      .take(32)
      .map(char::from)
      .collect::<String>(),
  );
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}

fn random_password() -> String {
  rand::thread_rng()
    .sample_iter(rand::distributions::Alphanumeric)
    .take(32)
    .map(char::from)
    .collect::<String>()
}

// async fn test_stronghold() -> Stronghold {
//   Stronghold::new(&random_temporary_path(), random_password(), None)
//     .await
//     .unwrap()
// }

fn random_did() -> IotaDID {
  let public_key: [u8; 32] = rand::thread_rng().gen();
  IotaDID::new(&public_key).unwrap()
}

fn random_key_location() -> KeyLocation {
  let mut thread_rng: rand::rngs::ThreadRng = rand::thread_rng();
  let fragment: String = rand::Rng::sample_iter(&mut thread_rng, rand::distributions::Alphanumeric)
    .take(32)
    .map(char::from)
    .collect();
  let public_key: [u8; 32] = rand::Rng::gen(&mut thread_rng);

  KeyLocation::new(KeyType::Ed25519, fragment, &public_key)
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
async fn test_key_delete() {
  let path: String = random_temporary_path();
  let password: String = random_password();

  let stronghold: Stronghold = Stronghold::new(&path, password.clone(), None).await.unwrap();

  let did: IotaDID = random_did();
  let location: &KeyLocation = &random_key_location();

  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();

  stronghold
    .mutate_client(&did, |client| async move {
      let vault: ClientVault = client.vault(location.into());

      vault
        .write_secret(location.into(), keypair.private().as_ref().to_vec())
        .unwrap();

      let exists: bool = client.record_exists(location.into()).await.unwrap();
      assert!(exists);

      Ok(())
    })
    .await
    .unwrap();

  // Running it once removes the record.
  assert!(stronghold.key_delete(&did, location).await.unwrap());

  // Running it a second time does not fail, but returns false.
  assert!(!stronghold.key_delete(&did, location).await.unwrap());
}
