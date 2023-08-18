// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::utils::test_generate_and_sign;
use super::utils::test_incompatible_key_alg;
use super::utils::test_incompatible_key_type;
use super::utils::test_insertion;
use super::utils::test_key_exists;
use crate::key_storage::JwkMemStore;

#[tokio::test]
async fn insert() {
  let store: JwkMemStore = JwkMemStore::new();
  test_insertion(store).await;
}

#[tokio::test]
async fn incompatible_key_alg() {
  let store: JwkMemStore = JwkMemStore::new();
  test_incompatible_key_alg(store).await;
}

#[tokio::test]
async fn incompatible_key_types() {
  let store: JwkMemStore = JwkMemStore::new();
  test_incompatible_key_type(store).await;
}

#[tokio::test]
async fn generate_and_sign() {
  let store: JwkMemStore = JwkMemStore::new();
  test_generate_and_sign(store).await;
}

#[tokio::test]
async fn key_exists() {
  let store: JwkMemStore = JwkMemStore::new();
  test_key_exists(store).await;
}
