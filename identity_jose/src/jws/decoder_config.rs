// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::JwsFormat;

#[derive(Clone, Debug)]
/// Configuration parameters used in [`Decoder::decode`](crate::jws::Decoder::decode()).
pub(super) struct DecodingConfig {
  pub(super) crits: Option<Vec<String>>,

  pub(super) jwk_must_have_alg: bool,

  pub(super) strict_signature_verification: bool,

  pub(super) format: JwsFormat,
}

impl Default for DecodingConfig {
  fn default() -> Self {
    Self {
      crits: None,
      jwk_must_have_alg: true,
      strict_signature_verification: true,
      format: JwsFormat::Compact,
    }
  }
}

impl DecodingConfig {}
