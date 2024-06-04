// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur in the identity_sui_name_tbd crate.

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// failed to connect to network
  #[error("failed to connect to sui network node; {0:?}")]
  Network(String, #[source] sui_sdk::error::Error),
  /// could not lookup an object ID
  #[error("failed to lookup an object; {0}")]
  ObjectLookup(String),
  /// MigrationRegistry error.
  #[error(transparent)]
  MigrationRegistryNotFound(crate::migration::Error),
  /// Caused by a look failures during resolution.
  #[error("DID resolution failed: {0}")]
  DIDResolutionErrorKinesis(String),
}
