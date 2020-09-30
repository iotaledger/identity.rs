use alloc::string::String;
use alloc::vec::Vec;

use crate::jwk::JwkType;

type BigUint = String; // TODO

/// Algorithm-specific parameters for JSON Web Keys.
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JwkParams {
  Ec(JwkParamsEc),
  Rsa(JwkParamsRsa),
  Oct(JwkParamsOct),
  Okp(JwkParamsOkp),
}

impl From<JwkParamsEc> for JwkParams {
  fn from(other: JwkParamsEc) -> Self {
    Self::Ec(other)
  }
}

impl From<JwkParamsRsa> for JwkParams {
  fn from(other: JwkParamsRsa) -> Self {
    Self::Rsa(other)
  }
}

impl From<JwkParamsOct> for JwkParams {
  fn from(other: JwkParamsOct) -> Self {
    Self::Oct(other)
  }
}

impl From<JwkParamsOkp> for JwkParams {
  fn from(other: JwkParamsOkp) -> Self {
    Self::Okp(other)
  }
}

/// Parameters for Elliptic Curve Keys.
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6.2)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsEc {
  /// Identifies the cryptographic curve used with the key.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.2.1.1)
  pub crv: String, // Curve
  /// The `x` coordinate for the Elliptic Curve point as a base64url-encoded
  /// value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.2.1.2)
  pub x: String, // X Coordinate
  /// The `y` coordinate for the Elliptic Curve point as a base64url-encoded
  /// value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.2.1.3)
  pub y: String, // Y Coordinate
  /// The Elliptic Curve private key as a base64url-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.2.2.1)
  pub d: Option<String>, // ECC Private Key
}

/// Parameters for RSA Keys.
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsRsa {
  /// The modulus value for the RSA public key as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.1.1)
  pub n: BigUint, // Modulus
  /// The exponent value for the RSA public key as a base64urlUInt-encoded
  /// value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.1.2)
  pub e: BigUint, // Exponent
  /// The private exponent value for the RSA private key as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub d: Option<BigUint>, // Private Exponent
  /// The first prime factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub p: Option<BigUint>, // First Prime Factor
  /// The second prime factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub q: Option<BigUint>, // Second Prime Factor
  /// The Chinese Remainder Theorem (CRT) exponent of the first factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dp: Option<BigUint>, // First Factor CRT Exponent
  /// The CRT exponent of the second factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dq: Option<BigUint>, // Second Factor CRT Exponent
  /// The CRT coefficient of the second factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub qi: Option<BigUint>, // First CRT Coefficient
  /// An array of information about any third and subsequent primes, should they
  /// exist.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub oth: Option<Vec<JwkParamsRsaPrime>>, // Other Primes Info
}

/// Parameters for RSA Primes
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsRsaPrime {
  /// The value of a subsequent prime factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7.1)
  pub r: BigUint, // Prime Factor
  /// The CRT exponent of the corresponding prime factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7.2)
  pub d: BigUint, // Factor CRT Exponent
  /// The CRT coefficient of the corresponding prime factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7.3)
  pub t: BigUint, // Factor CRT Coefficient
}

/// Parameters for Symmetric Keys.
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6.4)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsOct {
  /// The symmetric key as a base64url-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.4.1)
  pub k: String, // Key Value
}

/// Parameters for Octet Key Pairs.
///
/// [More Info](https://tools.ietf.org/html/rfc8037#section-2)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsOkp {
  /// The subtype of the key pair.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8037#section-2)
  pub crv: String, // Key SubType
  /// The public key as a base64url-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8037#section-2)
  pub x: String, // Public Key
  /// The private key as a base64url-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8037#section-2)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub d: Option<String>, // Private Key
}
