// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

// TODO: replace all variants with specific errors?
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  #[error("{0}")]
  CoreError(#[from] identity_core::Error),
  #[error("{0}")]
  CredError(#[from] identity_credential::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::did::DIDError),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[cfg(feature = "iota-client")]
  #[error("{0}")]
  ClientError(#[from] iota_client::error::Error),
  #[cfg(feature = "iota-client")]
  #[error("{0}")]
  BlockError(#[from] iota_client::block::Error),

  #[error("invalid network name")]
  InvalidNetworkName,
  #[error("invalid state metadata {0}")]
  InvalidStateMetadata(&'static str),
  #[error("credential revocation error")]
  RevocationError(#[source] identity_did::Error),
}
