// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur in the identity_sui_name_tbd crate.

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  // because we'll most probably need them later anyway
  //   /// Caused by a failure to encode Rust types as JSON.
  //   #[error("failed to encode JSON")]
  //   EncodeJSON(#[source] serde_json::Error),
  //   /// Caused by a failure to decode Rust types from JSON.
  //   #[error("failed to decode JSON")]
  //   DecodeJSON(#[source] serde_json::Error),
  /// failed to connect to network
  #[error("failed to connect to sui network node; {0:?}")]
  Network(String, #[source] sui_sdk::error::Error),
  /// could not lookup an object ID
  #[error("failed to lookup an object; {0}")]
  ObjectLookup(String),
}
