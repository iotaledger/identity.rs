// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::Signature;
use crate::crypto::TrySignature;
use crate::error::Error;
use crate::error::Result;

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
