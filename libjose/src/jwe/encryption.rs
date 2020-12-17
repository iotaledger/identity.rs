use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::crypto::ciphers::aes;
use crate::crypto::ciphers::chacha;
use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

macro_rules! check_key {
  ($this:expr, $value:expr) => {
    if $value.len() != $this.key_len() {
      return Err(Error::EncError("Invalid Content Encryption Key Length"));
    }
  };
}

macro_rules! check_iv {
  ($this:expr, $value:expr) => {
    if $value.len() != $this.iv_len() {
      return Err(Error::EncError("Invalid Initialization Vector Length"));
    }
  };
}

macro_rules! require_tag {
  ($tag:expr) => {
    $tag.ok_or(Error::EncError("Decryption Tag is Required"))
  };
}

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

  pub fn key_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 | Self::A128GCM => aes::key_len_AES_GCM_128(),
      Self::A192CBC_HS384 | Self::A192GCM => aes::key_len_AES_GCM_192(),
      Self::A256CBC_HS512 | Self::A256GCM => aes::key_len_AES_GCM_256(),
      Self::C20P => chacha::key_len_C20P(),
      Self::XC20P => chacha::key_len_XC20P(),
    }
  }

  pub fn iv_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 | Self::A128GCM => aes::iv_len_AES_GCM_128(),
      Self::A192CBC_HS384 | Self::A192GCM => aes::iv_len_AES_GCM_192(),
      Self::A256CBC_HS512 | Self::A256GCM => aes::iv_len_AES_GCM_256(),
      Self::C20P => chacha::iv_len_C20P(),
      Self::XC20P => chacha::iv_len_XC20P(),
    }
  }

  pub fn encrypt(
    &self,
    plaintext: &[u8],
    key: &[u8],
    iv: &[u8],
    aad: &[u8],
  ) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
    match self {
      Self::A128CBC_HS256 => {
        todo!("JweEncryption::encrypt(A128CBC_HS256)")
      }
      Self::A192CBC_HS384 => {
        todo!("JweEncryption::encrypt(A192CBC_HS384)")
      }
      Self::A256CBC_HS512 => {
        todo!("JweEncryption::encrypt(A256CBC_HS512)")
      }
      Self::A128GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::encrypt_AES_GCM_128(plaintext, key, iv, aad).map(|(c, t)| (c, Some(t)))
      }
      Self::A192GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::encrypt_AES_GCM_192(plaintext, key, iv, aad).map(|(c, t)| (c, Some(t)))
      }
      Self::A256GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::encrypt_AES_GCM_256(plaintext, key, iv, aad).map(|(c, t)| (c, Some(t)))
      }
      Self::C20P => {
        check_key!(self, key);
        check_iv!(self, iv);

        chacha::encrypt_C20P(plaintext, key, iv, aad).map(|(c, t)| (c, Some(t)))
      }
      Self::XC20P => {
        check_key!(self, key);
        check_iv!(self, iv);

        chacha::encrypt_XC20P(plaintext, key, iv, aad).map(|(c, t)| (c, Some(t)))
      }
    }
  }

  pub fn decrypt(
    &self,
    ciphertext: &[u8],
    key: &[u8],
    iv: &[u8],
    aad: &[u8],
    tag: Option<&[u8]>,
  ) -> Result<Vec<u8>> {
    match self {
      Self::A128CBC_HS256 => {
        todo!("JweEncryption::decrypt(A128CBC_HS256)")
      }
      Self::A192CBC_HS384 => {
        todo!("JweEncryption::decrypt(A192CBC_HS384)")
      }
      Self::A256CBC_HS512 => {
        todo!("JweEncryption::decrypt(A256CBC_HS512)")
      }
      Self::A128GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::decrypt_AES_GCM_128(ciphertext, key, iv, aad, require_tag!(tag)?)
      }
      Self::A192GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::decrypt_AES_GCM_192(ciphertext, key, iv, aad, require_tag!(tag)?)
      }
      Self::A256GCM => {
        check_key!(self, key);
        check_iv!(self, iv);

        aes::decrypt_AES_GCM_256(ciphertext, key, iv, aad, require_tag!(tag)?)
      }
      Self::C20P => {
        check_key!(self, key);
        check_iv!(self, iv);

        chacha::decrypt_C20P(ciphertext, key, iv, aad, require_tag!(tag)?)
      }
      Self::XC20P => {
        check_key!(self, key);
        check_iv!(self, iv);

        chacha::decrypt_XC20P(ciphertext, key, iv, aad, require_tag!(tag)?)
      }
    }
  }
}

impl Display for JweEncryption {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}
