// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::error::Result;
use crate::signature::SignatureData;
use crate::verification::MethodType;

pub trait SuiteName {
  fn name(&self) -> String;
}

impl<'a, T> SuiteName for &'a T
where
  T: SuiteName,
{
  fn name(&self) -> String {
    (**self).name()
  }
}

// =============================================================================
// =============================================================================

pub trait Sign {
  fn sign<T>(&self, data: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize;
}

impl<'a, T> Sign for &'a T
where
  T: Sign,
{
  fn sign<U>(&self, data: &U, secret: &[u8]) -> Result<SignatureData>
  where
    U: Serialize,
  {
    (**self).sign(data, secret)
  }
}

// =============================================================================
// =============================================================================

pub trait Verify {
  const METHODS: &'static [MethodType];

  fn verify<T>(&self, data: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    T: Serialize;
}

impl<'a, T> Verify for &'a T
where
  T: Verify,
{
  const METHODS: &'static [MethodType] = T::METHODS;

  fn verify<U>(&self, data: &U, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    U: Serialize,
  {
    (**self).verify(data, signature, public)
  }
}
