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

use alloc::vec::Vec;
pub use rsa_crate::BigUint;
use rsa_crate::Hash;
use rsa_crate::PaddingScheme;
use rsa_crate::PublicKey as _;
use rsa_crate::PublicKeyParts as _;
use rsa_crate::RSAPrivateKey;
use rsa_crate::RSAPublicKey;
use sha2::Digest as _;
use zeroize::Zeroize;

use crate::crypto::error::Error;
use crate::crypto::error::Result;
use crate::crypto::rand::CryptoRng;
use crate::crypto::rand::RngCore;

const RSA_ERR: Error = Error::SignatureError { alg: "rsa" };

// =============================================================================
// RSA Public Key
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKey(RSAPublicKey);

impl PublicKey {
  /// Creates a new `PublicKey` from primitive components.
  pub fn new(n: BigUint, e: BigUint) -> Result<Self> {
    RSAPublicKey::new(n, e).map_err(|_| RSA_ERR).map(Self)
  }

  /// Creates a `PublicKey` by parsing a PKCS#1/PKCS#8-encoded document.
  pub fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self> {
    Self::from_pkcs8(&slice).or_else(|_| Self::from_pkcs1(slice))
  }

  /// Creates a `PublicKey` by parsing a PKCS#1-encoded document.
  pub fn from_pkcs1(pkcs1: impl AsRef<[u8]>) -> Result<Self> {
    RSAPublicKey::from_pkcs1(pkcs1.as_ref())
      .map_err(|_| RSA_ERR)
      .map(Self)
  }

  /// Creates a `PublicKey` by parsing a PKCS#8-encoded document.
  pub fn from_pkcs8(pkcs8: impl AsRef<[u8]>) -> Result<Self> {
    RSAPublicKey::from_pkcs8(pkcs8.as_ref())
      .map_err(|_| RSA_ERR)
      .map(Self)
  }

  /// Returns the modulus of the `PublicKey`.
  pub fn n(&self) -> &BigUint {
    self.0.n()
  }

  /// Returns the public exponent of the `PublicKey`.
  pub fn e(&self) -> &BigUint {
    self.0.e()
  }

  /// Verifies an RSA signature using RSASSA-PKCS#1.5 padding and SHA-256.
  pub fn verify_pkcs1_sha256(
    &self,
    message: impl AsRef<[u8]>,
    signature: impl AsRef<[u8]>,
  ) -> Result<()> {
    self.verify(
      sha2::Sha256::digest(message.as_ref()),
      signature,
      pad_pkcs1_sha256(),
    )
  }

  /// Verifies an RSA signature using RSASSA-PKCS#1.5 padding and SHA-384.
  pub fn verify_pkcs1_sha384(
    &self,
    message: impl AsRef<[u8]>,
    signature: impl AsRef<[u8]>,
  ) -> Result<()> {
    self.verify(
      sha2::Sha384::digest(message.as_ref()),
      signature,
      pad_pkcs1_sha384(),
    )
  }

  /// Verifies an RSA signature using RSASSA-PKCS#1.5 padding and SHA-512.
  pub fn verify_pkcs1_sha512(
    &self,
    message: impl AsRef<[u8]>,
    signature: impl AsRef<[u8]>,
  ) -> Result<()> {
    self.verify(
      sha2::Sha512::digest(message.as_ref()),
      signature,
      pad_pkcs1_sha512(),
    )
  }

  /// Verifies an RSA signature using RSASSA-PSS padding and SHA-256.
  pub fn verify_pss_sha256<R, M, S>(&self, rng: R, message: M, signature: S) -> Result<()>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
    S: AsRef<[u8]>,
  {
    self.verify(
      sha2::Sha256::digest(message.as_ref()),
      signature,
      pad_pss_sha256(rng),
    )
  }

  /// Verifies an RSA signature using RSASSA-PSS padding and SHA-384.
  pub fn verify_pss_sha384<R, M, S>(&self, rng: R, message: M, signature: S) -> Result<()>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
    S: AsRef<[u8]>,
  {
    self.verify(
      sha2::Sha384::digest(message.as_ref()),
      signature,
      pad_pss_sha384(rng),
    )
  }

  /// Verifies an RSA signature using RSASSA-PSS padding and SHA-512.
  pub fn verify_pss_sha512<R, M, S>(&self, rng: R, message: M, signature: S) -> Result<()>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
    S: AsRef<[u8]>,
  {
    self.verify(
      sha2::Sha512::digest(message.as_ref()),
      signature,
      pad_pss_sha512(rng),
    )
  }

  /// Verifies an RSA signature using the specified padding algorithm.
  fn verify(
    &self,
    message: impl AsRef<[u8]>,
    signature: impl AsRef<[u8]>,
    padding: PaddingScheme,
  ) -> Result<()> {
    self
      .0
      .verify(padding, message.as_ref(), signature.as_ref())
      .map_err(|_| RSA_ERR)
  }
}

// =============================================================================
// RSA Private Key
// =============================================================================

#[derive(Clone)]
pub struct PrivateKey(RSAPrivateKey);

impl PrivateKey {
  /// Creates a new random `PrivateKey`.
  pub fn random<R>(rng: &mut R, bits: RsaBits) -> Result<Self>
  where
    R: RngCore + CryptoRng,
  {
    RSAPrivateKey::new(rng, bits.bits())
      .map_err(|_| RSA_ERR)
      .map(Self)
  }

