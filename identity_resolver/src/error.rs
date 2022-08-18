// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  /// Caused by one or more failures when validating a presentation.
  #[error("presentation validation failed")]
  PresentationValidationError(#[from] identity_credential::validator::CompoundPresentationValidationError),
  //TODO: IMPROVE ERROR
  #[error("{0}")]
  ResolutionProblem(String),
  /// Caused by a failure to parse a DID string during DID resolution. 
  #[error("could not parse the given did")]
  #[non_exhaustive]
  DIDError{
    error: identity_did::did::DIDError
  }, 
  /// None of the attached ResolutionHandlers are configured to resolve the given DID.  
  #[error("could not find a compatible resolution handler")]
  #[non_exhaustive]
  NoCompatibleHandlerError, 
  /// The handler attempted to resolve the did, but the resolution did not succeed. 
  #[error("attempted to resolve did, but this action did not succeed")]
  ResolutionAttemptError(Box<dyn std::error::Error + Send + Sync + 'static>), 
}
