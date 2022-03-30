// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::Proof;
use crate::error::Error;
use crate::error::Result;

/// A trait for types that can provide a reference to a [`Proof`].
pub trait TrySignature {
  /// Returns a reference to the [`Proof`] object, if any.
  fn signature(&self) -> Option<&Proof>;

  /// Returns a reference to the [`Proof`] object.
  ///
  /// Errors
  ///
  /// Fails if the signature is not found.
  fn try_signature(&self) -> Result<&Proof> {
    self.signature().ok_or(Error::MissingSignature)
  }
}

impl<'a, T> TrySignature for &'a T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Proof> {
    (**self).signature()
  }
}

impl<'a, T> TrySignature for &'a mut T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Proof> {
    (**self).signature()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can provide a mutable reference to a [`Proof`].
pub trait TrySignatureMut: TrySignature {
  /// Returns a mutable reference to the [`Proof`] object.
  fn signature_mut(&mut self) -> Option<&mut Proof>;

  /// Returns a mutable reference to the [`Proof`] object.
  ///
  /// Errors
  ///
  /// Fails if the signature is not found.
  fn try_signature_mut(&mut self) -> Result<&mut Proof> {
    self.signature_mut().ok_or(Error::MissingSignature)
  }
}

impl<'a, T> TrySignatureMut for &'a mut T
where
  T: TrySignatureMut,
{
  fn signature_mut(&mut self) -> Option<&mut Proof> {
    (**self).signature_mut()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can store a digital [signature][`Proof`].
pub trait SetSignature: TrySignatureMut {
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
