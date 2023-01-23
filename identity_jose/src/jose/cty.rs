// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Possible values of the JOSE "cty" header parameter
///
/// [More Info](https://tools.ietf.org/html/rfc7519#section-5.2)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum JoseContentType {
  /// Indicates the content of the token is a JSON Web Token.
  ///
  /// Note: This indicates a nested JWT structure.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7519#section-5.2)
  JWT,
  /// Indicates the content of the token is a JSON Web Message.
  ///
  /// Note: This indicates a nested JWM structure.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#rfc.section.4.2)
  JWM,
}

impl JoseContentType {
  /// Returns the JOSE "cty" parameter as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::JWT => "JWT",
      Self::JWM => "JWM",
    }
  }
}

impl Display for JoseContentType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
