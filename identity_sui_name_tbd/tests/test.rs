// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod common;

use std::path::PathBuf;

use common::get_client;
use identity_sui_name_tbd::migration::get_alias;

/// Creates a stronghold path in the temporary directory, whose exact location is OS-dependent.
pub fn stronghold_path() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push("001");
  file.set_extension("stronghold");
  file.to_owned()
}

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
