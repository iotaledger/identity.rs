#![allow(non_snake_case)]

use aes_gcm::aead::generic_array::typenum;
use aes_gcm::aead::generic_array::typenum::Unsigned as _;
use aes_gcm::aead::AeadInPlace;
use aes_gcm::aead::NewAead;
use aes_gcm::aes::Aes192;
use aes_gcm::Aes128Gcm;
use aes_gcm::Aes256Gcm;
use aes_gcm::AesGcm;

use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

type Aes192Gcm = AesGcm<Aes192, typenum::U12>;

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

pub fn key_len_AES_GCM_128() -> usize {
  <Aes128Gcm as NewAead>::KeySize::to_usize()
}

pub fn key_len_AES_GCM_192() -> usize {
  <Aes192Gcm as NewAead>::KeySize::to_usize()
}

pub fn key_len_AES_GCM_256() -> usize {
  <Aes256Gcm as NewAead>::KeySize::to_usize()
}

pub fn iv_len_AES_GCM_128() -> usize {
  <Aes128Gcm as AeadInPlace>::NonceSize::to_usize()
}

pub fn iv_len_AES_GCM_192() -> usize {
  <Aes192Gcm as AeadInPlace>::NonceSize::to_usize()
}

pub fn iv_len_AES_GCM_256() -> usize {
  <Aes256Gcm as AeadInPlace>::NonceSize::to_usize()
}

pub fn encrypt_AES_GCM_128(
  plaintext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  encrypt!(Aes128Gcm, plaintext, key, iv, aad)
}

pub fn encrypt_AES_GCM_192(
  plaintext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  encrypt!(Aes192Gcm, plaintext, key, iv, aad)
}

pub fn encrypt_AES_GCM_256(
  plaintext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  encrypt!(Aes256Gcm, plaintext, key, iv, aad)
}

pub fn decrypt_AES_GCM_128(
  ciphertext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
) -> Result<Vec<u8>> {
  decrypt!(Aes128Gcm, ciphertext, key, iv, aad, tag)
}

pub fn decrypt_AES_GCM_192(
  ciphertext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
) -> Result<Vec<u8>> {
  decrypt!(Aes192Gcm, ciphertext, key, iv, aad, tag)
}

pub fn decrypt_AES_GCM_256(
  ciphertext: &[u8],
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
) -> Result<Vec<u8>> {
  decrypt!(Aes256Gcm, ciphertext, key, iv, aad, tag)
}
