use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported algorithms for the JSON Web Encryption `alg` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms)
///
/// [ChaCha20-Poly1305 (draft)](https://tools.ietf.org/html/draft-amringer-jose-chacha-01)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum JweAlgorithm {
  /// RSAES-PKCS1-v1_5
  #[serde(rename = "RSA1_5")]
  RSA1_5,
  /// RSAES OAEP using default parameters
  #[serde(rename = "RSA-OAEP")]
  RSA_OAEP,
  /// RSAES OAEP using SHA-256 and MGF1 with SHA-256.
  #[serde(rename = "RSA-OAEP-256")]
  RSA_OAEP_256,
  /// RSA-OAEP using SHA-384 and MGF1 with SHA-384.
  #[serde(rename = "RSA-OAEP-384")]
  RSA_OAEP_384,
  /// RSA-OAEP using SHA-512 and MGF1 with SHA-512.
  #[serde(rename = "RSA-OAEP-512")]
  RSA_OAEP_512,
  /// AES Key Wrap with default initial value using 128-bit key.
  #[serde(rename = "A128KW")]
  A128KW,
  /// AES Key Wrap with default initial value using 192-bit key.
  #[serde(rename = "A192KW")]
  A192KW,
  /// AES Key Wrap with default initial value using 256-bit key.
  #[serde(rename = "A256KW")]
  A256KW,
  /// Direct use of a shared symmetric key as the CEK.
  #[serde(rename = "dir")]
  DIR,
  /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat
  /// KDF.
  #[serde(rename = "ECDH-ES")]
  ECDH_ES,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A128KW".
  #[serde(rename = "ECDH-ES+A128KW")]
  ECDH_ES_A128KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A192KW".
  #[serde(rename = "ECDH-ES+A192KW")]
  ECDH_ES_A192KW,
  /// ECDH-ES using Concat KDF and CEK  wrapped with "A256KW".
  #[serde(rename = "ECDH-ES+A256KW")]
  ECDH_ES_A256KW,
  /// Key wrapping with AES GCM using 128-bit key.
  A128GCMKW,
  /// Key wrapping with AES GCM using 192-bit key.
  A192GCMKW,
  /// Key wrapping with AES GCM using 256-bit key.
  A256GCMKW,
  /// PBES2 with HMAC SHA-256 and "A128KW" wrapping.
  #[serde(rename = "PBES2-HS256+A128KW")]
  PBES2_HS256_A128KW,
  /// PBES2 with HMAC SHA-384 and "A192KW" wrapping.
  #[serde(rename = "PBES2-HS384+A192KW")]
  PBES2_HS384_A192KW,
  /// PBES2 with HMAC SHA-512 and "A256KW" wrapping.
  #[serde(rename = "PBES2-HS512+A256KW")]
  PBES2_HS512_A256KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with C20PKW.
  #[serde(rename = "ECDH-ES+C20PKW")]
  ECDH_ES_C20PKW,
  /// ECDH-ES using Concat KDF and CEK wrapped with XC20PKW.
  #[serde(rename = "ECDH-ES+XC20PKW")]
  ECDH_ES_XC20PKW,
}

impl JweAlgorithm {
  pub const fn name(self) -> &'static str {
    match self {
      Self::RSA1_5 => "RSA1_5",
      Self::RSA_OAEP => "RSA-OAEP",
      Self::RSA_OAEP_256 => "RSA-OAEP-256",
      Self::RSA_OAEP_384 => "RSA-OAEP-384",
      Self::RSA_OAEP_512 => "RSA-OAEP-512",
      Self::A128KW => "A128KW",
      Self::A192KW => "A192KW",
      Self::A256KW => "A256KW",
      Self::DIR => "dir",
      Self::ECDH_ES => "ECDH-ES",
      Self::ECDH_ES_A128KW => "ECDH-ES+A128KW",
      Self::ECDH_ES_A192KW => "ECDH-ES+A192KW",
      Self::ECDH_ES_A256KW => "ECDH-ES+A256KW",
      Self::A128GCMKW => "A128GCMKW",
      Self::A192GCMKW => "A192GCMKW",
      Self::A256GCMKW => "A256GCMKW",
      Self::PBES2_HS256_A128KW => "PBES2-HS256+A128KW",
      Self::PBES2_HS384_A192KW => "PBES2-HS384+A192KW",
      Self::PBES2_HS512_A256KW => "PBES2-HS512+A256KW",
      Self::ECDH_ES_C20PKW => "ECDH-ES+C20PKW",
      Self::ECDH_ES_XC20PKW => "ECDH-ES+XC20PKW",
    }
  }
}

impl Default for JweAlgorithm {
  fn default() -> Self {
    Self::ECDH_ES
  }
}

impl Display for JweAlgorithm {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_fmt(format_args!("{}", self.name()))
  }
}
