// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::X25519;
use identity_iota_core::did::IotaDID;
use iota_stronghold::procedures;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::Client;
use iota_stronghold::ClientVault;
use iota_stronghold::Location;

use crate::storage::stronghold::aead_decrypt;
use crate::storage::stronghold::aead_encrypt;
use crate::storage::stronghold::concat_kdf;
use crate::storage::stronghold::diffie_hellman;
use crate::storage::stronghold::generate_private_key;
use crate::storage::stronghold::insert_private_key;
use crate::storage::stronghold::random_location;
use crate::storage::stronghold::retrieve_public_key;
use crate::storage::Storage;
use crate::storage::StorageTestSuite;
use crate::stronghold::test_util::random_did;
use crate::stronghold::test_util::random_key_location;
use crate::stronghold::test_util::random_string;
use crate::stronghold::test_util::random_temporary_path;
use crate::stronghold::ClientPath;
use crate::stronghold::Stronghold;
use crate::types::AgreementInfo;
use crate::types::CekAlgorithm;
use crate::types::EncryptedData;
use crate::types::EncryptionAlgorithm;
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

#[tokio::test]
async fn test_ecdhes_encryption() {
  // The following test vector is taken from RFC 8037
  // https://datatracker.ietf.org/doc/html/rfc8037#appendix-A.6

  let client: Client = Client::default();

  // Sender keys
  let location = random_location(KeyType::X25519);
  generate_private_key(&client, &location).unwrap();
  let public_key: PublicKey = retrieve_public_key(&client, &location).unwrap();
  let public_key: [u8; X25519::PUBLIC_KEY_LENGTH] = public_key.as_ref().try_into().unwrap();

  let ephemeral_secret_location: KeyLocation = random_key_location();
  let ephemeral_secret: PrivateKey = hex::decode("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a")
    .unwrap()
    .try_into()
    .unwrap();
  insert_private_key(&client, ephemeral_secret, &ephemeral_secret_location).unwrap();
  let ephemeral_public_key: [u8; X25519::PUBLIC_KEY_LENGTH] =
    hex::decode("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a")
      .unwrap()
      .try_into()
      .unwrap();

  let plaintext = b"HelloWorld!";
  let associated_data = b"AssociatedData";
  let agreement: AgreementInfo = AgreementInfo::new(b"Alice".to_vec(), b"Bob".to_vec(), Vec::new(), Vec::new());
  let cek_algorithm: CekAlgorithm = CekAlgorithm::ECDH_ES(agreement.clone());
  let enc_algorithm: EncryptionAlgorithm = EncryptionAlgorithm::AES256GCM;

  // Sender
  let shared_secret_location: Location = diffie_hellman(&client, &ephemeral_secret_location, public_key)
    .await
    .unwrap();
  let concat_secret: Location = concat_kdf(
    &client,
    &enc_algorithm,
    cek_algorithm.name().to_owned(),
    &agreement,
    shared_secret_location,
  )
  .await
  .unwrap();
  let encrypted_data: EncryptedData = aead_encrypt(
    &client,
    &enc_algorithm,
    concat_secret,
    plaintext.to_vec(),
    associated_data.to_vec(),
    Vec::new(),
    ephemeral_public_key.to_vec(),
  )
  .await
  .unwrap();
  // Receiver
  let shared_secret_location: Location = diffie_hellman(&client, &location, ephemeral_public_key).await.unwrap();
  let concat_secret: Location = concat_kdf(
    &client,
    &enc_algorithm,
    cek_algorithm.name().to_owned(),
    &agreement,
    shared_secret_location,
  )
  .await
  .unwrap();
  let data: Vec<u8> = aead_decrypt(&client, &enc_algorithm, concat_secret, encrypted_data)
    .await
    .unwrap();
  assert_eq!(plaintext.to_vec(), data);
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

#[tokio::test]
async fn test_stronghold_encryption() {
  StorageTestSuite::encryption_test(test_stronghold().await, test_stronghold().await)
    .await
    .unwrap()
}
