// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("Diff Error: {0}")]
  DiffError(String),
  #[error("Merge Error: {0}")]
  MergeError(String),
  #[error("Conversion Error: {0}")]
  ConversionError(String),
}

impl Error {
  pub fn diff<T>(message: T) -> Self
  where
    T: Display,
  {
    Self::DiffError(format!("{}", message))
  }

  pub fn merge<T>(message: T) -> Self
  where
    T: Display,
  {
    Self::MergeError(format!("{}", message))
  }

  pub fn convert<T>(message: T) -> Self
  where
    T: Display,
  {
    Self::ConversionError(format!("{}", message))
  }
}
