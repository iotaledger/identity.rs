// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::Category;

#[cfg(feature = "account")]
pub mod handler;
pub mod requests;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum StorageError {
  IdentityNotFound,
  IotaError(String),
  AccountError(String),
}

impl StorageError {
  pub fn classify(&self) -> Category {
    match self {
      Self::IdentityNotFound => Category::Client,
      _ => Category::Remote,
    }
  }
}

impl From<identity_iota::Error> for StorageError {
  fn from(err: identity_iota::Error) -> Self {
    Self::IotaError(err.to_string())
  }
}

impl From<identity_account::Error> for StorageError {
  fn from(err: identity_account::Error) -> Self {
    Self::AccountError(err.to_string())
  }
}
