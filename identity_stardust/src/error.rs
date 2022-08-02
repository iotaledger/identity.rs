// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::StardustDID;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  #[error("serialization error")]
  SerializationError(#[source] identity_core::Error),
  #[error("{0}")]
  DIDSyntaxError(#[from] identity_did::did::DIDError),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[cfg(feature = "iota-client")]
  #[error("DID update failed")]
  DIDUpdateError(#[source] iota_client::error::Error),
  #[cfg(feature = "iota-client")]
  #[error("DID resolution failed")]
  ResolutionError(#[source] iota_client::error::Error),
  #[cfg(feature = "iota-client")]
  #[error("{0}")]
  BasicOutputBuildError(#[source] iota_client::block::Error),
  #[error("\"{0}\" is not a valid network name")]
  InvalidNetworkName(String),
  #[error("unable to resolve a `{expected}` DID on network `{actual}`")]
  NetworkMismatch { expected: String, actual: String },
  #[error("invalid state metadata {0}")]
  InvalidStateMetadata(&'static str),
  #[error("credential revocation error")]
  RevocationError(#[source] identity_did::Error),
  #[error("DID `{0}` is deactivated")]
  DeactivatedDID(StardustDID),
  #[cfg(feature = "iota-client")]
  #[error("alias output build error")]
  AliasOutputBuildError(#[source] iota_client::block::Error),
  #[cfg(feature = "iota-client")]
  #[error("output with id `{0}` is not an alias output")]
  NotAnAliasOutput(iota_client::block::output::OutputId),
  #[cfg(feature = "iota-client")]
  #[error("converting a DTO to an output failed")]
  OutputConversionError(#[source] iota_client::block::DtoError),
}