  /// Creates a new `PrivateKey` from primitive components.
  pub fn new(n: BigUint, e: BigUint, d: BigUint, primes: Vec<BigUint>) -> Result<Self> {
    // Construct the RSA key
    let key: RSAPrivateKey = RSAPrivateKey::from_components(n, e, d, primes);

    // Ensure the key is well-formed.
    key.validate().map_err(|_| RSA_ERR)?;

    // Return the parsed key.
    Ok(Self(key))
  }

  /// Creates a `PrivateKey` by parsing a PKCS#1/PKCS#8-encoded document.
  pub fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self> {
    Self::from_pkcs8(&slice).or_else(|_| Self::from_pkcs1(slice))
  }

  /// Creates a `PrivateKey` by parsing a PKCS#1-encoded document.
  pub fn from_pkcs1(pkcs1: impl AsRef<[u8]>) -> Result<Self> {
    // Parse the private key from the input slice.
    let key: RSAPrivateKey = RSAPrivateKey::from_pkcs1(pkcs1.as_ref()).map_err(|_| RSA_ERR)?;

    // Ensure the key is well-formed.
    key.validate().map_err(|_| RSA_ERR)?;

    // Return the parsed key.
    Ok(Self(key))
  }

  /// Creates a `PrivateKey` by parsing a PKCS#8-encoded document.
  pub fn from_pkcs8(pkcs8: impl AsRef<[u8]>) -> Result<Self> {
    // Parse the private key from the input slice.
    let key: RSAPrivateKey = RSAPrivateKey::from_pkcs8(pkcs8.as_ref()).map_err(|_| RSA_ERR)?;

    // Ensure the key is well-formed.
    key.validate().map_err(|_| RSA_ERR)?;

    // Return the parsed key.
    Ok(Self(key))
  }

  /// Creates a `PublicKey` by cloning the public key components.
  pub fn public_key(&self) -> PublicKey {
    PublicKey(self.0.to_public_key())
  }

  /// Signs the given message using RSASSA-PKCS#1.5 padding and SHA-256.
  pub fn sign_pkcs1_sha256(&self, message: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    self.sign(sha2::Sha256::digest(message.as_ref()), pad_pkcs1_sha256())
  }

  /// Signs the given message using RSASSA-PKCS#1.5 padding and SHA-384.
  pub fn sign_pkcs1_sha384(&self, message: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    self.sign(sha2::Sha384::digest(message.as_ref()), pad_pkcs1_sha384())
  }

  /// Signs the given message using RSASSA-PKCS#1.5 padding and SHA-512.
  pub fn sign_pkcs1_sha512(&self, message: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    self.sign(sha2::Sha512::digest(message.as_ref()), pad_pkcs1_sha512())
  }

  /// Signs the given message using RSASSA-PSS padding and SHA-256.
  pub fn sign_pss_sha256<R, M>(&self, rng: R, message: M) -> Result<Vec<u8>>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
  {
    self.sign(sha2::Sha256::digest(message.as_ref()), pad_pss_sha256(rng))
  }

  /// Signs the given message using RSASSA-PSS padding and SHA-384.
  pub fn sign_pss_sha384<R, M>(&self, rng: R, message: M) -> Result<Vec<u8>>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
  {
    self.sign(sha2::Sha384::digest(message.as_ref()), pad_pss_sha384(rng))
  }

  /// Signs the given message using RSASSA-PSS padding and SHA-512.
  pub fn sign_pss_sha512<R, M>(&self, rng: R, message: M) -> Result<Vec<u8>>
  where
    R: RngCore + CryptoRng + 'static,
    M: AsRef<[u8]>,
  {
    self.sign(sha2::Sha512::digest(message.as_ref()), pad_pss_sha512(rng))
  }

  fn sign(&self, message: impl AsRef<[u8]>, padding: PaddingScheme) -> Result<Vec<u8>> {
    self.0.sign(padding, message.as_ref()).map_err(|_| RSA_ERR)
  }
}

impl_secret_debug!(PrivateKey);

impl Zeroize for PrivateKey {
  fn zeroize(&mut self) {
    self.0.zeroize();
  }
}

impl Drop for PrivateKey {
  fn drop(&mut self) {
    self.zeroize();
  }
}

// =============================================================================
// RSA Bits
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RsaBits {
  B2048,
  B3072,
  B4096,
}

impl RsaBits {
  pub const fn bits(self) -> usize {
    match self {
      Self::B2048 => 2048,
      Self::B3072 => 3072,
      Self::B4096 => 4096,
    }
  }
}

// =============================================================================
// RSA Padding
// =============================================================================

fn pad_pkcs1_sha256() -> PaddingScheme {
  PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256))
}

fn pad_pkcs1_sha384() -> PaddingScheme {
  PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_384))
}

fn pad_pkcs1_sha512() -> PaddingScheme {
  PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_512))
}

fn pad_pss_sha256<R>(rng: R) -> PaddingScheme
where
  R: RngCore + CryptoRng + 'static,
{
  PaddingScheme::new_pss::<sha2::Sha256, R>(rng)
}

fn pad_pss_sha384<R>(rng: R) -> PaddingScheme
where
  R: RngCore + CryptoRng + 'static,
{
  PaddingScheme::new_pss::<sha2::Sha384, R>(rng)
}

fn pad_pss_sha512<R>(rng: R) -> PaddingScheme
where
  R: RngCore + CryptoRng + 'static,
{
  PaddingScheme::new_pss::<sha2::Sha512, R>(rng)
}
