// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_sui_name_tbd::resolution::UnmigratedResolver;
use identity_sui_name_tbd::resolution::LOCAL_NETWORK;

#[tokio::test]
async fn can_initialize_resolver_for_unmigrated_alias_outputs() -> anyhow::Result<()> {
  let result = UnmigratedResolver::new(LOCAL_NETWORK).await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_fetch_alias_output_by_object_id() -> anyhow::Result<()> {
  let resolver = UnmigratedResolver::new(LOCAL_NETWORK).await?;
  let result = resolver
    .get_alias_output("0x669c70a008a5e226813927a7b62a5029306d8f7e7366b0634ef6027b3dbda850")
    .await;

  dbg!(&result);
  assert!(result.is_ok());

  Ok(())
}
