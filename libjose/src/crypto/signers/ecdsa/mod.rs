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

pub mod k256;
pub mod p256;

// TODO: Curve P-384
// TODO: Curve P-521

use zeroize::Zeroize;

use crate::crypto::error::Result;
use crate::crypto::rand::CryptoRng;
use crate::crypto::rand::RngCore;

// =============================================================================
// ECDSA Curve
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Curve {
  P256,
  P384,
  P521,
  K256,
}

// =============================================================================
// ECDSA Public Key
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum PublicKey {
  P256(p256::PublicKey),
  P384,
  P521,
  K256(k256::PublicKey),
}

impl PublicKey {
  pub fn from_coord(curve: Curve, x: impl AsRef<[u8]>, y: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      Curve::P256 => p256::PublicKey::from_coord(x, y).map(Self::P256),
      Curve::P384 => todo!("PublicKey::from_coord(P384)"),
      Curve::P521 => todo!("PublicKey::from_coord(P521)"),
      Curve::K256 => k256::PublicKey::from_coord(x, y).map(Self::K256),
    }
  }

  pub fn from_slice(curve: Curve, slice: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      Curve::P256 => p256::PublicKey::from_slice(slice).map(Self::P256),
      Curve::P384 => todo!("PublicKey::from_slice(P384)"),
      Curve::P521 => todo!("PublicKey::from_slice(P521)"),
      Curve::K256 => k256::PublicKey::from_slice(slice).map(Self::K256),
    }
  }

  pub fn verify(&self, message: impl AsRef<[u8]>, signature: impl AsRef<[u8]>) -> Result<()> {
    match self {
      Self::P256(inner) => inner.verify(message, signature),
      Self::P384 => todo!("PublicKey::verify(P384)"),
      Self::P521 => todo!("PublicKey::verify(P521)"),
      Self::K256(inner) => inner.verify(message, signature),
    }
  }
}

// =============================================================================
// ECDSA Private Key
// =============================================================================

pub enum PrivateKey {
  P256(p256::PrivateKey),
  P384,
  P521,
  K256(k256::PrivateKey),
}

impl PrivateKey {
  pub fn random(curve: Curve, rng: impl CryptoRng + RngCore) -> Self {
    match curve {
      Curve::P256 => Self::P256(p256::PrivateKey::random(rng)),
      Curve::P384 => todo!("PrivateKey::random(P384)"),
      Curve::P521 => todo!("PrivateKey::random(P521)"),
      Curve::K256 => Self::K256(k256::PrivateKey::random(rng)),
    }
  }

  /// Creates a `PrivateKey` from big-endian bytes.
  pub fn from_slice(curve: Curve, bytes: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      Curve::P256 => p256::PrivateKey::from_slice(bytes).map(Self::P256),
      Curve::P384 => todo!("PrivateKey::from_slice(P384)"),
      Curve::P521 => todo!("PrivateKey::from_slice(P521)"),
      Curve::K256 => k256::PrivateKey::from_slice(bytes).map(Self::K256),
    }
  }

  pub fn public_key(&self) -> PublicKey {
    match self {
      Self::P256(inner) => PublicKey::P256(inner.public_key()),
      Self::P384 => todo!("PrivateKey::public_key(P384)"),
      Self::P521 => todo!("PrivateKey::public_key(P384)"),
      Self::K256(inner) => PublicKey::K256(inner.public_key()),
    }
  }

  pub fn sign(&self, message: impl AsRef<[u8]>) -> Result<Signature> {
    match self {
      Self::P256(inner) => inner.sign(message).map(Signature::P256),
      Self::P384 => todo!("PrivateKey::sign(P384)"),
      Self::P521 => todo!("PrivateKey::sign(P521)"),
      Self::K256(inner) => inner.sign(message).map(Signature::K256),
    }
  }
}

impl_secret_debug!(PrivateKey);

impl Zeroize for PrivateKey {
  fn zeroize(&mut self) {
    match self {
      Self::P256(inner) => inner.zeroize(),
      Self::P384 => todo!("PrivateKey::zeroize(P384)"),
      Self::P521 => todo!("PrivateKey::zeroize(P521)"),
      Self::K256(inner) => inner.zeroize(),
    }
  }
}

impl Drop for PrivateKey {
  fn drop(&mut self) {
    self.zeroize();
  }
}

// =============================================================================
// ECDSA Signature
// =============================================================================

#[derive(Clone, Copy)]
pub enum Signature {
  P256(::p256::ecdsa::Signature),
  P384,
  P521,
  K256(::k256::ecdsa::Signature),
}

impl AsRef<[u8]> for Signature {
  fn as_ref(&self) -> &[u8] {
    match self {
      Self::P256(inner) => inner.as_ref(),
      Self::P384 => todo!("Signature::as_ref::<[u8]>(P384)"),
      Self::P521 => todo!("Signature::as_ref::<[u8]>(P521)"),
      Self::K256(inner) => inner.as_ref(),
    }
  }
}
