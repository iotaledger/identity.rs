// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A result type designed for `RevocationBitmap2022` handling.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors occurring when creating or extracting a Service of type `RevocationBitmap2022`
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("revocation bitmap could not be deserialized or decompressed")]
  BitmapDecodingError(#[source] std::io::Error),
  #[error("revocation bitmap could not be serialized or compressed")]
  BitmapEncodingError(#[source] std::io::Error),
  #[error("{0}")]
  InvalidService(&'static str),
  #[error("unable to decode base64 string: `{0}`")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("could not parse url")]
  #[non_exhaustive]
  UrlConstructionError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
