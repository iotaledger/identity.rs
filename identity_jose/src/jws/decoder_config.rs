// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::JwsFormat;

#[derive(Clone, Debug)]
/// Configuration defining the behaviour of a [`Decoder`].
pub struct JwsDecoderConfig {
  pub(super) crits: Option<Vec<String>>,

  pub(super) jwk_must_have_alg: bool,

  pub(super) strict_signature_verification: bool,

  pub(super) format: JwsFormat,

  pub(super) fallback_to_jwk_header: bool,
}

impl Default for JwsDecoderConfig {
  fn default() -> Self {
    Self {
      crits: None,
      jwk_must_have_alg: true,
      strict_signature_verification: true,
      format: JwsFormat::Compact,
      fallback_to_jwk_header: false,
    }
  }
}

impl JwsDecoderConfig {
  /// Append values to the list of permitted extension parameters.
  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  /// Defines whether a given [`Jwk`](crate::jwk::Jwk) used to verify a JWS,
  /// must have an `alg` parameter corresponding to the one extracted from the JWS header.
  /// This value is `true` by default.  
  pub fn jwk_must_have_alg(mut self, value: bool) -> Self {
    self.jwk_must_have_alg = value;
    self
  }

  /// When verifying a JWS encoded with the general JWS JSON serialization
  /// this value decides whether all signatures must be verified (the default behavior),
  /// otherwise only one signature needs to be verified in order for the entire JWS to be accepted.
  pub fn strict_signature_verification(mut self, value: bool) -> Self {
    self.strict_signature_verification = value;
    self
  }

  /// Specify the serialization format the `Decoder` accepts. The default is [`JwsFormat::Compact`].
  pub fn serialization_format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  /// Specify whether to attempt to extract a public key from the JOSE header if the
  /// `jwk` provider fails to provide one.
  pub fn fallback_to_jwk_header(mut self, value: bool) -> Self {
    self.fallback_to_jwk_header = value;
    self
  }

  /// Specify which jws serialization format the [`Decoder`] should accept.
  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }
}
