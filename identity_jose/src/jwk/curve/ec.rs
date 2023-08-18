// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported Elliptic Curves.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-elliptic-curve)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcCurve {
  /// P-256 Curve.
  P256,
  /// P-384 Curve.
  P384,
  /// P-521 Curve.
  P521,
  /// SECG secp256k1 curve.
  Secp256K1,
}

impl EcCurve {
  /// Returns the name of the curve as a string slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::P256 => "P-256",
      Self::P384 => "P-384",
      Self::P521 => "P-521",
      Self::Secp256K1 => "secp256k1",
    }
  }
}

impl Display for EcCurve {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
