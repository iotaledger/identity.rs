// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;

use crate::document::Document;
use crate::verifiable::Properties;

impl<T, U, V> Document<T, U, V> {
  pub fn into_verifiable(self) -> Document<Properties<T>, U, V> {
    self.map(Properties::new)
  }

  pub fn into_verifiable2(self, proof: Signature) -> Document<Properties<T>, U, V> {
    self.map(|old| Properties::with_proof(old, proof))
  }
}

impl<T, U, V> Document<Properties<T>, U, V> {
  pub fn proof(&self) -> Option<&Signature> {
    self.properties().proof()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.properties_mut().proof_mut()
  }

  pub fn set_proof(&mut self, signature: Signature) {
    self.properties_mut().set_proof(signature);
  }
}

impl<T, U, V> TrySignature for Document<Properties<T>, U, V> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T, U, V> TrySignatureMut for Document<Properties<T>, U, V> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T, U, V> SetSignature for Document<Properties<T>, U, V> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}
