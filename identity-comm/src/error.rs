// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides a composite of errors from identity.rs
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("invalid serialization")]
  InvalidSerialization, // TODO: Remove this when this crate has its error types refactored
  #[error("invalid deserialization")]
  InvalidDeserialization, //TODO: Remove this when this crate has its error types refactored
  #[error(transparent)]
  IotaError(#[from] identity_iota::Error),
  #[error("remove this error when the errors in this crate get refactored")]
  CoreError, //TODO: Remove this when this crate has its error types refactored
  #[error(transparent)]
  DidError(#[from] identity_did::Error),
  #[error(transparent)]
  JoseError(#[from] libjose::Error),
  #[error(transparent)]
  Utf8Error(#[from] std::string::FromUtf8Error),
}
