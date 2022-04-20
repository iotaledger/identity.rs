// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("{0}")]
  CoreError(#[from] identity_core::Error),
  #[error("{0}")]
  CredError(#[from] identity_credential::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::did::DIDError),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[error("{0}")]
  ClientError(#[from] iota_client::error::Error),
  #[error("{0}")]
  IotaCoreError(#[from] identity_iota_core::Error),

  #[error("{0}")]
  DIDNotFound(String),
  #[error("{0}")]
  IncompatibleNetwork(String),
  #[error("Chain Error: {error}")]
  ChainError { error: &'static str },
  #[error("no client nodes provided for network")]
  NoClientNodesProvided,
  #[error("Invalid Explorer Url")]
  InvalidExplorerURL,
  #[error("compression error")]
  CompressionError,
  #[error("invalid message flags")]
  InvalidMessageFlags,
  /// Caused by a single concern credential or presentation validation method failing.
  #[error("A validation unit failed")]
  IsolatedValidationError(#[from] crate::credential::ValidationError),
  /// Caused by one or more failures when validating a credential.
  #[error("credential validation failed")]
  CredentialValidationError(#[from] crate::credential::CompoundCredentialValidationError),
  /// Caused by one or more failures when validating a presentation.
  #[error("presentation validation failed")]
  PresentationValidationError(#[from] crate::credential::CompoundPresentationValidationError),
}
