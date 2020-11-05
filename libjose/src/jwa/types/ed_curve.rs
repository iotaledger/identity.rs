use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::crypto::signers::eddsa;

/// Supported Elliptic Curves.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-elliptic-curve)
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
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<EdCurve> for eddsa::Curve {
  fn from(other: EdCurve) -> Self {
    match other {
      EdCurve::Ed25519 => eddsa::Curve::Ed25519,
      EdCurve::Ed448 => eddsa::Curve::Ed448,
    }
  }
}
