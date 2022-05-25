// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = ServiceError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum ServiceError {
  #[error("invalid service id: {0}")]
  InvalidServiceId(String),
  #[error("invalid service type: {0}")]
  InvalidServiceType(String),
  #[error("invalid service endpoint: {0}")]
  InvalidServiceEndpoint(String),
}
