use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported algorithms for the JSON Web Encryption `enc` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms)
///
/// [ChaCha20-Poly1305 (draft)](https://tools.ietf.org/html/draft-amringer-jose-chacha-01)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum JweEncryption {
  /// AES_128_CBC_HMAC_SHA_256 authenticated encryption algorithm.
  #[serde(rename = "A128CBC-HS256")]
  A128CBC_HS256,
  /// AES_192_CBC_HMAC_SHA_384 authenticated encryption algorithm.
  #[serde(rename = "A192CBC-HS384")]
  A192CBC_HS384,
  /// AES_256_CBC_HMAC_SHA_512 authenticated encryption algorithm.
  #[serde(rename = "A256CBC-HS512")]
  A256CBC_HS512,
  /// AES GCM using 128-bit key.
  A128GCM,
  /// AES GCM using 192-bit key.
  A192GCM,
  /// AES GCM using 256-bit key.
  A256GCM,
  /// AEAD_CHACHA20_POLY1305 using 96-bit key.
  C20P,
  /// AEAD_XCHACHA20_POLY1305 using 192-bit key.
  XC20P,
}

impl JweEncryption {
  pub const fn name(self) -> &'static str {
    match self {
      Self::A128CBC_HS256 => "A128CBC-HS256",
      Self::A192CBC_HS384 => "A192CBC-HS384",
      Self::A256CBC_HS512 => "A256CBC-HS512",
      Self::A128GCM => "A128GCM",
      Self::A192GCM => "A192GCM",
      Self::A256GCM => "A256GCM",
      Self::C20P => "C20P",
      Self::XC20P => "XC20P",
    }
  }
}

impl Default for JweEncryption {
  fn default() -> Self {
    Self::A128CBC_HS256
  }
}

impl Display for JweEncryption {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_fmt(format_args!("{}", self.name()))
  }
}
