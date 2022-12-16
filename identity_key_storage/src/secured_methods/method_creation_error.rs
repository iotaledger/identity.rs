// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt::Display;

use crate::identity_storage::IdentityStorageError;
use crate::key_storage::KeyStorageError;

use super::storage_error::StorageError;
/// An error representing an unsuccessful attempt to create a method whose
/// key material is backed by a [`Storage`](crate::storage::Storage).
#[derive(Debug)]
pub struct MethodCreationError {
  kind: MethodCreationErrorKind,
  storage_error: Option<StorageError>,
}

impl MethodCreationError {
  pub(super) fn new(kind: MethodCreationErrorKind, source: StorageError) -> Self {
    Self {
      kind,
      storage_error: Some(source),
    }
  }

  pub(super) fn from_kind(kind: MethodCreationErrorKind) -> Self {
    Self {
      kind,
      storage_error: None,
    }
  }

  /// Get the [`MethodCreationErrorKind`] of the error.
  pub const fn kind(&self) -> &MethodCreationErrorKind {
    &self.kind
  }

  /// Returns a reference to an underlying [`KeyStorageError`] if it was set.
  ///
  /// # Important
  ///
  /// Values of [Self::kind](Self::kind()) indicating the problem was caused by
  /// [`KeyStorage`](crate::key_storage::KeyStorage) do not necessarily imply the return of the `Some` variant unless
  /// this is explicitly documented.
  pub fn key_storage_error(&self) -> Option<&KeyStorageError> {
    (&self.storage_error).as_ref().and_then(StorageError::key_storage_err)
  }

  /// Converts the error into the underlying [`KeyStorageError`] if it was set.
  ///
  /// # Important
  ///
  /// Values of [Self::kind](Self::kind()) indicating the problem was caused by
  /// [`KeyStorage`](crate::key_storage::KeyStorage) do not necessarily imply the return of the `Some` variant unless
  /// explicitly documented.
  pub fn into_key_storage_Error(self) -> Option<KeyStorageError> {
    self.storage_error.and_then(StorageError::into_key_storage_error)
  }

  /// Returns a reference to an underlying [`IdentityStorageError`] if it was set.
  ///
  /// # Important
  ///
  /// Values of [Self::kind](Self::kind()) indicating the problem was caused by
  /// [`IdentityStorage`](crate::identity_storage::IdentityStorage) do not necessarily imply the return of the `Some`
  /// variant unless explicitly documented.
  pub fn identity_storage_error(&self) -> Option<&IdentityStorageError> {
    (&self.storage_error)
      .as_ref()
      .and_then(StorageError::identity_storage_err)
  }

  /// Converts the error into the underlying [`IdentityStorageError`] if it was set.
  ///
  /// # Important
  ///
  /// Values of [Self::kind](Self::kind()) indicating the problem was caused by
  /// [`IdentityStorage`](crate::identity_storage::IdentityStorage) do not necessarily imply the return of the `Some`
  /// variant unless explicitly documented.
  pub fn into_identity_storage_error(self) -> Option<IdentityStorageError> {
    self.storage_error.and_then(StorageError::into_identity_storage_error)
  }
}

impl Display for MethodCreationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind().as_str())
  }
}

impl Error for MethodCreationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    let Some(ref storage_error) = self.storage_error else {return None;};
    match storage_error {
      StorageError::KeyStorage(ref key_storage_err) => Some(key_storage_err as &dyn Error),
      StorageError::IdentityStorage(ref identity_storage_err) => Some(identity_storage_err as &dyn Error),
      StorageError::Both(ref both) => {
        // We define the IdentityStorageError as the source because that is the reason method creation did not succeed.
        let err: &IdentityStorageError = &both.1;
        Some(err as &dyn Error)
      }
    }
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
