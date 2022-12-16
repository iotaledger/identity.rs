// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct MethodRemovalError {
  kind: MethodRemovalErrorKind,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl MethodRemovalError {
  pub const fn kind(&self) -> &MethodRemovalErrorKind {
    &self.kind
  }
}

impl Display for MethodRemovalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind().as_str())
  }
}

impl Error for MethodRemovalError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    (&self.source).as_deref().map(crate::error_utils::cast)
  }
}

#[derive(Debug)]
pub enum MethodRemovalErrorKind {
  /// Caused by an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
  ///
  /// It is at the caller's discretion whether to retry or not, and how often.
  RetryableIOFailure,

  /// A method with the provided [`DIDUrl`](::identity_did::did::DIDUrl) was not found in the document.  
  MethodNotFound,

  /// The key corresponding to the given method could not be found in the key storage.
  KeyNotFound,

  /// Unable to find the necessary key metadata in the [`IdentityStorage`](crate::identity_storage::IdentityStorage).
  MethodMetadataNotFound,

  /// An attempt was made to authenticate with the key storage, but this operation did not succeed.
  KeyStorageAuthenticationFailure,

  /// Indicates that an attempt was made to authenticate with the identity storage, but this operation did not succeed.
  IdentityStorageAuthenticationFailure,

  /// The [`Storage`](crate::storage::Storage) is currently unavailable.
  ///
  /// This could error could be caused by either of its components. See
  /// [`KeyStorageErrorKind::UnavailableKeyStorage`](crate::key_storage::error::KeyStorageErrorKind::UnavailableKeyStorage),
  /// [`IdentityStorageErrorKind::UnavailableKeyStorage`](crate::identity_storage::error::IdentityStorageErrorKind::UnavailableIdentityStorage).
  UnavailableStorage,

  /// The [`Storage`](crate::storage::Storage) failed in an unspecified manner.
  UnspecifiedStorageFailure,
}

impl MethodRemovalErrorKind {
  pub const fn as_str(&self) -> &str {
    match self {
      Self::RetryableIOFailure => "method removal failed: unsuccessful I/O operation",
      Self::MethodNotFound => "method removal failed: method not found",
      Self::KeyNotFound => "method removal failed: could not find key in storage",
      Self::MethodMetadataNotFound => "method removal failed: method metadata not found",
      Self::UnavailableStorage => "method creation failed: the storage is currently unavailable",
      Self::UnspecifiedStorageFailure => "method creation failed: the storage failed in an unspecified manner",
      Self::KeyStorageAuthenticationFailure => "method creation failed: authentication with the key storage failed",
      Self::IdentityStorageAuthenticationFailure => {
        "method creation failed: authentication with the identity storage failed"
      }
    }
  }
}
