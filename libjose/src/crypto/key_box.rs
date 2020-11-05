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

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use zeroize::Zeroize;

use crate::crypto::rand::CryptoRng;
use crate::crypto::rand::RngCore;
use crate::crypto::Error;
use crate::crypto::Result;

mod private {
  pub trait Sealed {}
}

/// A marker trait for representing a cryptographic key.
pub trait KeyType: private::Sealed {}

// =============================================================================
// Public
// =============================================================================

/// A marker type for `KeyBox`s with public key components.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Public {}

impl private::Sealed for Public {}

impl KeyType for Public {}

// =============================================================================
// Secret
// =============================================================================

/// A marker type for `KeyBox`s with private key components.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Secret {}

impl private::Sealed for Secret {}

impl KeyType for Secret {}

// =============================================================================
// Type Aliases
// =============================================================================

pub type PublicKey = KeyBox<Public>;

pub type SecretKey = KeyBox<Secret>;

// =============================================================================
// Key Box
// =============================================================================

/// A generic box of bytes.
///
/// The internal slice is zeroed on Drop.
#[derive(Clone)]
pub struct KeyBox<T>(Box<[u8]>, PhantomData<T>)
where
  T: KeyType;

impl<T> KeyBox<T>
where
  T: KeyType,
{
  pub fn random<R>(size: usize, rng: &mut R) -> Result<Self>
  where
    R: RngCore + CryptoRng,
  {
    let mut data: Vec<u8> = alloc::vec![0; size];

    rng
      .try_fill_bytes(&mut data)
      .map_err(|_| Error::RngError { what: "fill" })?;

    Ok(data.into())
  }
}

impl<T> Zeroize for KeyBox<T>
where
  T: KeyType,
{
  fn zeroize(&mut self) {
    self.0.zeroize();
  }
}

impl<T> Drop for KeyBox<T>
where
  T: KeyType,
{
  fn drop(&mut self) {
    self.zeroize();
  }
}

impl<T> AsRef<[u8]> for KeyBox<T>
where
  T: KeyType,
{
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl fmt::Debug for KeyBox<Public> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("KeyBox(*Public*)")
  }
}

impl fmt::Debug for KeyBox<Secret> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("KeyBox(*Secret*)")
  }
}

impl<T> From<Box<[u8]>> for KeyBox<T>
where
  T: KeyType,
{
  fn from(other: Box<[u8]>) -> Self {
    Self(other, PhantomData)
  }
}

impl<T> From<Vec<u8>> for KeyBox<T>
where
  T: KeyType,
{
  fn from(other: Vec<u8>) -> Self {
    Self(other.into(), PhantomData)
  }
}

impl<T> From<&[u8]> for KeyBox<T>
where
  T: KeyType,
{
  fn from(other: &[u8]) -> Self {
    Self(other.into(), PhantomData)
  }
}
