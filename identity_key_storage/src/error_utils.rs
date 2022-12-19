// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;

pub(crate) fn cast<'a>(error: &'a (dyn Error + Send + Sync + 'static)) -> &'a (dyn Error + 'static) {
  error
}

pub(crate) trait AsDynError {
  fn as_dyn_err(&self) -> Option<&(dyn Error + 'static)>;
}

impl AsDynError for Option<Box<dyn Error + Send + Sync + 'static>> {
  fn as_dyn_err(&self) -> Option<&(dyn Error + 'static)> {
    self.as_deref().map(cast)
  }
}

/// Variants the associated error kinds of errors returned by CoreDocumentExt have in common, that originate from
/// [KeyStorageErrorKind](crate::key_storage::KeyStorageErrorKind) or
/// [IdentityStorageErrorKind](crate::identity_storage::IdentityStorageErrorKind).
pub(crate) enum CommonErrorKindVariants {
  /// Caused by an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
  ///
  /// It is at the caller's discretion whether to retry or not, and how often.
  RetryableIOFailure,

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

macro_rules! impl_from_common_error_kind_variants {
  ($t:ty) => {
    impl From<crate::error_utils::CommonErrorKindVariants> for $t {
      fn from(kind: crate::error_utils::CommonErrorKindVariants) -> $t {
        match kind {
          crate::error_utils::CommonErrorKindVariants::RetryableIOFailure => <$t>::RetryableIOFailure,
          crate::error_utils::CommonErrorKindVariants::KeyStorageAuthenticationFailure => {
            <$t>::KeyStorageAuthenticationFailure
          }
          crate::error_utils::CommonErrorKindVariants::IdentityStorageAuthenticationFailure => {
            <$t>::IdentityStorageAuthenticationFailure
          }
          crate::error_utils::CommonErrorKindVariants::UnavailableStorage => <$t>::UnavailableStorage,
          crate::error_utils::CommonErrorKindVariants::UnspecifiedStorageFailure => <$t>::UnspecifiedStorageFailure,
        }
      }
    }
  };
}
pub(crate) use impl_from_common_error_kind_variants;
