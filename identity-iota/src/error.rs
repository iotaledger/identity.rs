// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::errors::CompoundError;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("{0}")]
  CoreError(#[from] identity_core::Error),
  #[error("{0}")]
  DiffError(#[from] identity_core::diff::Error),
  #[error("{0}")]
  CredError(#[from] identity_credential::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::did::DIDError),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[error("{0}")]
  ClientError(#[from] iota_client::error::Error),
  #[error("Invalid Message: {0}")]
  InvalidMessage(#[from] iota_client::bee_message::Error),

  #[error("{0}")]
  DIDNotFound(String),
  #[error("Invalid Document - Missing Message Id")]
  InvalidDocumentMessageId,
  #[error("Invalid Document - Signing Verification Method Type Not Supported")]
  InvalidDocumentSigningMethodType,
  #[error("Invalid Verification Method - Missing Fragment")]
  InvalidMethodMissingFragment,
  #[error("Invalid Root Document")]
  InvalidRootDocument,
  #[error("Invalid Network Name")]
  InvalidNetworkName,
  #[error("{0}")]
  IncompatibleNetwork(String),
  #[error("Invalid Presentation Holder")]
  InvalidPresentationHolder,
  #[error("Chain Error: {error}")]
  ChainError { error: &'static str },
  #[error("Missing Signing Key")]
  MissingSigningKey,
  #[error("Cannot Revoke Verification Method")]
  CannotRevokeMethod,
  #[error("no client nodes provided for network")]
  NoClientNodesProvided,
  #[error("No Explorer URL Set")]
  NoExplorerURLSet,
  #[error("Invalid Explorer Url")]
  InvalidExplorerURL,
  #[error("compression error")]
  CompressionError,
  #[error("invalid message flags")]
  InvalidMessageFlags,
  /// Caused by a single validation unit failing.
  #[error("A validation unit failed")]
  UnsuccessfulValidationUnit(#[from] crate::credential::errors::StandaloneValidationError),
  /// Caused by a failure to resolve a Credential.  
  #[error("credential validation failed")]
  UnsuccessfulCredentialValidation(#[source] crate::credential::errors::AccumulatedCredentialValidationError),
  /// Caused by a failure to validate a Presentation.
  #[error("presentation validation failed")]
  UnsuccessfulPresentationValidation(#[source] crate::credential::errors::AccumulatedPresentationValidationError),
  /// Caused by a failure to construct a `ResolvedCredential`.
  #[error("failed to construct ResolvedCredential: invalid input data")]
  InvalidCredentialPairing(#[source] CompoundError),
  /// Caused by a failure to construct a `ResolvedPresentation`.
  #[error("failed to construct ResolvedPresentation: invalid input data")]
  InvalidPresentationPairing(#[source] CompoundError),
}
