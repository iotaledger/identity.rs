// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported types for the JSON Web Key `kty` property.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-types)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum JwkType {
  /// Elliptic Curve.
  #[serde(rename = "EC")]
  Ec,
  /// RSA.
  #[serde(rename = "RSA")]
  Rsa,
  /// Octet sequence.
  #[serde(rename = "oct")]
  Oct,
  /// Octet string key pairs.
  #[serde(rename = "OKP")]
  Okp,

  //TODO: PQ - new JwkType
  /// JSON Web Key Type for the ML-DSA Algorithm Family.
  /// [More Info] (https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium#name-the-ml-dsa-key-type)
  #[serde(rename = "ML-DSA")]
  MLDSA,
  /// JSON Web Key Type for the SLH-DSA Algorithm Family.
  /// [More Info] (https://datatracker.ietf.org/doc/html/draft-ietf-cose-sphincs-plus#name-the-slh-dsa-key-type)
  #[serde(rename = "SLH-DSA")]
  SLHDSA,

  FALCON,
}

impl JwkType {
  /// Returns the JWK "kty" as a `str` slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::Ec => "EC",
      Self::Rsa => "RSA",
      Self::Oct => "oct",
      Self::Okp => "OKP",
      Self::MLDSA => "ML-DSA",
      Self::SLHDSA => "SLH-DSA",
      Self::FALCON => "FALCON",
    }
  }
}

impl Display for JwkType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
