// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Possible values of the JOSE "typ" header parameter
///
/// [More Info](https://tools.ietf.org/html/rfc7519#section-5.1)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum JoseTokenType {
  /// Indicates that the token is a JSON Web Token.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-5.1)
  JWT,
  /// Indicates the token is a JSON Web Message.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#rfc.section.4.1)
  JWM,
  /// Indicates that the token is a JWE/JWS using compact serialization.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  JOSE,
  /// Indicates that the token is a JWE/JWS using JSON serialization.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[serde(rename = "JOSE+JSON")]
  JOSE_JSON,
}

impl JoseTokenType {
  /// Returns the JOSE "typ" parameter as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::JWT => "JWT",
      Self::JWM => "JWM",
      Self::JOSE => "JOSE",
      Self::JOSE_JSON => "JOSE+JSON",
    }
  }
}

impl Display for JoseTokenType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
