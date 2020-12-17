#![allow(non_snake_case)]

use hmac::crypto_mac::generic_array::typenum::Unsigned as _;
use hmac::Hmac;
use hmac::Mac as _;
use hmac::NewMac as _;

use crate::crypto::digest::Digest;
use crate::crypto::digest::SHA2_256;
use crate::crypto::digest::SHA2_384;
use crate::crypto::digest::SHA2_512;
use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

macro_rules! sign {
  ($impl:ident, $key:expr, $message:expr) => {{
    let mut mac: Hmac<$impl> =
      Hmac::new_varkey($key).map_err(|_| Error::KeyError("Invalid HMAC Key"))?;

    mac.update($message);

    Ok(mac.finalize().into_bytes().to_vec())
  }};
}

macro_rules! verify {
  ($impl:ident, $key:expr, $message:expr, $signature:expr) => {{
    let mut mac: Hmac<$impl> =
      Hmac::new_varkey($key).map_err(|_| Error::KeyError("Invalid HMAC Key"))?;

    mac.update($message);
    mac
      .verify($signature)
      .map_err(|_| Error::SigError("Invalid HMAC Signature"))?;

    Ok(())
  }};
}

pub fn key_len_HMAC_SHA256() -> usize {
  <SHA2_256 as Digest>::OutputSize::to_usize()
}

pub fn key_len_HMAC_SHA384() -> usize {
  <SHA2_384 as Digest>::OutputSize::to_usize()
}

pub fn key_len_HMAC_SHA512() -> usize {
  <SHA2_512 as Digest>::OutputSize::to_usize()
}

pub fn sign_HMAC_SHA256(key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
  sign!(SHA2_256, key, message)
}

pub fn sign_HMAC_SHA384(key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
  sign!(SHA2_384, key, message)
}

pub fn sign_HMAC_SHA512(key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
  sign!(SHA2_512, key, message)
}

pub fn verify_HMAC_SHA256(key: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
  verify!(SHA2_256, key, message, signature)
}

pub fn verify_HMAC_SHA384(key: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
  verify!(SHA2_384, key, message, signature)
}

pub fn verify_HMAC_SHA512(key: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
  verify!(SHA2_512, key, message, signature)
}
