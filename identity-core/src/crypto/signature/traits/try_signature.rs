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
