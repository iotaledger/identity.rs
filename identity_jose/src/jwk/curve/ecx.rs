// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported Elliptic Curves.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-elliptic-curve)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcxCurve {
  /// X25519 function key pairs.
  X25519,
  /// X448 function key pairs.
  X448,
}

impl EcxCurve {
  /// Returns the name of the curve as a string slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::X25519 => "X25519",
      Self::X448 => "X448",
    }
  }
}

impl Display for EcxCurve {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
