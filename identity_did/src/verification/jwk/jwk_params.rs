use super::key_type::JwkType;

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

impl JwkParams {
  pub const fn new(kty: JwkType) -> Self {
    match kty {
      JwkType::Ec => Self::Ec(JwkParamsEc::new()),
      JwkType::Rsa => Self::Rsa(JwkParamsRsa::new()),
      JwkType::Oct => Self::Oct(JwkParamsOct::new()),
      JwkType::Okp => Self::Okp(JwkParamsOkp::new()),
    }
  }

  pub const fn kty(&self) -> JwkType {
    match self {
      Self::Ec(inner) => inner.kty(),
      Self::Rsa(inner) => inner.kty(),
      Self::Oct(inner) => inner.kty(),
      Self::Okp(inner) => inner.kty(),
    }
  }

  pub fn to_public(&self) -> Self {
    match self {
      Self::Ec(inner) => Self::Ec(inner.to_public()),
      Self::Rsa(inner) => Self::Rsa(inner.to_public()),
      Self::Oct(inner) => Self::Oct(inner.to_public()),
      Self::Okp(inner) => Self::Okp(inner.to_public()),
    }
  }
}

// =============================================================================
// Jwk Params Ec
// =============================================================================

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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub d: Option<String>, // ECC Private Key
}

impl JwkParamsEc {
  pub const fn new() -> Self {
    Self {
      crv: String::new(),
      x: String::new(),
      y: String::new(),
      d: None,
    }
  }

  pub const fn kty(&self) -> JwkType {
    JwkType::Ec
  }

  pub fn to_public(&self) -> Self {
    Self {
      crv: self.crv.clone(),
      x: self.x.clone(),
      y: self.y.clone(),
      d: None,
    }
  }
}

impl From<JwkParamsEc> for JwkParams {
  fn from(other: JwkParamsEc) -> Self {
    Self::Ec(other)
  }
}

// =============================================================================
// Jwk Params Rsa
// =============================================================================

/// Parameters for RSA Keys.
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwkParamsRsa {
  /// The modulus value for the RSA public key as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.1.1)
  pub n: String, // Modulus
  /// The exponent value for the RSA public key as a base64urlUInt-encoded
  /// value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.1.2)
  pub e: String, // Exponent
  /// The private exponent value for the RSA private key as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.1)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub d: Option<String>, // Private Exponent
  /// The first prime factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.2)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub p: Option<String>, // First Prime Factor
  /// The second prime factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub q: Option<String>, // Second Prime Factor
  /// The Chinese Remainder Theorem (CRT) exponent of the first factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dp: Option<String>, // First Factor CRT Exponent
  /// The CRT exponent of the second factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dq: Option<String>, // Second Factor CRT Exponent
  /// The CRT coefficient of the second factor as a base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub qi: Option<String>, // First CRT Coefficient
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
  pub r: String, // Prime Factor
  /// The CRT exponent of the corresponding prime factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7.2)
  pub d: String, // Factor CRT Exponent
  /// The CRT coefficient of the corresponding prime factor as a
  /// base64urlUInt-encoded value.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7518#section-6.3.2.7.3)
  pub t: String, // Factor CRT Coefficient
}

impl JwkParamsRsa {
  pub const fn new() -> Self {
    Self {
      n: String::new(),
      e: String::new(),
      d: None,
      p: None,
      q: None,
      dp: None,
      dq: None,
      qi: None,
      oth: None,
    }
  }

  pub const fn kty(&self) -> JwkType {
    JwkType::Rsa
  }

  pub fn to_public(&self) -> Self {
    Self {
      n: self.n.clone(),
      e: self.e.clone(),
      d: None,
      p: None,
      q: None,
      dp: None,
      dq: None,
      qi: None,
      oth: None,
    }
  }
}

impl From<JwkParamsRsa> for JwkParams {
  fn from(other: JwkParamsRsa) -> Self {
    Self::Rsa(other)
  }
}

// =============================================================================
// Jwk Params Oct
// =============================================================================

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

impl JwkParamsOct {
  pub const fn new() -> Self {
    Self { k: String::new() }
  }

  pub const fn kty(&self) -> JwkType {
    JwkType::Oct
  }

  pub fn to_public(&self) -> Self {
    Self { k: self.k.clone() }
  }
}

impl From<JwkParamsOct> for JwkParams {
  fn from(other: JwkParamsOct) -> Self {
    Self::Oct(other)
  }
}

// =============================================================================
// Jwk Params Okp
// =============================================================================

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

impl JwkParamsOkp {
  pub const fn new() -> Self {
    Self {
      crv: String::new(),
      x: String::new(),
      d: None,
    }
  }

  pub const fn kty(&self) -> JwkType {
    JwkType::Okp
  }

  pub fn to_public(&self) -> Self {
    Self {
      crv: self.crv.clone(),
      x: self.x.clone(),
      d: None,
    }
  }
}

impl From<JwkParamsOkp> for JwkParams {
  fn from(other: JwkParamsOkp) -> Self {
    Self::Okp(other)
  }
}
