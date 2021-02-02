// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::crypto::SignatureData;
use crate::error::Result;

/// A trait for signature suites identified by a particular name.
pub trait SigName {
  /// Returns the name identifying this signature suite.
  fn name(&self) -> String;
}

impl<'a, T> SigName for &'a T
where
  T: SigName,
{
  fn name(&self) -> String {
    (**self).name()
  }
}

// =============================================================================
// =============================================================================

/// A trait for general-purpose signature creation
pub trait SigSign {
  /// Signs the given `data` with `secret` and returns a digital signature.
  fn sign<T>(&self, data: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize;
}

impl<'a, T> SigSign for &'a T
where
  T: SigSign,
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

/// A trait for general-purpose signature verification
pub trait SigVerify {
  /// Verifies the authenticity of `data` using `signature` and `public`.
  fn verify<T>(&self, data: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    T: Serialize;
}

impl<'a, T> SigVerify for &'a T
where
  T: SigVerify,
{
  fn verify<U>(&self, data: &U, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    U: Serialize,
  {
    (**self).verify(data, signature, public)
  }
}
