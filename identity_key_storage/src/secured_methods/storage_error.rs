// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::identity_storage::IdentityStorageError;
use crate::key_storage::KeyStorageError;

/// An internal storage error container
#[derive(Debug)]
pub(super) enum StorageError {
  KeyStorage(KeyStorageError),
  IdentityStorage(IdentityStorageError),
  Both(Box<(KeyStorageError, IdentityStorageError)>),
}

impl StorageError {
  pub(super) fn key_storage_err(&self) -> Option<&KeyStorageError> {
    match self {
      Self::KeyStorage(ref err) => Some(err),
      Self::Both(ref errors) => Some(&errors.0),
      Self::IdentityStorage(_) => None,
    }
  }

  pub(super) fn into_key_storage_error(self) -> Option<KeyStorageError> {
    match self {
      Self::KeyStorage(err) => Some(err),
      Self::Both(errors) => Some(errors.0),
      Self::IdentityStorage(_) => None,
    }
  }
  pub(super) fn identity_storage_err(&self) -> Option<&IdentityStorageError> {
    match self {
      Self::IdentityStorage(ref err) => Some(err),
      Self::Both(ref errors) => Some(&errors.1),
      Self::KeyStorage(_) => None,
    }
  }

  pub(super) fn into_identity_storage_error(self) -> Option<IdentityStorageError> {
    match self {
      Self::IdentityStorage(err) => Some(err),
      Self::Both(errors) => Some(errors.1),
      Self::KeyStorage(_) => None,
    }
  }
}
