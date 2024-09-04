// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::get_client as get_test_client;
use crate::common::TEST_DOC;
use identity_sui_name_tbd::migration;
use identity_sui_name_tbd::transaction::Transaction;

#[tokio::test]
async fn can_create_an_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let result = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute(&identity_client)
    .await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_resolve_a_new_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let new_identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute(&identity_client)
    .await?;

  let identity = migration::get_identity(&identity_client, new_identity.id()).await?;

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
        .map(|identity| identity.id())
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
    use super::*;

    #[serial]
    #[tokio::test]
    async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let identity_client = test_client.new_user_client().await?;

      let alias_id = test_client.create_legacy_did().await?;

      let result = identity_client.get_identity(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let identity_client = test_client.new_user_client().await?;

      let alias_id = test_client.create_legacy_did().await?;
      let _ = test_client.migrate_legacy_did(alias_id).await?;

      let result = identity_client.get_identity(alias_id).await;

      assert!(result.is_ok());

      Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn new_identity_resolution_works() -> anyhow::Result<()> {
      let test_client = get_test_client().await?;
      let identity_client = test_client.new_user_client().await?;

      let object_id = test_client.create_identity().await?;

      let result = identity_client.get_identity(object_id).await;

      assert!(result.is_ok());

      Ok(())
    }
  }
}
