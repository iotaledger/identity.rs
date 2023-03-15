// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

/// A container implementing the [`std::error::Error`] trait.
///
/// Instances always carry a corresponding `kind` of type `T` and may be extended with custom error
/// messages, source and recovery data.
///
/// This type is mainly designed to accommodate for the [single struct error design pattern](https://nrc.github.io/error-docs/error-design/error-type-design.html#single-struct-style).
///
/// When used in a specialized context it is recommended to use a type alias (i.e. `type MyError =
/// SingleStructError<MyErrorKind>` or `type MyError = SingleStructError<MyErrorKind, MyRecoveryData>`).
#[derive(Debug)]
pub struct SingleStructError<T: Debug + Display, S: Debug = ()> {
  repr: Repr<T, S>,
}

impl<T: Display + Debug, S: Debug> Display for SingleStructError<T, S> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.repr {
      Repr::Simple(ref cause) => write!(f, "{}", cause),
      Repr::Extensive(ref extensive) => {
        write!(f, "{}", &extensive.cause)?;
        let Some(ref message) = extensive.message else {return Ok(())};
        write!(f, " message: {}", message.as_ref())
      }
    }
  }
}

impl<T: Debug + Display, S: Debug> Error for SingleStructError<T, S> {
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
struct Extensive<T: Debug + Display, S: Debug> {
  cause: T,
  source: Option<Box<dyn Error + Send + Sync + 'static>>,
  message: Option<Cow<'static, str>>,
  recovery_data: Option<S>,
}

#[derive(Debug)]
enum Repr<T: Debug + Display, S: Debug> {
  Simple(T),
  Extensive(Box<Extensive<T, S>>),
}

impl<T: Debug + Display, S: Debug> From<T> for SingleStructError<T, S> {
  fn from(cause: T) -> Self {
    Self::new(cause)
  }
}

impl<T: Debug + Display, S: Debug> From<Box<Extensive<T, S>>> for SingleStructError<T, S> {
  fn from(extensive: Box<Extensive<T, S>>) -> Self {
    Self {
      repr: Repr::Extensive(extensive),
    }
  }
}

impl<T: Debug + Display, S: Debug> SingleStructError<T, S> {
  /// Constructs a new [`SingleStructError`].  
  pub fn new(cause: T) -> Self {
    Self {
      repr: Repr::Simple(cause),
    }
  }

  /// Returns a reference to the corresponding [`ErrorCause`] of this error.
  pub fn kind(&self) -> &T {
    match self.repr {
      Repr::Simple(ref cause) => cause,
      Repr::Extensive(ref extensive) => &extensive.cause,
    }
  }

  /// Converts this error into the corresponding [`ErrorCause`] of this error.
  pub fn into_kind(self) -> T {
    match self.repr {
      Repr::Simple(cause) => cause,
      Repr::Extensive(extensive) => extensive.cause,
    }
  }

  /// Returns a reference to the custom message of the [`SingleStructError`] if it was set.
  pub fn custom_message(&self) -> Option<&str> {
    self
      .extensive()
      .into_iter()
      .flat_map(|extensive| extensive.message.as_deref())
      .next()
  }

  /// Returns a reference to the attached source of the [`SingleStructError`] if it was set.
  pub fn source_ref(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
    self.extensive().and_then(|extensive| extensive.source.as_deref())
  }

  /// Converts this error into the source error if it was set.
  pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync + 'static>> {
    self.into_extensive().source
  }

  /// Returns a reference to the attached recovery data of the [`SingleStructError`] if it was set.
  pub fn recovery_data(&self) -> Option<&S> {
    self.extensive().and_then(|extensive| extensive.recovery_data.as_ref())
  }

  /// Converts this error into the recovery data if it was set.
  pub fn into_recovery_data(self) -> Option<S> {
    self.into_extensive().recovery_data
  }

  fn extensive(&self) -> Option<&Extensive<T, S>> {
    match self.repr {
      Repr::Extensive(ref extensive) => Some(extensive.as_ref()),
      _ => None,
    }
  }

  fn into_extensive(self) -> Box<Extensive<T, S>> {
    match self.repr {
      Repr::Extensive(extensive) => extensive,
      Repr::Simple(cause) => Box::new(Extensive {
        cause,
        source: None,
        message: None,
        recovery_data: None,
      }),
    }
  }

  /// Updates the `source` of the [`SingleStructError`].
  pub fn with_source(self, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(self, source: Box<dyn Error + Send + Sync + 'static>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().source = Some(source);
    Self::from(extensive)
  }

  /// Updates the custom message of the [`SingleStructError`].
  pub fn with_custom_message(self, message: impl Into<Cow<'static, str>>) -> Self {
    self._with_custom_message(message.into())
  }

  fn _with_custom_message(self, message: Cow<'static, str>) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().message = Some(message);
    Self::from(extensive)
  }

  /// Add recovery data to the [`SingleStructError`].
  pub fn with_recovery_data(self, data: S) -> Self {
    self._with_recovery_data(data)
  }

  fn _with_recovery_data(self, data: S) -> Self {
    let mut extensive = self.into_extensive();
    extensive.as_mut().recovery_data = Some(data);
    Self::from(extensive)
  }
}
