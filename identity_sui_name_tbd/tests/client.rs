// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use common::get_client as get_test_client;
use common::request_funds;
use identity_storage::JwkMemStore;
use identity_storage::JwkStorage;
use identity_storage::KeyIdMemstore;
use identity_storage::KeyType;
use identity_storage::Storage;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::migration;
use identity_sui_name_tbd::utils::get_client as get_sui_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use identity_verification::jws::JwsAlgorithm;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

const TEST_DOC: &[u8] = &[
  68, 73, 68, 1, 0, 131, 1, 123, 34, 100, 111, 99, 34, 58, 123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58,
  48, 34, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91,
  123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58, 48, 35, 79, 115, 55, 95, 66, 100, 74, 120, 113, 86, 119,
  101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100, 108, 74, 73, 99, 48,
  45, 100, 50, 49, 73, 34, 44, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58, 48,
  58, 48, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 74, 115, 111, 110, 87, 101, 98, 75, 101, 121, 34, 44, 34, 112,
  117, 98, 108, 105, 99, 75, 101, 121, 74, 119, 107, 34, 58, 123, 34, 107, 116, 121, 34, 58, 34, 79, 75, 80, 34, 44,
  34, 97, 108, 103, 34, 58, 34, 69, 100, 68, 83, 65, 34, 44, 34, 107, 105, 100, 34, 58, 34, 79, 115, 55, 95, 66, 100,
  74, 120, 113, 86, 119, 101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100,
  108, 74, 73, 99, 48, 45, 100, 50, 49, 73, 34, 44, 34, 99, 114, 118, 34, 58, 34, 69, 100, 50, 53, 53, 49, 57, 34, 44,
  34, 120, 34, 58, 34, 75, 119, 99, 54, 89, 105, 121, 121, 65, 71, 79, 103, 95, 80, 116, 118, 50, 95, 49, 67, 80, 71,
  52, 98, 86, 87, 54, 102, 89, 76, 80, 83, 108, 115, 57, 112, 122, 122, 99, 78, 67, 67, 77, 34, 125, 125, 93, 125, 44,
  34, 109, 101, 116, 97, 34, 58, 123, 34, 99, 114, 101, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50, 52, 45, 48, 53, 45,
  50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 44, 34, 117, 112, 100, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50,
  52, 45, 48, 53, 45, 50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 125, 125,
];
const TEST_GAS_BUDGET: u64 = 50_000_000;

#[tokio::test]
async fn can_publish_a_did_document() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let sui_client = get_sui_client(LOCAL_NETWORK).await?;
  let storage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  // generate new key
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key = generate.jwk.to_public().expect("public components should be derivable");

  let identity_client = IdentityClient::builder()
    .identity_iota_package_id(test_client.package_id())
    .sender_key_id(generate.key_id)
    .sender_public_jwk(public_key)
    .storage(storage)
    .sui_client(sui_client)
    .build()?;

  // call faucet with out new account
  request_funds(&identity_client.sender_address()?).await?;

  let result = identity_client.publish_did(TEST_DOC, TEST_GAS_BUDGET).await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_resolve_a_new_did_document() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let sui_client = get_sui_client(LOCAL_NETWORK).await?;
  let storage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  // generate new key
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key = generate.jwk.to_public().expect("public components should be derivable");

  let identity_client = IdentityClient::builder()
    .identity_iota_package_id(test_client.package_id())
    .sender_key_id(generate.key_id)
    .sender_public_jwk(public_key)
    .storage(storage)
    .sui_client(sui_client)
    .build()?;

  // call faucet with out new account
  request_funds(&identity_client.sender_address()?).await?;

  let object_id = identity_client.publish_did(TEST_DOC, TEST_GAS_BUDGET).await?;

  let sui_client = get_sui_client(LOCAL_NETWORK).await?;
  let document = migration::get_identity_document(&sui_client, object_id).await?;

  assert!(document.is_some());

  Ok(())
}

mod resolution {
  use serial_test::serial;

  use super::*;

  mod direct_lookup {
    use identity_sui_name_tbd::migration;

    use super::*;

    #[serial]
    #[tokio::test]
    async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let alias_id = test_client.create_legacy_did().await?;

      let resolved_alias = migration::get_alias(&test_client, alias_id).await?;

      assert!(resolved_alias.is_some());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let alias_id = test_client.create_legacy_did().await?;
      let (doc_id, _cap_id) = test_client.migrate_legacy_did(alias_id).await?;

      let resolved_id = migration::lookup(&test_client, alias_id)
        .await?
        .map(|doc| *doc.id.object_id())
        .unwrap();

      assert_eq!(resolved_id, doc_id);

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn resolving_migrated_documents_without_registry_does_not_work() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let alias_id = test_client.create_legacy_did().await?;
      let _ = test_client.migrate_legacy_did(alias_id).await?;

      assert!(migration::get_alias(&test_client, alias_id).await?.is_none());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn new_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let object_id = test_client.create_did_document().await?;

      let document = migration::get_identity_document(&test_client, object_id).await?;

      assert!(document.is_some());

      Ok(())
    }
  }

  mod parallel_lookup {
    use identity_sui_name_tbd::client::IdentityClient;

    use super::*;

    #[serial]
    #[tokio::test]
    async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let sui_client = get_sui_client(LOCAL_NETWORK).await?;
      let alias_id = test_client.create_legacy_did().await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .sui_client(sui_client)
        .build()?;

      let result = identity_client.get_did_document(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let sui_client = get_sui_client(LOCAL_NETWORK).await?;
      let alias_id = test_client.create_legacy_did().await?;
      let _ = test_client.migrate_legacy_did(alias_id).await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .sui_client(sui_client)
        .build()?;

      let result = identity_client.get_did_document(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn new_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let sui_client = get_sui_client(LOCAL_NETWORK).await?;
      let object_id = test_client.create_did_document().await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .sui_client(sui_client)
        .build()?;

      let result = identity_client.get_did_document(object_id).await;

      assert!(result.is_ok());

      Ok(())
    }
  }
}
