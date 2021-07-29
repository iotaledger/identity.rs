// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = CommError> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum CommError {
  #[error(transparent)]
  IotaError(#[from] identity_iota::IotaError),
  #[error(transparent)]
  CoreError(#[from] identity_core::CoreError),
  #[error(transparent)]
  DidError(#[from] identity_did::DIDError),
  #[error(transparent)]
  JoseError(#[from] libjose::Error),
  #[error(transparent)]
  Utf8Error(#[from] std::string::FromUtf8Error),
}
