// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Stronghold;

use crate::key_id_storage::tests::utils::test_storage_operations;
use crate::utils::fs::random_temporary_path;

#[tokio::test]
pub async fn test_stronghold() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_storage_operations(stronghold).await;
}
