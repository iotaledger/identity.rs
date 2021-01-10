use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use crypto::ciphers::aes::AES_128_CBC_HMAC_SHA_256;
use crypto::ciphers::aes::AES_128_GCM;
use crypto::ciphers::aes::AES_192_CBC_HMAC_SHA_384;
use crypto::ciphers::aes::AES_192_GCM;
use crypto::ciphers::aes::AES_256_CBC_HMAC_SHA_512;
use crypto::ciphers::aes::AES_256_GCM;
use crypto::ciphers::chacha::CHACHA20_POLY1305;
use crypto::ciphers::chacha::XCHACHA20_POLY1305;

/// Supported algorithms for the JSON Web Encryption `enc` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms)
///
/// [ChaCha20-Poly1305 (draft)](https://tools.ietf.org/html/draft-amringer-jose-chacha-02)
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
  /// ChaCha20-Poly1305.
  C20P,
  /// ChaCha20-Poly1305.
  XC20P,
}

impl JweEncryption {
  pub const ALL: &'static [Self] = &[
    Self::A128CBC_HS256,
    Self::A192CBC_HS384,
    Self::A256CBC_HS512,
    Self::A128GCM,
    Self::A192GCM,
    Self::A256GCM,
    Self::C20P,
    Self::XC20P,
  ];

  /// Returns the JWE "enc" claim as a `str` slice.
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

  pub const fn key_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 => AES_128_CBC_HMAC_SHA_256::KEY_LENGTH,
      Self::A192CBC_HS384 => AES_192_CBC_HMAC_SHA_384::KEY_LENGTH,
      Self::A256CBC_HS512 => AES_256_CBC_HMAC_SHA_512::KEY_LENGTH,
      Self::A128GCM => AES_128_GCM::KEY_LENGTH,
      Self::A192GCM => AES_192_GCM::KEY_LENGTH,
      Self::A256GCM => AES_256_GCM::KEY_LENGTH,
      Self::C20P => CHACHA20_POLY1305::KEY_LENGTH,
      Self::XC20P => XCHACHA20_POLY1305::KEY_LENGTH,
    }
  }

  pub const fn iv_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 => AES_128_CBC_HMAC_SHA_256::IV_LENGTH,
      Self::A192CBC_HS384 => AES_192_CBC_HMAC_SHA_384::IV_LENGTH,
      Self::A256CBC_HS512 => AES_256_CBC_HMAC_SHA_512::IV_LENGTH,
      Self::A128GCM => AES_128_GCM::IV_LENGTH,
      Self::A192GCM => AES_192_GCM::IV_LENGTH,
      Self::A256GCM => AES_256_GCM::IV_LENGTH,
      Self::C20P => CHACHA20_POLY1305::IV_LENGTH,
      Self::XC20P => XCHACHA20_POLY1305::IV_LENGTH,
    }
  }
}

impl Display for JweEncryption {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_str(self.name())
  }
}
