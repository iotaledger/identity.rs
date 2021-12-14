// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;
use identity_core::common::Object;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_core::diff::Diff;

use crate::verification::MethodUriType;
use crate::verification::TryMethod;

/// A generic container for a set of properties (`T`) and a
/// [`digital signature`][Signature].
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ProofProperties<T = Object> {
  #[serde(flatten)]
  pub properties: T,
  // TODO: Support multiple signatures (?)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Signature>,
}

impl<T> ProofProperties<T> {
  /// Creates a new `Properties` object.
  pub const fn new(properties: T) -> Self {
    Self {
      properties,
      proof: None,
    }
  }

  /// Creates a new `Properties` object with the given `proof`.
  pub const fn new_with_proof(properties: T, proof: Signature) -> Self {
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

/// NOTE: excludes the `proof` Signature from the diff to save space on the Tangle and because
/// a merged signature will be invalid in general.
impl<T> Diff for ProofProperties<T>
where
  T: Diff,
{
  type Type = <T as Diff>::Type;

  fn diff(&self, other: &Self) -> identity_core::diff::Result<Self::Type> {
    self.properties.diff(&other.properties)
  }

  fn merge(&self, diff: Self::Type) -> identity_core::diff::Result<Self> {
    let mut this: ProofProperties<T> = self.clone();
    this.properties = this.properties.merge(diff)?;
    Ok(this)
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    let properties: T = T::from_diff(diff)?;
    Ok(ProofProperties {
      properties,
      proof: None, // proof intentionally excluded
    })
  }

  fn into_diff(self) -> identity_core::diff::Result<Self::Type> {
    self.properties.into_diff()
  }
}

impl<T> Deref for ProofProperties<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.properties
  }
}

impl<T> DerefMut for ProofProperties<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.properties
  }
}

impl<T> TrySignature for ProofProperties<T> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T> TrySignatureMut for ProofProperties<T> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T> SetSignature for ProofProperties<T> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

impl<T> TryMethod for ProofProperties<T> {
  const TYPE: MethodUriType = MethodUriType::Relative;
}
