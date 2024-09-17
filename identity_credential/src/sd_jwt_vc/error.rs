// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde_json::Value;
use thiserror::Error;

/// Error type that represents failures that might arise when dealing
/// with `SdJwtVc`s.
#[derive(Error, Debug)]
pub enum Error {
  /// A JWT claim required for an operation is missing.
  #[error("missing required claim \"{0}\"")]
  MissingClaim(&'static str),
  /// A JWT claim that must not be disclosed was found among the disclosed values.
  #[error("claim \"{0}\" cannot be disclosed")]
  DisclosedClaim(&'static str),
  /// Invalid value for a given JWT claim.
  #[error("invalid value for claim \"{name}\"; expected value of type {expected}, but {found} was found")]
  InvalidClaimValue {
    /// Name of the invalid claim.
    name: &'static str,
    /// Type expected for the claim's value.
    expected: &'static str,
    /// The claim's value.
    found: Value,
  },
  /// A low level SD-JWT error.
  #[error(transparent)]
  SdJwt(#[from] sd_jwt_payload_rework::Error),
  /// Value of header parameter `typ` is not valid.
  #[error("invalid \"typ\" value; expected \"vc+sd-jwt\" (or a superset) but found \"{0}\"")]
  InvalidJoseType(String),
}

/// Either a value of type `T` or an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
