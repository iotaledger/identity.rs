use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use crypto::ciphers::aes::AES_128_GCM;
use crypto::ciphers::aes::AES_192_GCM;
use crypto::ciphers::aes::AES_256_GCM;
use crypto::ciphers::chacha::CHACHA20_POLY1305;
use crypto::ciphers::chacha::XCHACHA20_POLY1305;

use crate::error::Result;
use crate::lib::*;

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

  pub const fn key_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 | Self::A128GCM => AES_128_GCM::KEY_LENGTH,
      Self::A192CBC_HS384 | Self::A192GCM => AES_192_GCM::KEY_LENGTH,
      Self::A256CBC_HS512 | Self::A256GCM => AES_256_GCM::KEY_LENGTH,
      Self::C20P => CHACHA20_POLY1305::KEY_LENGTH,
      Self::XC20P => XCHACHA20_POLY1305::KEY_LENGTH,
    }
  }

  pub const fn iv_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 | Self::A128GCM => AES_128_GCM::IV_LENGTH,
      Self::A192CBC_HS384 | Self::A192GCM => AES_192_GCM::IV_LENGTH,
      Self::A256CBC_HS512 | Self::A256GCM => AES_256_GCM::IV_LENGTH,
      Self::C20P => CHACHA20_POLY1305::IV_LENGTH,
      Self::XC20P => XCHACHA20_POLY1305::IV_LENGTH,
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
        let key: _ = to_bytes!(key, AES_128_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_128_GCM::IV_LENGTH, "IV")?;

        let mut ciphertext: Vec<u8> = plaintext.to_vec();
        let mut tag: [u8; AES_128_GCM::TAG_LENGTH] = [0; AES_128_GCM::TAG_LENGTH];

        AES_128_GCM::encrypt(&key, &iv, aad, plaintext, &mut ciphertext, &mut tag)?;

        Ok((ciphertext, Some(tag.to_vec())))
      }
      Self::A192GCM => {
        let key: _ = to_bytes!(key, AES_192_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_192_GCM::IV_LENGTH, "IV")?;

        let mut ciphertext: Vec<u8> = plaintext.to_vec();
        let mut tag: [u8; AES_192_GCM::TAG_LENGTH] = [0; AES_192_GCM::TAG_LENGTH];

        AES_192_GCM::encrypt(&key, &iv, aad, plaintext, &mut ciphertext, &mut tag)?;

        Ok((ciphertext, Some(tag.to_vec())))
      }
      Self::A256GCM => {
        let key: _ = to_bytes!(key, AES_256_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_256_GCM::IV_LENGTH, "IV")?;

        let mut ciphertext: Vec<u8> = plaintext.to_vec();
        let mut tag: [u8; AES_256_GCM::TAG_LENGTH] = [0; AES_256_GCM::TAG_LENGTH];

        AES_256_GCM::encrypt(&key, &iv, aad, plaintext, &mut ciphertext, &mut tag)?;

        Ok((ciphertext, Some(tag.to_vec())))
      }
      Self::C20P => {
        let key: _ = to_bytes!(key, CHACHA20_POLY1305::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, CHACHA20_POLY1305::IV_LENGTH, "IV")?;

        let mut ciphertext: Vec<u8> = plaintext.to_vec();
        let mut tag: [u8; CHACHA20_POLY1305::TAG_LENGTH] = [0; CHACHA20_POLY1305::TAG_LENGTH];

        CHACHA20_POLY1305::encrypt(&key, &iv, aad, plaintext, &mut ciphertext, &mut tag)?;

        Ok((ciphertext, Some(tag.to_vec())))
      }
      Self::XC20P => {
        let key: _ = to_bytes!(key, XCHACHA20_POLY1305::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, XCHACHA20_POLY1305::IV_LENGTH, "IV")?;

        let mut ciphertext: Vec<u8> = plaintext.to_vec();
        let mut tag: [u8; XCHACHA20_POLY1305::TAG_LENGTH] = [0; XCHACHA20_POLY1305::TAG_LENGTH];

        XCHACHA20_POLY1305::encrypt(&key, &iv, aad, plaintext, &mut ciphertext, &mut tag)?;

        Ok((ciphertext, Some(tag.to_vec())))
      }
    }
  }

  pub fn decrypt(
    &self,
    ciphertext: &[u8],
    key: &[u8],
    iv: &[u8],
    aad: &[u8],
    tag: &[u8],
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
        let key: _ = to_bytes!(key, AES_128_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_128_GCM::IV_LENGTH, "IV")?;
        let tag: _ = to_bytes!(tag, AES_128_GCM::TAG_LENGTH, "Tag")?;

        let mut plaintext: Vec<u8> = ciphertext.to_vec();

        AES_128_GCM::decrypt(&key, &iv, aad, &tag, ciphertext, &mut plaintext)?;

        Ok(plaintext)
      }
      Self::A192GCM => {
        let key: _ = to_bytes!(key, AES_192_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_192_GCM::IV_LENGTH, "IV")?;
        let tag: _ = to_bytes!(tag, AES_192_GCM::TAG_LENGTH, "Tag")?;

        let mut plaintext: Vec<u8> = ciphertext.to_vec();

        AES_192_GCM::decrypt(&key, &iv, aad, &tag, ciphertext, &mut plaintext)?;

        Ok(plaintext)
      }
      Self::A256GCM => {
        let key: _ = to_bytes!(key, AES_256_GCM::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, AES_256_GCM::IV_LENGTH, "IV")?;
        let tag: _ = to_bytes!(tag, AES_256_GCM::TAG_LENGTH, "Tag")?;

        let mut plaintext: Vec<u8> = ciphertext.to_vec();

        AES_256_GCM::decrypt(&key, &iv, aad, &tag, ciphertext, &mut plaintext)?;

        Ok(plaintext)
      }
      Self::C20P => {
        let key: _ = to_bytes!(key, CHACHA20_POLY1305::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, CHACHA20_POLY1305::IV_LENGTH, "IV")?;
        let tag: _ = to_bytes!(tag, CHACHA20_POLY1305::TAG_LENGTH, "Tag")?;

        let mut plaintext: Vec<u8> = ciphertext.to_vec();

        CHACHA20_POLY1305::decrypt(&key, &iv, aad, &tag, ciphertext, &mut plaintext)?;

        Ok(plaintext)
      }
      Self::XC20P => {
        let key: _ = to_bytes!(key, XCHACHA20_POLY1305::KEY_LENGTH, "CEK")?;
        let iv: _ = to_bytes!(iv, XCHACHA20_POLY1305::IV_LENGTH, "IV")?;
        let tag: _ = to_bytes!(tag, XCHACHA20_POLY1305::TAG_LENGTH, "Tag")?;

        let mut plaintext: Vec<u8> = ciphertext.to_vec();

        XCHACHA20_POLY1305::decrypt(&key, &iv, aad, &tag, ciphertext, &mut plaintext)?;

        Ok(plaintext)
      }
    }
  }
}

impl Display for JweEncryption {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str(self.name())
  }
}
