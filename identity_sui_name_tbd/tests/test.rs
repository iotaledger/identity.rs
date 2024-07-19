// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use identity_sui_name_tbd::migration::get_alias;
use identity_sui_name_tbd::utils::get_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use iota_sdk_legacy::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk_legacy::crypto::keys::bip39::Mnemonic;

mod common;

const TEST_MNEMONIC: &str =
  "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

/// Creates a stronghold path in the temporary directory, whose exact location is OS-dependent.
pub fn stronghold_path() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push("001");
  file.set_extension("stronghold");
  file.to_owned()
}

// must be done and can only be done once to import test mnemonic
#[tokio::test]
#[ignore]
async fn can_import_the_test_mnemonic() -> anyhow::Result<()> {
  let stronghold_secret_manager = StrongholdSecretManager::builder()
    .password("secure_password".to_string())
    .build("test.stronghold")
    .expect("Failed to create temporary stronghold");

  stronghold_secret_manager
    .store_mnemonic(Mnemonic::from(TEST_MNEMONIC))
    .await?;

  Ok(())
}

#[tokio::test]
async fn can_initialize_new_client() -> anyhow::Result<()> {
  let result = get_client(LOCAL_NETWORK).await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_fetch_alias_output_by_object_id() -> anyhow::Result<()> {
  let test_client = common::get_client().await?;
  let alias_id = test_client.create_legacy_did().await?;
  let iota_client = get_client(LOCAL_NETWORK).await?;

  let result = get_alias(&iota_client, alias_id).await;

  assert!(result.is_ok());

  Ok(())
}
