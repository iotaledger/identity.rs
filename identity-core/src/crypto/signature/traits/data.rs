// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::Signature;
use crate::error::Error;
use crate::error::Result;

/// A trait for types that can provide a reference to a [`Signature`].
pub trait TrySignature {
  /// Returns a reference to the [`Signature`] object, if any.
  fn signature(&self) -> Option<&Signature>;

  /// Returns a reference to the [`Signature`] object.
  ///
  /// Errors
  ///
  /// Fails if the signature is not found.
  fn try_signature(&self) -> Result<&Signature> {
    self.signature().ok_or(Error::MissingSignature)
  }
}

impl<'a, T> TrySignature for &'a T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Signature> {
    (**self).signature()
  }
}

impl<'a, T> TrySignature for &'a mut T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Signature> {
    (**self).signature()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can provide a mutable reference to a [`Signature`].
pub trait TrySignatureMut: TrySignature {
  /// Returns a mutable reference to the [`Signature`] object.
  fn signature_mut(&mut self) -> Option<&mut Signature>;

  /// Returns a mutable reference to the [`Signature`] object.
  ///
  /// Errors
  ///
  /// Fails if the signature is not found.
  fn try_signature_mut(&mut self) -> Result<&mut Signature> {
    self.signature_mut().ok_or(Error::MissingSignature)
  }
}

impl<'a, T> TrySignatureMut for &'a mut T
where
  T: TrySignatureMut,
{
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    (**self).signature_mut()
  }
}

// =============================================================================
// =============================================================================

/// A trait for types that can store a digital [signature][`Signature`].
pub trait SetSignature: TrySignatureMut {
  /// Sets the [`Signature`] object of `self`.
  fn set_signature(&mut self, signature: Signature);
}

impl<'a, T> SetSignature for &'a mut T
where
  T: SetSignature,
{
  fn set_signature(&mut self, signature: Signature) {
    (**self).set_signature(signature);
  }
}
