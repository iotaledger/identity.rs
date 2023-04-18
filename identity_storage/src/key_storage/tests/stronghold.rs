// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::utils::fs::random_temporary_path;
use crate::Stronghold;

use super::utils::test_generate_and_sign;
use super::utils::test_incompatible_key_alg;
use super::utils::test_incompatible_key_type;
use super::utils::test_insertion;
use super::utils::test_key_exists;

#[tokio::test]
async fn insert() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_insertion(stronghold).await;
}

#[tokio::test]
async fn incompatible_key_alg() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_incompatible_key_alg(stronghold).await;
}

#[tokio::test]
async fn incompatible_key_types() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_incompatible_key_type(stronghold).await;
}

#[tokio::test]
async fn generate_and_sign() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_generate_and_sign(stronghold).await;
}

#[tokio::test]
async fn key_exists() {
  let path: String = random_temporary_path();
  let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
    .await
    .unwrap();
  test_key_exists(stronghold).await;
}
