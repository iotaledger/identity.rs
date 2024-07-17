// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod common;

use common::get_client as get_test_client;
use common::request_funds;
use common::TEST_DOC;
use common::TEST_GAS_BUDGET;
use identity_storage::JwkMemStore;
use identity_storage::JwkStorage;
use identity_storage::KeyIdMemstore;
use identity_storage::KeyType;
use identity_storage::Storage;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::migration;
use identity_sui_name_tbd::utils::get_client as get_iota_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use identity_verification::jws::JwsAlgorithm;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

#[tokio::test]
async fn can_create_an_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let iota_client = get_iota_client(LOCAL_NETWORK).await?;
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
    .iota_client(iota_client)
    .build()?;

  // call faucet with out new account
  request_funds(&identity_client.sender_address()?).await?;

  let result = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_resolve_a_new_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let iota_client = get_iota_client(LOCAL_NETWORK).await?;
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
    .iota_client(iota_client)
    .build()?;

  // call faucet with out new account
  request_funds(&identity_client.sender_address()?).await?;

  let newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await?;

  let iota_client = get_iota_client(LOCAL_NETWORK).await?;
  let identity = migration::get_identity(&iota_client, *newly_created_identity.id.object_id()).await?;

  assert!(identity.is_some());

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
    async fn new_identity_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let object_id = test_client.create_identity().await?;

      let document = migration::get_identity(&test_client, object_id).await?;

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
      let iota_client = get_iota_client(LOCAL_NETWORK).await?;
      let alias_id = test_client.create_legacy_did().await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .iota_client(iota_client)
        .build()?;

      let result = identity_client.get_identity(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let iota_client = get_iota_client(LOCAL_NETWORK).await?;
      let alias_id = test_client.create_legacy_did().await?;
      let _ = test_client.migrate_legacy_did(alias_id).await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .iota_client(iota_client)
        .build()?;

      let result = identity_client.get_identity(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn new_identity_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let iota_client = get_iota_client(LOCAL_NETWORK).await?;
      let object_id = test_client.create_identity().await?;
      let identity_client: IdentityClient<JwkMemStore, KeyIdMemstore> = IdentityClient::builder()
        .identity_iota_package_id(test_client.package_id())
        .iota_client(iota_client)
        .build()?;

      let result = identity_client.get_identity(object_id).await;

      assert!(result.is_ok());

      Ok(())
    }
  }
}
