// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use common::get_client;
use serial_test::serial;

mod common;

mod direct_lookup {
  use identity_sui_name_tbd::migration;

  use super::*;

  #[serial]
  #[tokio::test]
  async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let alias_id = client.create_legacy_did().await?;

    let resolved_alias = migration::get_alias(&client, alias_id).await?;

    assert!(resolved_alias.is_some());

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let alias_id = client.create_legacy_did().await?;
    let (doc_id, _cap_id) = client.migrate_legacy_did(alias_id).await?;

    let resolved_id = migration::lookup(&client, alias_id)
      .await?
      .map(|doc| *doc.id.object_id())
      .unwrap();

    assert_eq!(resolved_id, doc_id);

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn resolving_migrated_documents_without_registry_does_not_work() -> anyhow::Result<()> {
    let client = get_client().await?;
    let alias_id = client.create_legacy_did().await?;
    let _ = client.migrate_legacy_did(alias_id).await?;

    assert!(migration::get_alias(&client, alias_id).await?.is_none());

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn new_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let object_id = client.create_did_document().await?;

    let document = migration::get_identity_document(&client, object_id).await?;

    assert!(document.is_some());

    Ok(())
  }
}

mod parallel_lookup {
  use identity_sui_name_tbd::resolution::get_did_document;

  use super::*;

  #[serial]
  #[tokio::test]
  async fn legacy_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let alias_id = client.create_legacy_did().await?;

    let result = get_did_document(&client, alias_id).await;

    assert!(result.is_ok());

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn migrated_legacy_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let alias_id = client.create_legacy_did().await?;
    let _ = client.migrate_legacy_did(alias_id).await?;

    let result = get_did_document(&client, alias_id).await;

    assert!(result.is_ok());

    Ok(())
  }

  #[serial]
  #[tokio::test]
  async fn new_did_document_resolution_works() -> anyhow::Result<()> {
    let client = get_client().await?;
    let object_id = client.create_did_document().await?;

    let result = get_did_document(&client, object_id).await;

    assert!(result.is_ok());

    Ok(())
  }
}
