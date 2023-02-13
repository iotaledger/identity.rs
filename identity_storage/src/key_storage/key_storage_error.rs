// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

// use crate::error_utils::AsDynError;

/// The error type for KeyStorage operations.
///
/// Instances always carry a corresponding [`StorageErrorKind`] and may be extended with custom error messages and
/// source.
#[derive(Debug)]
pub struct KeyStorageError {
  repr: Repr,
}

impl Display for KeyStorageError {
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

impl Error for KeyStorageError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.extensive().and_then(|err| {
      err
        .source
        .as_ref()
        .map(|source| source.as_ref() as &(dyn Error + 'static))
    })
  }
}

#[derive(Debug)]
struct Extensive {
  cause: KeyStorageErrorKind,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
  message: Option<Cow<'static, str>>,
}

#[derive(Debug)]
enum Repr {
  Simple(KeyStorageErrorKind),
  Extensive(Box<Extensive>),
}

impl From<KeyStorageErrorKind> for KeyStorageError {
  fn from(cause: KeyStorageErrorKind) -> Self {
    Self::new(cause)
  }
}

impl From<Box<Extensive>> for KeyStorageError {
  fn from(extensive: Box<Extensive>) -> Self {
    Self {
      repr: Repr::Extensive(extensive),
    }
  }
}

impl KeyStorageError {
  /// Constructs a new [`KeyStorageError`].  
  pub fn new(cause: KeyStorageErrorKind) -> Self {
    Self {
      repr: Repr::Simple(cause),
    }
  }

  /// Returns a reference to corresponding [`KeyStorageErrorKind`] of this error.
  pub fn kind(&self) -> &KeyStorageErrorKind {
    match self.repr {
      Repr::Simple(ref cause) => cause,
      Repr::Extensive(ref extensive) => &extensive.cause,
    }
  }

  /// Converts this error into the corresponding [`KeyStorageErrorKind`] of this error.
  pub fn into_kind(self) -> KeyStorageErrorKind {
    match self.repr {
      Repr::Simple(cause) => cause,
      Repr::Extensive(extensive) => extensive.cause,
    }
  }

  /// Returns a reference to the custom message of the [`KeyStorageError`] if it was set.
  pub fn custom_message(&self) -> Option<&str> {
    self
      .extensive()
      .into_iter()
      .flat_map(|extensive| extensive.message.as_deref())
      .next()
  }

  /// Returns a reference to the attached source of the [`KeyStorageError`] if it was set.
  pub fn source_ref(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
    self.extensive().and_then(|extensive| extensive.source.as_deref())
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

  /// Updates the `source` of the [`KeyStorageError`].
  pub fn with_source(self, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(self, source: Box<dyn Error + Send + Sync + 'static>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().source = Some(source);
    Self::from(extensive)
  }

  /// Updates the custom message of the [`KeyStorageError`].
  pub fn with_custom_message(self, message: impl Into<Cow<'static, str>>) -> Self {
    self._with_custom_message(message.into())
  }

  fn _with_custom_message(self, message: Cow<'static, str>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().message = Some(message);
    Self::from(extensive)
  }
}

/// The cause of the failed [`KeyStorage`] operation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum KeyStorageErrorKind {
  /// Indicates that a user tried to generate a key which the [`KeyStorage`] implementation
  /// does not support.
  UnsupportedKeyType,

  /// Indicates an attempt to generate or insert a key with a key type that the [`KeyStorage`] implementation
  /// deems incompatible with the given signature algorithm.
  KeyAlgorithmMismatch,

  /// Indicates an attempt to parse a signature algorithm that is not recognized by the [`KeyStorage`] implementation.
  UnsupportedSignatureAlgorithm,

  /// Indicates that the [`KeyStorage`] implementation is not able to find the requested key.
  KeyNotFound,

  /// Indicates that the storage is unavailable for an unpredictable amount of time.
  ///
  /// Occurrences of this variant should hopefully be rare, but could occur if hardware fails, or a hosted key store
  /// goes offline.
  Unavailable,

  /// Indicates that an attempt was made to authenticate with the key storage, but the operation did not succeed.
  Unauthenticated,

  /// Indicates an unsuccessful I/O operation that may be retried, such as a temporary connection failure or timeouts.
  ///
  /// Returning this error signals to the caller that the operation may be retried with a chance of success.
  /// It is at the caller's discretion whether to retry or not, and how often.
  RetryableIOFailure,

  /// Indicates a failure to serialize or deserialize.
  SerializationError,

  /// Indicates that something went wrong, but it is unclear whether the reason matches any of the other variants.
  ///
  /// When using this variant one may want to attach additional context to the corresponding [`KeyStorageError`]. See
  /// [`KeyStorageError::with_message`](KeyStorageError::with_message()) and
  /// [`KeyStorageError::with_source`](KeyStorageError::with_source()).
  Unspecified,
}

impl KeyStorageErrorKind {
  /// Returns a report friendly representation of the [`KeyStorageErrorCause`].
  const fn as_str(&self) -> &str {
    match self {
      Self::UnsupportedKeyType => "key generation failed: the provided multikey schema is not supported",
      Self::KeyAlgorithmMismatch => "the key type cannot be used with the algorithm",
      Self::UnsupportedSignatureAlgorithm => "signing algorithm parsing failed",
      Self::KeyNotFound => "key not found",
      Self::Unavailable => "key storage unavailable",
      Self::Unauthenticated => "authentication with the key storage failed",
      Self::Unspecified => "key storage operation failed",
      Self::RetryableIOFailure => "key storage was unsuccessful because of an I/O failure",
      Self::SerializationError => "(de)serialization error",
    }
  }
}

impl Display for KeyStorageErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
