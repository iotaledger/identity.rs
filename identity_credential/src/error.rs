// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Verifiable Credentials.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused when validating a Credential without a valid base context.
  #[error("Missing Base Context")]
  MissingBaseContext,
  /// Caused when validating a Credential without a valid base type.
  #[error("Missing Base Type")]
  MissingBaseType,
  /// Caused when validating a Credential without an issuer.
  #[error("Missing Credential Issuer")]
  MissingIssuer,
  /// Caused when validating a Credential without a subject.
  #[error("Missing Credential Subject")]
  MissingSubject,
  /// Caused when validating a Credential with a malformed subject.
  #[error("Invalid Credential Subject")]
  InvalidSubject,
  /// Caused when trying to construct an invalid status.
  #[error("invalid credential status: {0}")]
  InvalidStatus(String),
}
