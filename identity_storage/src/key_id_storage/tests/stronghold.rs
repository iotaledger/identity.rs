// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Stronghold;

use crate::key_id_storage::tests::utils::test_storage_operations;

#[tokio::test]
pub async fn test_stronghold() {
  let stronghold: Stronghold = Stronghold::new("stronghold.hodl", "test-password".to_owned(), Some(false), None)
    .await
    .unwrap();
  test_storage_operations(stronghold).await;
}
