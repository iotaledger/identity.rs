// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::get_client;
use identity_sui_name_tbd::migration;
use iota_sdk::types::base_types::ObjectID;

#[tokio::test]
async fn migration_registry_is_found() -> anyhow::Result<()> {
  let client = get_client().await?;
  let random_alias_id = ObjectID::random();

  let doc = migration::lookup(&client, random_alias_id).await?;
  assert!(doc.is_none());

  Ok(())
}

#[tokio::test]
async fn migration_of_legacy_did_works() -> anyhow::Result<()> {
  let client = get_client().await?;
  let alias_id = client.create_legacy_did().await?;

  let (doc_id, _) = client.migrate_legacy_did(alias_id).await?;
  let resolved_id = migration::lookup(&client, alias_id)
    .await?
    .map(|identity| identity.id())
    .unwrap();

  assert_eq!(resolved_id, doc_id);

  Ok(())
}
