// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur in the library.

use core::fmt::Display;
use core::fmt::Formatter;

/// Alias for a `Result` with the error type [Error].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// All possible errors that can occur in the library.
#[derive(Debug)]
pub enum Error {
  InvalidRsaPrime,
  InvalidJson(serde_json::Error),
  InvalidBase64(base64::DecodeError),
  InvalidUtf8(core::str::Utf8Error),
  InvalidClaim(&'static str),
  MissingClaim(&'static str),
  InvalidParam(&'static str),
  MissingParam(&'static str),
  InvalidContent(&'static str),
  InvalidArray(&'static str),
  AlgError(&'static str),
  EncError(&'static str),
  KeyError(&'static str),
  SignatureCreationError(Box<dyn std::error::Error + Send + Sync>),
  SignatureVerificationError(Box<dyn std::error::Error + Send + Sync>),
}

// TODO: Use thiserror? Lowercase error message!
impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::InvalidRsaPrime => f.write_str("Invalid Rsa Prime Value"),
      Self::InvalidJson(inner) => f.write_fmt(format_args!("Invalid JSON: {}", inner)),
      Self::InvalidBase64(inner) => f.write_fmt(format_args!("Invalid Base64: {}", inner)),
      Self::InvalidUtf8(inner) => f.write_fmt(format_args!("Invalid Utf-8: {:?}", inner)),
      Self::InvalidClaim(inner) => f.write_fmt(format_args!("Invalid Claim: {}", inner)),
      Self::MissingClaim(inner) => f.write_fmt(format_args!("Missing Claim: {}", inner)),
      Self::InvalidParam(inner) => f.write_fmt(format_args!("Invalid Param: {}", inner)),
      Self::MissingParam(inner) => f.write_fmt(format_args!("Missing Param: {}", inner)),
      Self::InvalidContent(inner) => f.write_fmt(format_args!("Invalid Content: {}", inner)),
      Self::InvalidArray(inner) => f.write_fmt(format_args!("Invalid Array: {}", inner)),
      Self::AlgError(inner) => f.write_fmt(format_args!("Unsupported Algorithm: {}", inner)),
      Self::EncError(inner) => f.write_fmt(format_args!("Encryption Error: {}", inner)),
      Self::KeyError(inner) => f.write_fmt(format_args!("Invalid Key Format: {}", inner)),
      Self::SignatureCreationError(inner) => f.write_fmt(format_args!("signature creation error: {}", inner)),
      Self::SignatureVerificationError(inner) => f.write_fmt(format_args!("signature verification error: {}", inner)),
    }
  }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
  fn from(other: serde_json::Error) -> Self {
    Self::InvalidJson(other)
  }
}

impl From<base64::DecodeError> for Error {
  fn from(other: base64::DecodeError) -> Self {
    Self::InvalidBase64(other)
  }
}
