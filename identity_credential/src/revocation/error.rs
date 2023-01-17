// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A result type designed for `RevocationBitmap2022` handling.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors occurring when creating or extracting a Service of type `RevocationBitmap2022`
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("revocation bitmap could not be deserialized or decompressed")]
  /// Indicates that the bitmap could not be reconstructed.
  BitmapDecodingError(#[source] std::io::Error),
  #[error("revocation bitmap could not be serialized or compressed")]
  /// Indicates that the bitmap could not be encoded.
  BitmapEncodingError(#[source] std::io::Error),
  /// This variant is typically used to indicate invalid conversions between `Services`, `ServiceEndpoints` and
  /// `RevocationBitmaps`.
  #[error("{0}")]
  InvalidService(&'static str),
  /// Indicates a failure to decode a bitmap from a base64 string representation.
  #[error("unable to decode base64 string: `{0}`")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("could not parse url")]
  #[non_exhaustive]
  /// Indicates a failure to construct a URL when attempting to construct a `ServiceEndpoint`.
  UrlConstructionError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}
