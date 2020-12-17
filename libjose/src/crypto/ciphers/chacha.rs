#![allow(non_snake_case)]

use chacha20poly1305::aead::generic_array::typenum::Unsigned as _;
use chacha20poly1305::aead::AeadInPlace;
use chacha20poly1305::aead::NewAead;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::XChaCha20Poly1305;

use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

macro_rules! encrypt {
  ($impl:ident, $plaintext:expr, $key:expr, $iv:expr, $aad:expr) => {{
    let mut ciphertext: Vec<u8> = $plaintext.to_vec();

    let tag: _ = $impl::new($key.into())
      .encrypt_in_place_detached($iv.into(), $aad, &mut ciphertext)
      .map_err(|_| Error::EncError("Failed to Encrypt Plaintext"))?
      .to_vec();

    Ok((ciphertext, tag))
  }};
}

macro_rules! decrypt {
  ($impl:ident, $ciphertext:expr, $key:expr, $iv:expr, $aad:expr, $tag:expr) => {{
    let mut plaintext: Vec<u8> = $ciphertext.to_vec();

    $impl::new($key.into())
      .decrypt_in_place_detached($iv.into(), $aad, &mut plaintext, $tag.into())
      .map_err(|_| Error::EncError("Failed to Decrypt Plaintext"))?;

    Ok(plaintext)
  }};
}

pub fn key_len_C20P() -> usize {
  <ChaCha20Poly1305 as NewAead>::KeySize::to_usize()
}

pub fn key_len_XC20P() -> usize {
  <XChaCha20Poly1305 as NewAead>::KeySize::to_usize()
}

pub fn iv_len_C20P() -> usize {
  <ChaCha20Poly1305 as AeadInPlace>::NonceSize::to_usize()
}

pub fn iv_len_XC20P() -> usize {
  <XChaCha20Poly1305 as AeadInPlace>::NonceSize::to_usize()
}

pub fn encrypt_C20P(
  plaintext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  encrypt!(ChaCha20Poly1305, plaintext, key, iv, aad)
}

pub fn encrypt_XC20P(
  plaintext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  encrypt!(XChaCha20Poly1305, plaintext, key, iv, aad)
}

pub fn decrypt_C20P(
  ciphertext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
) -> Result<Vec<u8>> {
  decrypt!(ChaCha20Poly1305, ciphertext, key, iv, aad, tag)
}

pub fn decrypt_XC20P(
  ciphertext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
) -> Result<Vec<u8>> {
  decrypt!(XChaCha20Poly1305, ciphertext, key, iv, aad, tag)
}
