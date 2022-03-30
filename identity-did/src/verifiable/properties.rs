// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;

use identity_core::common::Object;
use identity_core::crypto::Proof;
use identity_core::crypto::SetSignature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_core::diff::Diff;

use crate::verification::MethodUriType;
use crate::verification::TryMethod;

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

/// NOTE: excludes the `proof` Signature from the diff to save space on the Tangle and because
/// a merged signature will be invalid in general.
impl<T> Diff for VerifiableProperties<T>
where
  T: Diff,
{
  type Type = <T as Diff>::Type;

  fn diff(&self, other: &Self) -> identity_core::diff::Result<Self::Type> {
    self.properties.diff(&other.properties)
  }

  fn merge(&self, diff: Self::Type) -> identity_core::diff::Result<Self> {
    let mut this: VerifiableProperties<T> = self.clone();
    this.properties = this.properties.merge(diff)?;
    Ok(this)
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    let properties: T = T::from_diff(diff)?;
    Ok(VerifiableProperties {
      properties,
      proof: None, // proof intentionally excluded
    })
  }

  fn into_diff(self) -> identity_core::diff::Result<Self::Type> {
    self.properties.into_diff()
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
  fn signature(&self) -> Option<&Proof> {
    self.proof.as_ref()
  }
}

impl<T> TrySignatureMut for VerifiableProperties<T> {
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
