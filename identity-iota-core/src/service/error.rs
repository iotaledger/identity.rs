// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = ServiceError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum ServiceError {
  #[error("invalid service")]
  InvalidService(#[source] identity_did::error::Error),
  #[error("invalid service id: {0}")]
  InvalidServiceId(String),
  #[error("invalid service type: {0}")]
  InvalidServiceType(String),
  #[error("invalid service endpoint: {0}")]
  InvalidServiceEndpoint(String),
  #[error("revocation method Error: {0}")]
  RevocationMethodError(String, #[source] crate::revocation::RevocationMethodError),
}
