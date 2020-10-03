use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use crypto::signers::ecdsa;

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
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl From<EcCurve> for ecdsa::Curve {
  fn from(other: EcCurve) -> Self {
    match other {
      EcCurve::P256 => ecdsa::Curve::P256,
      EcCurve::P384 => ecdsa::Curve::P384,
      EcCurve::P521 => ecdsa::Curve::P521,
      EcCurve::Secp256K1 => ecdsa::Curve::K256,
    }
  }
}
