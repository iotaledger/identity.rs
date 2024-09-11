// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::get_client;
use crate::common::{self};
use identity_sui_name_tbd::migration::get_alias;

#[tokio::test]
async fn can_initialize_new_client() -> anyhow::Result<()> {
  let result = get_client().await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_fetch_alias_output_by_object_id() -> anyhow::Result<()> {
  let test_client = common::get_client().await?;
  let alias_id = test_client.create_legacy_did().await?;
  let iota_client = get_client().await?;

  let result = get_alias(&iota_client, alias_id).await;

  assert!(result.is_ok());

  Ok(())
}
