// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Error returned from the [Resolver's](crate::Resolver) methods.
///
/// The [`Self::cause`](Self::cause()) method provides information about the cause of the error,
/// while [`Self::action`](Self::action()) provides more context about the action the resolver was carrying out when the
/// error occurred.
#[derive(Debug)]
pub struct Error {
  cause: ErrorCause,
  action: Option<ResolutionAction>,
}

impl Error {
  pub(crate) fn new(cause: ErrorCause) -> Self {
    Self { cause, action: None }
  }

  /// Returns the cause of the error.
  pub fn cause(&self) -> &ErrorCause {
    &self.cause
  }

  /// Converts the error into [`ErrorCause`].
  pub fn into_cause(self) -> ErrorCause {
    self.cause
  }

  /// Returns more context regarding the action the [`Resolver`](crate::Resolver) was performing when the error occurred
  /// if available.
  ///
  /// This is mainly useful when the error originated from calling
  /// [Resolver::verify_presentation](crate::Resolver::verify_presentation()) as one may then want to know answers to
  /// questions of the form: did the problem occur when attempting to resolve the holder's DID, or was there perhaps a
  /// problem when resolving the DID of a certain credential issuer?.
  pub fn action(&self) -> Option<ResolutionAction> {
    self.action
  }

  /// Replaces the value of the [`ResolutionAction`], but leaves the category untouched.
  pub(crate) fn update_resolution_action(mut self, action: ResolutionAction) -> Self {
    self.action = Some(action);
    self
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(action) = self.action {
      write!(f, "{}: {}", action, self.cause)
    } else {
      write!(f, "{}", self.cause)
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self.cause.source()
  }
}

/// Error failure modes associated with the methods on the [Resolver's](crate::Resolver).
///
/// NOTE: This is a "read only error" in the sense that it can only be constructed by the methods in this crate.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum ErrorCause {
  /// Caused by one or more failures when validating a presentation.
  #[error("presentation validation failed")]
  #[non_exhaustive]
  PresentationValidationError {
    source: identity_credential::validator::CompoundPresentationValidationError,
  },
  /// Caused by a failure to parse a DID string during DID resolution.
  #[error("could not parse the given did")]
  #[non_exhaustive]
  DIDParsingError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
  },
  /// A handler attached to the [`Resolver`](crate::resolution::Resolver) attempted to resolve the DID, but the
  /// resolution did not succeed.
  #[error("the attached handler failed")]
  #[non_exhaustive]
  HandlerError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
  },
  /// Caused by attempting to resolve a DID whose method does not have a corresponding handler attached to the
  /// [`Resolver`](crate::resolution::Resolver).
  #[error("the DID method \"{method}\" is not supported by the resolver")]
  UnsupportedMethodError { method: String },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
/// Indicates the action the [`Resolver`](crate::resolution::Resolver) was performing when an error ocurred.
pub enum ResolutionAction {
  /// Errored while attempting to resolve a presentation holder's DID.
  PresentationHolderResolution,
  /// Errored while attempting to resolve the DIDs of the credential issuers of the given presentation.
  ///
  ///  The wrapped `usize` indicates the position of a credential whose issuer's DID could not be resolved.
  PresentationIssuersResolution(usize),
}

impl std::fmt::Display for ResolutionAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: Cow<str> = match self {
      ResolutionAction::PresentationHolderResolution => {
        "attempt to resolve the presentation holder's DID failed".into()
      }
      ResolutionAction::PresentationIssuersResolution(idx) => format!(
        "attempt to resolve the credential issuer's DID of credential num. {} in the presentation failed",
        idx
      )
      .into(),
    };

    write!(f, "{}", message)
  }
}
