// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::memstore::KeyIdMemstore;

use crate::key_id_storage::tests::utils::test_storage_operations;

#[tokio::test]
async fn test_memstore() {
  let memstore: KeyIdMemstore = KeyIdMemstore::new();
  test_storage_operations(memstore).await;
}
