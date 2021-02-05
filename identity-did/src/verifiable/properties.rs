// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;
use identity_core::common::Object;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Properties<T = Object> {
  #[serde(flatten)]
  pub(crate) properties: T,
  // TODO: Support multiple signatures (?)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Signature>,
}

impl<T> Properties<T> {
  pub const fn new(properties: T) -> Self {
    Self {
      properties,
      proof: None,
    }
  }

  pub const fn with_proof(properties: T, proof: Signature) -> Self {
    Self {
      properties,
      proof: Some(proof),
    }
  }

  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }

  pub fn set_proof(&mut self, signature: Signature) {
    self.proof = Some(signature);
  }
}

impl<T> Deref for Properties<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.properties
  }
}

impl<T> DerefMut for Properties<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.properties
  }
}

impl<T> TrySignature for Properties<T> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T> TrySignatureMut for Properties<T> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T> SetSignature for Properties<T> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}
