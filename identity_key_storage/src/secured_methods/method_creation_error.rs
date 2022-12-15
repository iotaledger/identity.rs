// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt::Display;
/// An error representing an unsuccessful attempt to create a method whose
/// key material is backed by a [`Storage`](crate::storage::Storage).
#[derive(Debug)]
pub struct MethodCreationError {
  kind: MethodCreationErrorKind,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl MethodCreationError {
  pub(crate) fn new(kind: MethodCreationErrorKind, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
    Self {
      kind,
      source: Some(source.into()),
    }
  }

  pub(crate) fn from_kind(kind: MethodCreationErrorKind) -> Self {
    Self { kind, source: None }
  }

  /// Get the [`MethodCreationErrorKind`] of the error.
  pub const fn kind(&self) -> &MethodCreationErrorKind {
    &self.kind
  }
}

impl Display for MethodCreationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind().as_str())
  }
}

impl Error for MethodCreationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    fn cast<'a>(error: &'a (dyn Error + Send + Sync + 'static)) -> &'a (dyn Error + 'static) {
      error
    }
    self.source.as_deref().map(cast)
  }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum MethodCreationErrorKind {
  /// The provided fragment is used by another resource in the DID Document.
  FragmentInUse,

  /// The provided fragment representation does not comply with the [specified syntax](https://www.w3.org/TR/did-core/#fragment).
  InvalidFragmentSyntax,

  /// The provided [`KeyStorage`] implementation does not support generating keys of the given form.
  UnsupportedMultikeySchema,

  /// Caused by an attempt to create a method
  /// whose metadata has already been persisted.
  ///
  /// This could be caused by the [`KeyStorage`] returning a previously
  /// generated key rather than generating a new one contrary to the prescribed behaviour.
  /// Using the same verification material in different verification methods goes against SSI principles.
  /// If you want to use the same verification material across different context consider [referring to a single verification method](https://www.w3.org/TR/did-core/#referring-to-verification-methods)
  /// containing the given verification material instead.
  MethodMetadataAlreadyStored,

  /// Caused by an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
  ///
  /// Returning this error signals to the caller that the operation may be retried with a chance of success.
  /// It is at the caller's discretion whether to retry or not, and how often.
  RetryableIOFailure,

  /// An attempt was made to authenticate with the key storage, but this operation did not succeed.
  KeyStorageAuthenticationFailure,

  /// Indicates that an attempt was made to authenticate with the identity storage, but this operation did not succeed.
  IdentityStorageAuthenticationFailure,

  /// The key storage is currently not available. See
  /// [`KeyStorageErrorKind::UnavailableKeyStorage`](crate::key_storage::error::KeyStorageErrorKind::UnavailableKeyStorage).
  UnavailableKeyStorage,
  /// The identity storage is currently not available. See
  /// [`IdentityStorageErrorKind::UnavailableKeyStorage`](crate::identity_storage::error::IdentityStorageErrorKind::UnavailableIdentityStorage).
  UnavailableIdentityStorage,

  /// The key storage failed in an unspecified manner.
  UnspecifiedKeyStorageFailure,

  /// The identity storage failed in an unspecified manner.
  UnspecifiedIdentityStorageFailure,

  /// A key was generated, but the necessary metadata could not be persisted in the [`IdentityStorage`],
  /// the follow up attempt to remove the generated key from storage did not succeed.
  // TODO: Do we want to communicate this?
  // TODO: Should the variant wrap the `KeyId` so users can try deleting the corresponding key
  // at a later point themselves?
  // TODO: What expectations do we have for `MethodCreationError::source()` whenever this variant is encountered?
  TransactionRollbackFailure,
}

impl MethodCreationErrorKind {
  pub const fn as_str(&self) -> &str {
    match self {
      Self::FragmentInUse => "method creation failed: fragment in use",
      Self::InvalidFragmentSyntax => "method creation failed: invalid fragment syntax",
      Self::UnsupportedMultikeySchema => {
        "method creation failed: the key storage does not support the provided multikey schema"
      }
      Self::MethodMetadataAlreadyStored => {
        "method creation failed: the metadata corresponding to this method already exists in the identity storage"
      }
      Self::RetryableIOFailure => "method creation failed: unsuccessful I/O operation",
      Self::KeyStorageAuthenticationFailure => "method creation failed: authentication with the key storage failed",
      Self::IdentityStorageAuthenticationFailure => {
        "method creation failed: authentication with the identity storage failed"
      }
      Self::UnavailableKeyStorage => "method creation failed: key storage unavailable",
      Self::UnavailableIdentityStorage => "method creation failed: identity storage unavailable",
      Self::UnspecifiedKeyStorageFailure => "method creation failed: key storage failed",
      Self::UnspecifiedIdentityStorageFailure => "method creation failed: identity storage failed",
      Self::TransactionRollbackFailure => {
        "method creation failed: unable to persist generated metadata: could not rollback key generation"
      }
    }
  }
}
