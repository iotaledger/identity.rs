// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::crypto::SetSignature;
use crate::crypto::Signature;
use crate::crypto::SignatureName;
use crate::crypto::SignatureValue;
use crate::error::Result;

/// A trait for general-purpose signature creation
pub trait SignatureSign<'key>: SignatureName {
  /// Delegated signature implementation.
  type Actual: SignatureSign<'key>;

  /// The secret key type of this signature
  type Secret: ?Sized;

  /// Creates a new [`SignatureSign`] instance.
  fn create(key: &'key Self::Secret) -> Self::Actual;

  /// Signs the given `data` and returns a digital signature.
  fn sign<T>(&self, data: &T) -> Result<SignatureValue>
  where
    T: Serialize;

  /// Creates and applies a [signature][`Signature`] to the given `data`.
  fn create_signature<T>(&self, data: &mut T, method: &str) -> Result<()>
  where
    T: Serialize + SetSignature,
  {
    data.set_signature(Signature::new(Self::NAME, method));

    let value: SignatureValue = self.sign(&data)?;
    let write: &mut Signature = data.try_signature_mut()?;

    write.set_value(value);

    Ok(())
  }
}
