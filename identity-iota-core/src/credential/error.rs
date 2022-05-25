// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = CredentialStatusError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum CredentialStatusError {
  #[error("invalid status id: {0}")]
  InvalidStatusId(String),
  #[error("invalid status type: {0}")]
  InvalidStatusType(String),
  #[error("invalid status index: {0}")]
  InvalidStatusIndex(String),
}
