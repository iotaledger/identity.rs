// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Verifiable Credentials.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents errors that can occur when constructing credentials and presentations.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused when constructing a credential or presentation without a valid base context.
  #[error("missing base context")]
  MissingBaseContext,
  /// Caused when constructing a credential or presentation without a valid base type.
  #[error("missing base type")]
  MissingBaseType,
  /// Caused when constructing a credential without an issuer.
  #[error("missing credential issuer")]
  MissingIssuer,
  /// Caused when constructing a credential without a subject.
  #[error("missing credential subject")]
  MissingSubject,
  /// Caused when constructing a Domain Linkage credential without an expiration date.
  #[error("missing expiration date")]
  MissingExpirationDate,
  /// Caused when constructing a Domain Linkage credential without an origin.
  #[error("missing origin")]
  MissingOrigin,
  /// Caused when constructing a credential with a malformed subject.
  #[error("invalid credential subject")]
  InvalidSubject,
  /// Caused when trying to construct an invalid status.
  #[error("invalid credential status: {0}")]
  InvalidStatus(String),
  /// Caused when constructing an invalid `LinkedDomainService` or `DomainLinkageConfiguration`.
  #[error("domain linkage error")]
  DomainLinkageError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
