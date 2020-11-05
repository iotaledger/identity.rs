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

use core::convert::TryFrom as _;
use ed25519_dalek as ed25519;
use ed25519_dalek::Verifier as _;
use zeroize::Zeroize;

use crate::crypto::error::Error;
use crate::crypto::error::Result;
use crate::crypto::rand::CryptoRng;
use crate::crypto::rand::RngCore;

pub const ED25519_ERR: Error = Error::SignatureError {
  alg: "EdDSA (ed25519)",
};

// =========================================================================
// Ed25519 Public Key
// =========================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKey(ed25519::PublicKey);

impl PublicKey {
  pub fn from_slice(bytes: impl AsRef<[u8]>) -> Result<Self> {
    ed25519::PublicKey::from_bytes(bytes.as_ref())
      .map_err(|_| ED25519_ERR)
      .map(Self)
  }

  pub fn verify(&self, message: impl AsRef<[u8]>, signature: impl AsRef<[u8]>) -> Result<()> {
    let sig: ed25519::Signature =
      ed25519::Signature::try_from(signature.as_ref()).map_err(|_| ED25519_ERR)?;

    self
      .0
      .verify(message.as_ref(), &sig)
      .map_err(|_| ED25519_ERR)?;

    Ok(())
  }
}

// =========================================================================
// Ed25519 Secret Key
// =========================================================================

pub struct PrivateKey(ed25519::SecretKey);

impl PrivateKey {
  pub fn random<R>(rng: &mut R) -> Result<Self>
  where
    R: CryptoRng + RngCore,
  {
    let mut key: [u8; ed25519::SECRET_KEY_LENGTH] = [0; ed25519::SECRET_KEY_LENGTH];

    rng
      .try_fill_bytes(&mut key[..])
      .map_err(|_| Error::RngError { what: "fill" })?;

    // This should never fail as we construct a correctly sized key using
    // `ed25519::SECRET_KEY_LENGTH`.
    Ok(Self::from_slice(&key[..]).expect("infallible"))
  }

  pub fn from_slice(bytes: impl AsRef<[u8]>) -> Result<Self> {
    ed25519::SecretKey::from_bytes(bytes.as_ref())
      .map_err(|_| ED25519_ERR)
      .map(Self)
  }

  pub fn public_key(&self) -> PublicKey {
    PublicKey((&self.0).into())
  }

  pub fn sign(&self, message: impl AsRef<[u8]>) -> Result<ed25519::Signature> {
    let key: ed25519::ExpandedSecretKey = (&self.0).into();
    let sig: ed25519::Signature = key.sign(message.as_ref(), &(&key).into());

    Ok(sig)
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
