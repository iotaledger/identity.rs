// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::common::get_client as get_test_client;
use identity_iota_core::rebased::migration;
use identity_iota_core::rebased::transaction::Transaction;
use identity_iota_core::IotaDocument;

#[tokio::test]
async fn can_create_an_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let identity = identity_client
    .create_identity(IotaDocument::new(identity_client.network()))
    .finish()
    .execute(&identity_client)
    .await?
    .output;

  let did = identity.deref().id();
  assert_eq!(did.network_str(), identity_client.network().as_ref());

  Ok(())
}

#[tokio::test]
async fn can_resolve_a_new_identity() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let new_identity = identity_client
    .create_identity(IotaDocument::new(identity_client.network()))
    .finish()
    .execute(&identity_client)
    .await?
    .output;

  let identity = migration::get_identity(&identity_client, new_identity.id()).await?;

  assert!(identity.is_some());

  Ok(())
}
