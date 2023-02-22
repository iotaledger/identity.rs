// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported Elliptic Curves.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-elliptic-curve)
#[typeshare::typeshare]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdCurve {
  /// Ed25519 signature algorithm key pairs.
  Ed25519,
  /// Ed448 signature algorithm key pairs.
  Ed448,
}

impl EdCurve {
  pub const fn name(self) -> &'static str {
    match self {
      Self::Ed25519 => "Ed25519",
      Self::Ed448 => "Ed448",
    }
  }
}

impl Display for EdCurve {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
