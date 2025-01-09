// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur for the rebased logic.

#[cfg(target_arch = "wasm32")]
use iota_interaction_ts::error::TsSdkError;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// failed to connect to network.
  #[error("failed to connect to iota network node; {0:?}")]
  Network(String, #[source] identity_iota_interaction::error::Error),
  /// could not lookup an object ID.
  #[error("failed to lookup an object; {0}")]
  ObjectLookup(String),
  /// MigrationRegistry error.
  #[error(transparent)]
  MigrationRegistryNotFound(crate::rebased::migration::Error),
  /// Caused by a look failures during resolution.
  #[error("DID resolution failed: {0}")]
  DIDResolutionError(String),
  /// Caused by invalid or missing arguments.
  #[error("invalid or missing argument: {0}")]
  InvalidArgument(String),
  /// Caused by invalid keys.
  #[error("invalid key: {0}")]
  InvalidKey(String),
  /// Caused by issues with paying for transaction.
  #[error("issue with gas for transaction: {0}")]
  GasIssue(String),
  /// Could not parse module, package, etc.
  #[error("failed to parse {0}")]
  ParsingFailed(String),
  /// Could not build transaction.
  #[error("failed to build transaction; {0}")]
  TransactionBuildingFailed(String),
  /// Could not sign transaction.
  #[error("failed to sign transaction; {0}")]
  TransactionSigningFailed(String),
  /// Could not execute transaction.
  #[error("transaction execution failed; {0}")]
  TransactionExecutionFailed(#[from] identity_iota_interaction::error::Error),
  /// Transaction yielded invalid response. This usually means that the transaction was executed but did not produce
  /// the expected result.
  #[error("transaction returned an unexpected response; {0}")]
  TransactionUnexpectedResponse(String),
  /// Config is invalid.
  #[error("invalid config: {0}")]
  InvalidConfig(String),
  /// Failed to parse DID document.
  #[error("failed to parse DID document; {0}")]
  DidDocParsingFailed(String),
  /// Failed to serialize DID document.
  #[error("failed to serialize DID document; {0}")]
  DidDocSerialization(String),
  /// Identity related error.
  #[error("identity error; {0}")]
  Identity(String),
  #[error("unexpected state when looking up identity history; {0}")]
  /// Unexpected state when looking up identity history.
  InvalidIdentityHistory(String),
  /// An operation cannot be carried on for a lack of permissions - e.g. missing capability.
  #[error("the requested operation cannot be performed for a lack of permissions; {0}")]
  MissingPermission(String),
  /// An error caused by either a connection issue or an invalid RPC call.
  #[error("RPC error: {0}")]
  RpcError(String),
  /// An error caused by a bcs serialization or deserialization.
  #[error("BCS error: {0}")]
  BcsError(#[from] bcs::Error),
  /// An anyhow::error.
  #[error("Any error: {0}")]
  AnyError(#[from] anyhow::Error),
  #[cfg(target_arch = "wasm32")]
  /// An error originating from IOTA typescript SDK import bindings
  #[error("TsSdkError: {0}")]
  TsSdkError(#[from] TsSdkError),
}

/// Can be used for example like `map_err(rebased_err)` to convert other error
///  types to identity_iota_core::rebased::Error.
pub fn rebased_err<T>(error: T) -> Error
where
  Error: From<T>,
{
  error.into()
}
