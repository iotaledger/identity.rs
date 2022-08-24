// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

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
  #[error("could not parse the given did {context}")]
  #[non_exhaustive]
  DIDParsingError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
    context: ParsingContext,
  },
  /// A handler attached to the [`Resolver`](crate::resolution::Resolver) attempted to resolve the DID, but the resolution did not succeed.
  #[error("attempted to resolve DID, but this action did not succeed")]
  HandlerError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
  /// Caused by attempting to resolve a DID whose method does not have a corresponding handler attached to the [`Resolver`](crate::resolution::Resolver).  
  #[error("the DID method: {0} is not supported by the resolver")]
  UnsupportedMethodError(String),

  /// Caused by a failure to cast a resolved document to the specified output type.
  ///
  /// The error wraps an `Any` trait object enabling the caller to efficiently try casting to another type
  /// of their choice.
  ///
  /// This variant can only be returned from a dynamic resolver ([`Resolver<Box<dyn ValidatorDocument>>`](crate::resolution::Resolver)).  
  #[error("the resolved abstract document did not match the specified type")]
  CastingError(Box<dyn Any>),
}

#[derive(Debug)]
#[non_exhaustive]
/// Indicates the context in which the
/// DID could not be parsed during resolution.
pub enum ParsingContext {
  /// Attempted to resolve a presentation holder's DID document, but the holder's DID could not be parsed.  
  PresentationHolderResolution,
  /// Attempted to resolve a credential issuer's DID document, but the issuer's DID could not be parsed.  
  CredentialIssuerResolution,

  /// Attempted to resolve the DID documents belonging to the issuers of the presentation's credentials, but (at least)
  /// one issuer's DID could not be parsed.
  PresentationIssuersResolution(usize),
  /// Further context regarding the resolution of the DID is not available.
  Unknown,
}

impl std::fmt::Display for ParsingContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: &str  = match self {
      ParsingContext::PresentationHolderResolution =>
        ": failed to parse the presentation holder's URL to the required DID format",
      ParsingContext::CredentialIssuerResolution =>
        ": failed to parse the credential issuer's URL to the required DID format",
      ParsingContext::PresentationIssuersResolution(idx) => {let message = format!(
        ": the URL of credential num. {}'s issuer could not be parsed to the required DID format",
        idx
      ); 
      message.as_str()
    },
      ParsingContext::Unknown => "",
    };

    write!(
      f,
      "{message}",
    )
  }
}
