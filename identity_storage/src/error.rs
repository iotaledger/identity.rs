// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

/// The error type for key storage operations.
///
/// Instances always carry a corresponding [`StorageErrorKind`] and may be extended with custom error messages and
/// source.
#[derive(Debug)]
pub struct StorageError<T: StorageErrorKind> {
  repr: Repr<T>,
}

/// Error types that can happen during storage operations.
pub trait StorageErrorKind: Display + Debug {
  fn description(&self) -> &str;
}

impl<T: StorageErrorKind> Display for StorageError<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.repr {
      Repr::Simple(ref cause) => write!(f, "{}", cause.description()),
      Repr::Extensive(ref extensive) => {
        write!(f, "{}", extensive.cause.description())?;
        let Some(ref message) = extensive.message else {return Ok(())};
        write!(f, " message: {}", message.as_ref())
      }
    }
  }
}

impl<T: StorageErrorKind> Error for StorageError<T> {
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
struct Extensive<T: StorageErrorKind> {
  cause: T,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
  message: Option<Cow<'static, str>>,
}

#[derive(Debug)]
enum Repr<T: StorageErrorKind> {
  Simple(T),
  Extensive(Box<Extensive<T>>),
}

impl<T: StorageErrorKind> From<T> for StorageError<T> {
  fn from(cause: T) -> Self {
    Self::new(cause)
  }
}

impl<T: StorageErrorKind> From<Box<Extensive<T>>> for StorageError<T> {
  fn from(extensive: Box<Extensive<T>>) -> Self {
    Self {
      repr: Repr::Extensive(extensive),
    }
  }
}

impl<T: StorageErrorKind> StorageError<T> {
  /// Constructs a new [`StorageError`].  
  pub fn new(cause: T) -> Self {
    Self {
      repr: Repr::Simple(cause),
    }
  }

  /// Returns a reference to corresponding [`StorageErrorKind`] of this error.
  pub fn kind(&self) -> &T {
    match self.repr {
      Repr::Simple(ref cause) => cause,
      Repr::Extensive(ref extensive) => &extensive.cause,
    }
  }

  /// Converts this error into the corresponding [`StorageErrorKind`] of this error.
  pub fn into_kind(self) -> T {
    match self.repr {
      Repr::Simple(cause) => cause,
      Repr::Extensive(extensive) => extensive.cause,
    }
  }

  /// Returns a reference to the custom message of the [`StorageError`] if it was set.
  pub fn custom_message(&self) -> Option<&str> {
    self
      .extensive()
      .into_iter()
      .flat_map(|extensive| extensive.message.as_deref())
      .next()
  }

  /// Returns a reference to the attached source of the [`StorageError`] if it was set.
  pub fn source_ref(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
    self.extensive().and_then(|extensive| extensive.source.as_deref())
  }

  /// Converts this error into the source error if it was set.
  pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync + 'static>> {
    self.into_extensive().source
  }

  fn extensive(&self) -> Option<&Extensive<T>> {
    match self.repr {
      Repr::Extensive(ref extensive) => Some(extensive.as_ref()),
      _ => None,
    }
  }

  fn into_extensive(self) -> Box<Extensive<T>> {
    match self.repr {
      Repr::Extensive(extensive) => extensive,
      Repr::Simple(cause) => Box::new(Extensive {
        cause,
        source: None,
        message: None,
      }),
    }
  }

  /// Updates the `source` of the [`StorageError`].
  pub fn with_source(self, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(self, source: Box<dyn Error + Send + Sync + 'static>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().source = Some(source);
    Self::from(extensive)
  }

  /// Updates the custom message of the [`StorageError`].
  pub fn with_custom_message(self, message: impl Into<Cow<'static, str>>) -> Self {
    self._with_custom_message(message.into())
  }

  fn _with_custom_message(self, message: Cow<'static, str>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().message = Some(message);
    Self::from(extensive)
  }
}
