// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Data Integrity types.

/// Alias for a [`Result`][::core::result::Result] with the error type [Error].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// Possible errors that can occur when working with Data Integrity types.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("multikey decoding error: {0}")]
  MultikeyDecode(&'static str, #[source] Option<Box<dyn std::error::Error + Send + Sync>>),
}
