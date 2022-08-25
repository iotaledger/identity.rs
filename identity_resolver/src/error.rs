// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// Caused by one or more failures when validating a presentation.
  #[error("presentation validation failed")]
  #[non_exhaustive]
  PresentationValidationError {
    source: identity_credential::validator::CompoundPresentationValidationError,
  },
  /// Caused by a failure to parse a DID string during DID resolution.
  #[error("{context}: could not parse the given did")]
  #[non_exhaustive]
  DIDParsingError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
    context: ResolutionAction,
  },
  /// A handler attached to the [`Resolver`](crate::resolution::Resolver) attempted to resolve the DID, but the
  /// resolution did not succeed.
  #[error("{context}: the attached handler failed")]
  HandlerError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
    context: ResolutionAction,
  },
  /// Caused by attempting to resolve a DID whose method does not have a corresponding handler attached to the
  /// [`Resolver`](crate::resolution::Resolver).
  #[error("{context} the DID method \"{method}\" is not supported by the resolver")]
  UnsupportedMethodError { method: String, context: ResolutionAction },
}

impl Error {
  /// Replaces the value of the wrapped [`ResolutionAction`] when relevant, otherwise the [`Error`] is left untouched.
  pub(super) fn update_resolution_action(self, context: ResolutionAction) -> Self {
    match self {
      Error::DIDParsingError { source, .. } => Self::DIDParsingError { source, context },
      Error::HandlerError { source, .. } => Self::HandlerError { source, context },
      _ => self,
    }
  }
}

#[derive(Debug)]
#[non_exhaustive]
/// Indicates the action the [`Resolver`](crate::resolution::Resolver) was performing when an error ocurred.
pub enum ResolutionAction {
  PresentationHolderResolution,

  CredentialIssuerResolution,

  PresentationIssuersResolution(usize),
  /// Further context regarding the resolution of the DID is not available.
  Unknown,
}

impl std::fmt::Display for ResolutionAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: Cow<str> = match self {
      ResolutionAction::PresentationHolderResolution => {
        ": attempt to resolve the presentation holder's DID failed".into()
      }
      ResolutionAction::CredentialIssuerResolution => ": attempt to resolve the credential issuer's DID failed".into(),
      ResolutionAction::PresentationIssuersResolution(idx) => format!(
        ": attempt to resolve the credential issuer's DID of credential num. {} failed",
        idx
      )
      .into(),
      ResolutionAction::Unknown => "".into(),
    };

    write!(f, "{}", message)
  }
}
