// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
  #[error("method hash construction failed")]
  MethodHashConstruction(String),
}
