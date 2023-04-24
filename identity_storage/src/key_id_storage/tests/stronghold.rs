// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::utils::test_utils::create_stronghold_secret_manager;

use super::utils::test_storage_operations;

#[tokio::test]
pub async fn test_stronghold() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_storage_operations(stronghold_secret_manager).await;
}
