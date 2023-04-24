// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::utils::test_utils::create_stronghold_secret_manager;

use super::utils::test_generate_and_sign;
use super::utils::test_incompatible_key_alg;
use super::utils::test_incompatible_key_type;
use super::utils::test_insertion;
use super::utils::test_key_exists;

#[tokio::test]
async fn insert() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_insertion(stronghold_secret_manager).await;
}

#[tokio::test]
async fn incompatible_key_alg() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_incompatible_key_alg(stronghold_secret_manager).await;
}

#[tokio::test]
async fn incompatible_key_types() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_incompatible_key_type(stronghold_secret_manager).await;
}

#[tokio::test]
async fn generate_and_sign() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_generate_and_sign(stronghold_secret_manager).await;
}

#[tokio::test]
async fn key_exists() {
  let stronghold_secret_manager = create_stronghold_secret_manager();
  test_key_exists(stronghold_secret_manager).await;
}
