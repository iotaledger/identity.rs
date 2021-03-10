// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::Signature;
use crate::crypto::TrySignatureMut;

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
