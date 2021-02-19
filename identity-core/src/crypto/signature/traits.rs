// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use erased_serde::serialize_trait_object;
use erased_serde::Serialize;

use crate::crypto::Signature;
use crate::crypto::SignatureValue;
use crate::error::Error;
use crate::error::Result;

/// A trait for signature suites identified by a particular name.
pub trait SignatureName {
  /// Returns a unique identifier for the signatures created by this suite.
  fn name(&self) -> String;
}

impl<T: ?Sized> SignatureName for Box<T>
where
  T: SignatureName,
{
  fn name(&self) -> String {
    (**self).name()
  }
}

// =============================================================================
// =============================================================================

/// A trait for general-purpose signature creation
pub trait SignatureSign: SignatureName {
  /// Signs the given `data` with `secret` and returns a digital signature.
  fn sign(&self, data: &dyn Serialize, secret: &[u8]) -> Result<SignatureValue>;

  #[doc(hidden)]
  fn __sign(&self, data: &mut dyn __TargetSign, method: String, secret: &[u8]) -> Result<()> {
    let signature: Signature = Signature::new(self.name(), method);

    data.set_signature(signature);

    let value: SignatureValue = self.sign(&data, secret)?;
    let write: &mut Signature = data.try_signature_mut()?;

    write.set_value(value);

    Ok(())
  }
}

macro_rules! impl_sign_deref {
  ($($tt:tt)+) => {
    impl $($tt)+ {
      fn sign(&self, data: &dyn Serialize, secret: &[u8]) -> Result<SignatureValue> {
        (**self).sign(data, secret)
      }
    }
  };
}

impl_sign_deref!(<'a> SignatureSign for Box<dyn SignatureSign + 'a>);
impl_sign_deref!(<'a> SignatureSign for Box<dyn SignatureSign + Send + 'a>);
impl_sign_deref!(<'a> SignatureSign for Box<dyn SignatureSign + Sync + 'a>);
impl_sign_deref!(<'a> SignatureSign for Box<dyn SignatureSign + Send + Sync + 'a>);

#[doc(hidden)]
pub trait __TargetSign: Serialize + SetSignature {}

#[doc(hidden)]
impl<T> __TargetSign for T where T: Serialize + SetSignature {}

serialize_trait_object!(__TargetSign);

// =============================================================================
// =============================================================================

/// A trait for general-purpose signature verification
pub trait SignatureVerify: SignatureName {
  /// Verifies the authenticity of `data` using `signature` and `public`.
  fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()>;

  #[doc(hidden)]
  fn __verify(&self, data: &dyn __TargetVerify, public: &[u8]) -> Result<()> {
    let signature: &Signature = data.try_signature()?;

    if signature.type_() != self.name() {
      return Err(Error::InvalidProofValue);
    }

    signature.hide_value();

    self.verify(&data, signature.value(), public)?;

    signature.show_value();

    Ok(())
  }
}

macro_rules! impl_verify_deref {
  ($($tt:tt)+) => {
    impl $($tt)+ {
      fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()> {
        (**self).verify(data, signature, public)
      }
    }
  };
}

impl_verify_deref!(<'a> SignatureVerify for Box<dyn SignatureVerify + 'a>);
impl_verify_deref!(<'a> SignatureVerify for Box<dyn SignatureVerify + Send + 'a>);
impl_verify_deref!(<'a> SignatureVerify for Box<dyn SignatureVerify + Sync + 'a>);
impl_verify_deref!(<'a> SignatureVerify for Box<dyn SignatureVerify + Send + Sync + 'a>);

#[doc(hidden)]
pub trait __TargetVerify: Serialize + TrySignature {}

#[doc(hidden)]
impl<T> __TargetVerify for T where T: Serialize + TrySignature {}

serialize_trait_object!(__TargetVerify);

// =============================================================================
// =============================================================================

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
