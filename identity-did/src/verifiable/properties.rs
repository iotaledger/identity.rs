// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;
use identity_core::common::Object;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;

use crate::verification::MethodUriType;
use crate::verification::TryMethod;

/// A generic container for a set of properties (`T`) and a
/// [`digital signature`][Signature].
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VerifiableProperties<T = Object> {
  #[serde(flatten)]
  pub(crate) properties: T,
  // TODO: Support multiple signatures (?)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Signature>,
}

impl<T> VerifiableProperties<T> {
  /// Creates a new `Properties` object.
  pub const fn new(properties: T) -> Self {
    Self {
      properties,
      proof: None,
    }
  }

  /// Creates a new `Properties` object with the given `proof`.
  pub const fn with_proof(properties: T, proof: Signature) -> Self {
    Self {
      properties,
      proof: Some(proof),
    }
  }

  /// Returns a reference to the [`proof`][`Signature`].
  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  /// Returns a mutable reference to the [`proof`][`Signature`].
  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }

  /// Sets the value of the [`proof`][`Signature`].
  pub fn set_proof(&mut self, signature: Signature) {
    self.proof = Some(signature);
  }
}

impl<T> Deref for VerifiableProperties<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.properties
  }
}

impl<T> DerefMut for VerifiableProperties<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.properties
  }
}

impl<T> TrySignature for VerifiableProperties<T> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T> TrySignatureMut for VerifiableProperties<T> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T> SetSignature for VerifiableProperties<T> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

impl<T> TryMethod for VerifiableProperties<T> {
  const TYPE: MethodUriType = MethodUriType::Relative;
}
