// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

use crate::error_utils::AsDynError;

// TODO: This follows the same pattern as KeyStorageError. Might be an idea to make a macro_rules macro for this sort of
// thing.

/// The error type for IdentityStorage operations.
///
/// Instances always carry a corresponding [`StorageErrorKind`] and may be extended with custom error messages and
/// source.
#[derive(Debug)]
pub struct IdentityStorageError {
  repr: Repr,
}

impl Display for IdentityStorageError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.repr {
      Repr::Simple(ref cause) => write!(f, "{}", cause.as_str()),
      Repr::Extensive(ref extensive) => {
        write!(f, "{}", extensive.cause.as_str())?;
        let Some(ref message) = extensive.message else {return Ok(())};
        write!(f, " message: {}", message.as_ref())
      }
    }
  }
}

impl Error for IdentityStorageError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.source_as_dyn()
  }
}
#[derive(Debug)]
struct Extensive {
  cause: IdentityStorageErrorKind,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
  message: Option<Cow<'static, str>>,
}

#[derive(Debug)]
enum Repr {
  Simple(IdentityStorageErrorKind),
  Extensive(Box<Extensive>),
}

impl From<IdentityStorageErrorKind> for IdentityStorageError {
  fn from(cause: IdentityStorageErrorKind) -> Self {
    Self::new(cause)
  }
}

impl From<Box<Extensive>> for IdentityStorageError {
  fn from(extensive: Box<Extensive>) -> Self {
    Self {
      repr: Repr::Extensive(extensive),
    }
  }
}

impl IdentityStorageError {
  /// Constructs a new [`IdentityStorageError`].  
  pub fn new(cause: IdentityStorageErrorKind) -> Self {
    Self {
      repr: Repr::Simple(cause),
    }
  }

  /// Returns a reference to the corresponding [`IdentityStorageErrorKind`] of this error.
  pub fn kind(&self) -> &IdentityStorageErrorKind {
    match self.repr {
      Repr::Simple(ref cause) => cause,
      Repr::Extensive(ref extensive) => &extensive.cause,
    }
  }

  /// Converts this error into the corresponding [`IdentityStorageErrorKind`].
  pub fn into_kind(self) -> IdentityStorageErrorKind {
    match self.repr {
      Repr::Simple(cause) => cause,
      Repr::Extensive(extensive) => extensive.cause,
    }
  }

  /// Returns a reference to the custom message of the [`IdentityStorageError`] if it was set.
  pub fn custom_message(&self) -> Option<&str> {
    self
      .extensive()
      .into_iter()
      .flat_map(|extensive| extensive.message.as_deref())
      .next()
  }

  /// Returns a reference to the attached source of the [`IdentityStorageError`] if it was set.
  pub fn source_ref(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
    self.extensive().and_then(|extensive| extensive.source.as_deref())
  }

  fn source_as_dyn(&self) -> Option<&(dyn Error + 'static)> {
    self.extensive().and_then(|extensive| extensive.source.as_dyn_err())
  }

  /// Converts this error into the source error if it was set.
  pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync + 'static>> {
    self.into_extensive().source
  }

  fn extensive(&self) -> Option<&Extensive> {
    match self.repr {
      Repr::Extensive(ref extensive) => Some(extensive.as_ref()),
      _ => None,
    }
  }

  fn into_extensive(self) -> Box<Extensive> {
    match self.repr {
      Repr::Extensive(extensive) => extensive,
      Repr::Simple(cause) => Box::new(Extensive {
        cause,
        source: None,
        message: None,
      }),
    }
  }

  /// Updates the `source` of the [`IdentityStorageError`].
  pub fn with_source(self, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(self, source: Box<dyn Error + Send + Sync + 'static>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().source = Some(source);
    Self::from(extensive)
  }

  /// Updates the custom message of the [`IdentityStorageError`].
  pub fn with_custom_message(self, message: impl Into<Cow<'static, str>>) -> Self {
    self._with_custom_message(message.into())
  }

  fn _with_custom_message(self, message: Cow<'static, str>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().message = Some(message);
    Self::from(extensive)
  }
}

pub type IdentityStorageResult<T> = Result<T, IdentityStorageError>;

#[non_exhaustive]
#[derive(Debug)]
/// The cause of the failed [`IdentityStorage`](crate::identity_storage::IdentityStorage) operation.
pub enum IdentityStorageErrorKind {
  /// Indicates that the given [`MethodIdx`](crate::identifiers::MethodIdx) already exists in storage.
  MethodIdxAlreadyExists,
  /// Indicates that the storages could not find an entry corresponding to the given
  /// [`MethodIdx`](crate::identifiers::MethodIdx).
  MethodIdxNotFound,
  /// Indicates that the storage is unavailable for an unpredictable amount of time.
  ///
  /// Occurrences of this variant should hopefully be rare, but could occur if hardware fails, or a hosted storage
  /// goes offline.
  UnavailableIdentityStorage,

  /// Indicates that an attempt was made to authenticate with the identity storage, but this operation did not succeed.
  CouldNotAuthenticate,

  /// Indicates an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
  ///
  /// Returning this error signals to the caller that the operation may be retried with a chance of success.
  /// It is at the caller's discretion whether to retry or not, and how often.
  RetryableIOFailure,

  /// Indicates that something went wrong, but it is unclear whether the reason matches any of the other variants.
  ///
  /// When using this variant one may want to attach additional context to the corresponding [`IdentityStorageError`].
  /// See [`IdentityStorageError::with_message`](IdentityStorageError::with_message()) and
  /// [`IdentityStorageError::with_source`](IdentityStorageError::with_source()).
  Unspecified,
}

impl IdentityStorageErrorKind {
  /// Returns a report friendly representation of the [`IdentityStorageErrorCause`].
  const fn as_str(&self) -> &str {
    match self {
      Self::UnavailableIdentityStorage => "unavailable identity storage",
      Self::CouldNotAuthenticate => "authentication with the identity storage failed",
      Self::MethodIdxAlreadyExists => "method index already exists",
      Self::MethodIdxNotFound => "method index not found",
      Self::RetryableIOFailure => "identity storage operation was unsuccessful because of an I/O failure",
      Self::Unspecified => "identity storage operation failed",
    }
  }
}
