// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::get_client as get_test_client;
use crate::common::TEST_DOC;
use identity_iota_core::rebased::migration;
use identity_iota_core::rebased::transaction::Transaction;

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
    .await?
    .output;

  let identity = migration::get_identity(&identity_client, new_identity.id()).await?;

  assert!(identity.is_some());

  Ok(())
}