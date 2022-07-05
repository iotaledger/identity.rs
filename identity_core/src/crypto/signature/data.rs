// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::Proof;

/// A trait for types that can provide a reference to a [`Proof`].
pub trait GetSignature {
  /// Returns a reference to the [`Proof`] object, if any.
  fn signature(&self) -> Option<&Proof>;
}

impl<'a, T> GetSignature for Box<T>
  where
    T: GetSignature,
{
  fn signature(&self) -> Option<&Proof> {
    (*self).as_ref().signature()
  }
}

impl<'a, T> GetSignature for &'a T
where
  T: GetSignature,
{
  fn signature(&self) -> Option<&Proof> {
    (**self).signature()
  }
}

impl<'a, T> GetSignature for &'a mut T
where
  T: GetSignature,
{
  fn signature(&self) -> Option<&Proof> {
    (**self).signature()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can provide a mutable reference to a [`Proof`].
pub trait GetSignatureMut: GetSignature {
  /// Returns a mutable reference to the [`Proof`] object, if any.
  fn signature_mut(&mut self) -> Option<&mut Proof>;
}

impl<'a, T> GetSignatureMut for &'a mut T
where
  T: GetSignatureMut,
{
  fn signature_mut(&mut self) -> Option<&mut Proof> {
    (**self).signature_mut()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can store a digital [signature][`Proof`].
pub trait SetSignature: GetSignatureMut {
  /// Sets the [`Proof`] object of `self`.
  fn set_signature(&mut self, signature: Proof);
}

impl<'a, T> SetSignature for &'a mut T
where
  T: SetSignature,
{
  fn set_signature(&mut self, signature: Proof) {
    (**self).set_signature(signature);
  }
}
