// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;

use identity_core::common::Object;
use identity_core::crypto::GetSignature;
use identity_core::crypto::GetSignatureMut;
use identity_core::crypto::Proof;
use identity_core::crypto::SetSignature;

use identity_verification::MethodUriType;
use identity_verification::TryMethod;

/// A generic container for a [`digital signature`][Proof] and a set of properties.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VerifiableProperties<T = Object> {
  #[serde(flatten)]
  pub properties: T,
  // TODO: Support multiple signatures (?)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Proof>,
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
  pub const fn new_with_proof(properties: T, proof: Proof) -> Self {
    Self {
      properties,
      proof: Some(proof),
    }
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

impl<T> GetSignature for VerifiableProperties<T> {
  fn signature(&self) -> Option<&Proof> {
    self.proof.as_ref()
  }
}

impl<T> GetSignatureMut for VerifiableProperties<T> {
  fn signature_mut(&mut self) -> Option<&mut Proof> {
    self.proof.as_mut()
  }
}

impl<T> SetSignature for VerifiableProperties<T> {
  fn set_signature(&mut self, signature: Proof) {
    self.proof = Some(signature);
  }
}

impl<T> TryMethod for VerifiableProperties<T> {
  const TYPE: MethodUriType = MethodUriType::Relative;
}
