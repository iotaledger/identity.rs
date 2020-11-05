// Copyright 2020 IOTA Stiftung
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use hmac_crate::crypto_mac::generic_array::typenum::Unsigned as _;
use hmac_crate::digest::BlockInput;
use hmac_crate::digest::FixedOutput;
use hmac_crate::digest::Reset;
use hmac_crate::digest::Update;
use hmac_crate::Hmac;
use hmac_crate::Mac as _;
use hmac_crate::NewMac as _;

use crate::crypto::error::Error;
use crate::crypto::error::Result;

const HMAC_ERR: Error = Error::SignatureError { alg: "hmac" };

// TODO: Move to a digest module
// - HashAlgorithm::Sha256::output_size();
pub const SHA256_OUTPUT_SIZE: usize = 256 / 8; // 32
pub const SHA384_OUTPUT_SIZE: usize = 384 / 8; // 48
pub const SHA512_OUTPUT_SIZE: usize = 512 / 8; // 64

// TODO: Maybe move to a digest module
// - HashAlgorithm::Sha256::new_output();
pub type Hmac256Signature = [u8; SHA256_OUTPUT_SIZE];
pub type Hmac384Signature = [u8; SHA384_OUTPUT_SIZE];
pub type Hmac512Signature = [u8; SHA512_OUTPUT_SIZE];

pub fn sign<D, K, M>(key: K, message: M, out: &mut [u8]) -> Result<()>
where
  D: Clone + Default + BlockInput + FixedOutput + Reset + Update,
  K: AsRef<[u8]>,
  M: AsRef<[u8]>,
{
  if out.len() != D::OutputSize::to_usize() {
    return Err(Error::BufferSize {
      what: "output buffer",
      needs: D::OutputSize::to_usize(),
      has: out.len(),
    });
  }

  let mut mac: Hmac<D> = new_mac(key)?;

  mac.update(message.as_ref());
  out.copy_from_slice(&mac.finalize().into_bytes());

  Ok(())
}

pub fn verify<D, K, M, S>(key: K, message: M, signature: S) -> Result<()>
where
  D: Clone + Default + BlockInput + FixedOutput + Reset + Update,
  K: AsRef<[u8]>,
  M: AsRef<[u8]>,
  S: AsRef<[u8]>,
{
  let mut mac: Hmac<D> = new_mac(key)?;

  mac.update(message.as_ref());
  mac.verify(signature.as_ref()).map_err(|_| HMAC_ERR)?;

  Ok(())
}

pub fn sign_sha256(key: impl AsRef<[u8]>, message: impl AsRef<[u8]>) -> Result<Hmac256Signature> {
  let mut out: Hmac256Signature = [0; SHA256_OUTPUT_SIZE];
  sign::<sha2::Sha256, _, _>(key, message, &mut out)?;
  Ok(out)
}

pub fn verify_sha256(
  key: impl AsRef<[u8]>,
  message: impl AsRef<[u8]>,
  signature: impl AsRef<[u8]>,
) -> Result<()> {
  verify::<sha2::Sha256, _, _, _>(key, message, signature)
}

pub fn sign_sha384(key: impl AsRef<[u8]>, message: impl AsRef<[u8]>) -> Result<Hmac384Signature> {
  let mut out: Hmac384Signature = [0; SHA384_OUTPUT_SIZE];
  sign::<sha2::Sha384, _, _>(key, message, &mut out)?;
  Ok(out)
}

pub fn verify_sha384(
  key: impl AsRef<[u8]>,
  message: impl AsRef<[u8]>,
  signature: impl AsRef<[u8]>,
) -> Result<()> {
  verify::<sha2::Sha384, _, _, _>(key, message, signature)
}

pub fn sign_sha512(key: impl AsRef<[u8]>, message: impl AsRef<[u8]>) -> Result<Hmac512Signature> {
  let mut out: Hmac512Signature = [0; SHA512_OUTPUT_SIZE];
  sign::<sha2::Sha512, _, _>(key, message, &mut out)?;
  Ok(out)
}

pub fn verify_sha512(
  key: impl AsRef<[u8]>,
  message: impl AsRef<[u8]>,
  signature: impl AsRef<[u8]>,
) -> Result<()> {
  verify::<sha2::Sha512, _, _, _>(key, message, signature)
}

// A helper for creating a new MAC instance
fn new_mac<D, K>(key: K) -> Result<Hmac<D>>
where
  D: Clone + Default + BlockInput + FixedOutput + Reset + Update,
  K: AsRef<[u8]>,
{
  Hmac::new_varkey(key.as_ref()).map_err(|_| HMAC_ERR)
}
