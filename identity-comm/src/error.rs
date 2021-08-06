// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error(transparent)]
  IotaError(#[from] identity_iota::Error),
  #[error(transparent)]
  CoreError(#[from] identity_core::Error),
  #[error(transparent)]
  DidError(#[from] identity_did::Error),
  #[error(transparent)]
  JoseError(#[from] libjose::Error),
  #[error(transparent)]
  Utf8Error(#[from] std::string::FromUtf8Error),
}
