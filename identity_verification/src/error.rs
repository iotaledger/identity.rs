// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Decentralized Identifiers.

/// Alias for a [`Result`][::core::result::Result] with the error type [Error].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the crate.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused by invalid or missing properties when constructing a
  /// [`VerificationMethod`](crate::VerificationMethod).
  #[error("invalid verification method property: {0}")]
  InvalidMethod(&'static str),
  #[error("invalid DID url")]
  DIDUrlConstructionError(#[source] identity_did::Error),
  #[error("invalid or empty `id` fragment")]
  MissingIdFragment,
  #[error("Unknown Method Scope")]
  UnknownMethodScope,
  #[error("Unknown Method Type")]
  UnknownMethodType,
  #[error("Invalid Base58 Key Data")]
  InvalidKeyDataBase58,
  #[error("Invalid Multibase Key Data")]
  InvalidKeyDataMultibase,
  #[error("can only decode multibase verification material, but received publicKeyJwk")]
  InvalidDecodingRequest, 
}
