// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::deflate::CompressionLevel;
use miniz_oxide::inflate::decompress_to_vec;

use crate::error::Result;
use crate::lib::*;

const DEFLATE_LEVEL: u8 = CompressionLevel::DefaultLevel as u8;

/// Supported algorithms for the JSON Web Encryption `zip` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-encryption-compression-algorithms)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JweCompression {
  /// Compression with the DEFLATE [RFC1951](https://tools.ietf.org/html/rfc1951) algorithm.
  #[serde(rename = "DEF")]
  Deflate,
}

impl JweCompression {
  /// Returns the JWE "zip" claim as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Deflate => "DEF",
    }
  }

  pub fn compress(&self, content: &[u8]) -> Result<Vec<u8>> {
    match self {
      Self::Deflate => Ok(compress_to_vec(content, DEFLATE_LEVEL)),
    }
  }

  pub fn decompress(&self, content: &[u8]) -> Result<Vec<u8>> {
    match self {
      Self::Deflate => decompress_to_vec(content).map_err(Into::into),
    }
  }
}

impl Default for JweCompression {
  fn default() -> Self {
    Self::Deflate
  }
}

impl Display for JweCompression {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.name())
  }
}
