// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("IOTA Error: {0}")]
  IotaError(#[from] identity_iota::Error),
  #[error("IOTA Core Error: {0}")]
  CoreError(#[from] identity_core::Error),
  #[error("DID Error: {0}")]
  DidError(#[from] identity_did::Error),
  #[error("JOSE Error: {0}")]
  JoseError(#[from] libjose::Error),
  #[error(transparent)]
  Utf8Error(#[from] std::string::FromUtf8Error),
}
