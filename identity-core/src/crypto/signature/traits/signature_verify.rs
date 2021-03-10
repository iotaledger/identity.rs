// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::crypto::Signature;
use crate::crypto::SignatureName;
use crate::crypto::SignatureValue;
use crate::crypto::TrySignature;
use crate::error::Error;
use crate::error::Result;

/// A trait for general-purpose signature verification
pub trait SignatureVerify<'key>: SignatureName {
  /// Delegated signature implementation.
  type Actual: SignatureVerify<'key>;

  /// The public key type of this signature
  type Public: ?Sized + 'key;

  /// Creates a new [`SignatureVerify`] instance.
  fn create(key: &'key Self::Public) -> Self::Actual;

  /// Verifies the authenticity of `data` and `signature`.
  fn verify<T>(&self, data: &T, signature: &SignatureValue) -> Result<()>
  where
    T: Serialize;

  /// Extracts and verifies a [signature][`Signature`] from the given `data`.
  fn verify_signature<T>(&self, data: &T) -> Result<()>
  where
    T: Serialize + TrySignature,
  {
    let signature: &Signature = data.try_signature()?;

    if signature.type_() != Self::NAME {
      return Err(Error::InvalidProofValue);
    }

    signature.hide_value();
    self.verify(&data, signature.value())?;
    signature.show_value();

    Ok(())
  }
}
