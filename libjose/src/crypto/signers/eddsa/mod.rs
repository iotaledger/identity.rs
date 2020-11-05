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

pub mod ed25519;

// TODO: Curve Ed448

use zeroize::Zeroize;

use crate::crypto::error::Result;
use crate::crypto::rand::CryptoRng;
use crate::crypto::rand::RngCore;

// =============================================================================
// EdDSA Curve
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Curve {
  Ed25519,
  Ed448,
}

// =============================================================================
// EdDSA Public Key
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum PublicKey {
  Ed25519(ed25519::PublicKey),
  Ed448,
}

impl PublicKey {
  pub fn from_slice(curve: Curve, slice: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      Curve::Ed25519 => ed25519::PublicKey::from_slice(slice).map(Self::Ed25519),
      Curve::Ed448 => todo!("PublicKey::from_slice(Ed448)"),
    }
  }

  pub fn verify(&self, message: impl AsRef<[u8]>, signature: impl AsRef<[u8]>) -> Result<()> {
    match self {
      Self::Ed25519(inner) => inner.verify(message, signature),
      Self::Ed448 => todo!("PrivateKey::public_key(Ed448)"),
    }
  }
}

// =============================================================================
// EdDSA Private Key
// =============================================================================

pub enum PrivateKey {
  Ed25519(ed25519::PrivateKey),
  Ed448,
}

impl PrivateKey {
  pub fn random<R>(curve: Curve, rng: &mut R) -> Result<Self>
  where
    R: CryptoRng + RngCore,
  {
    match curve {
      Curve::Ed25519 => ed25519::PrivateKey::random(rng).map(Self::Ed25519),
      Curve::Ed448 => todo!("PrivateKey::random(Ed448)"),
    }
  }

  pub fn from_slice(curve: Curve, bytes: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      Curve::Ed25519 => ed25519::PrivateKey::from_slice(bytes).map(Self::Ed25519),
      Curve::Ed448 => todo!("PrivateKey::from_slice(Ed448)"),
    }
  }

  pub fn public_key(&self) -> PublicKey {
    match self {
      Self::Ed25519(inner) => PublicKey::Ed25519(inner.public_key()),
      Self::Ed448 => todo!("PrivateKey::public_key(Ed448)"),
    }
  }

  pub fn sign(&self, message: impl AsRef<[u8]>) -> Result<Signature> {
    match self {
      Self::Ed25519(inner) => inner.sign(message).map(Signature::Ed25519),
      Self::Ed448 => todo!("PrivateKey::sign(Ed448)"),
    }
  }
}

impl_secret_debug!(PrivateKey);

impl Zeroize for PrivateKey {
  fn zeroize(&mut self) {
    match self {
      Self::Ed25519(inner) => inner.zeroize(),
      Self::Ed448 => todo!("PrivateKey::zeroize(Ed448)"),
    }
  }
}

impl Drop for PrivateKey {
  fn drop(&mut self) {
    self.zeroize();
  }
}

// =============================================================================
// EdDSA Signature
// =============================================================================

#[derive(Clone, Copy)]
pub enum Signature {
  Ed25519(::ed25519_dalek::Signature),
  Ed448,
}

impl AsRef<[u8]> for Signature {
  fn as_ref(&self) -> &[u8] {
    match self {
      Self::Ed25519(inner) => inner.as_ref(),
      Self::Ed448 => todo!("Signature::as_ref::<[u8]>(Ed448)"),
    }
  }
}
