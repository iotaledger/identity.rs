// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use identity_core::crypto::Signature;
use serde::Serialize;

use crate::document::Document as Document_;
use crate::verifiable::Properties;
use crate::verifiable::ResolveMethod;
use crate::verifiable::SetSignature;
use crate::verifiable::TrySignature;
use crate::verifiable::TrySignatureMut;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Document<T = (), U = (), V = ()> {
  document: Document_<Properties<T>, U, V>,
}

impl<T, U, V> Document<T, U, V> {
  pub fn new(document: Document_<T, U, V>) -> Self {
    Self {
      document: document.map(Properties::new),
    }
  }

  pub fn with_proof(document: Document_<T, U, V>, proof: Signature) -> Self {
    Self {
      document: document.map(|old| Properties::with_proof(old, proof)),
    }
  }

  pub fn proof(&self) -> Option<&Signature> {
    self.properties().proof()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.properties_mut().proof_mut()
  }

  pub fn set_proof(&mut self, signature: Signature) {
    self.properties_mut().proof = Some(signature);
  }
}

impl<T, U, V> Deref for Document<T, U, V> {
  type Target = Document_<Properties<T>, U, V>;

  fn deref(&self) -> &Self::Target {
    &self.document
  }
}

impl<T, U, V> DerefMut for Document<T, U, V> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.document
  }
}

impl<T, U, V> Debug for Document<T, U, V>
where
  T: Debug,
  U: Debug,
  V: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.document, f)
  }
}

impl<T, U, V> Display for Document<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.document, f)
  }
}

impl<T, U, V> TrySignature for Document<T, U, V> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T, U, V> TrySignatureMut for Document<T, U, V> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T, U, V> SetSignature for Document<T, U, V> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

impl<T, U, V> ResolveMethod<U> for Document<T, U, V> {
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, U>> {
    self.document.resolve(query)
  }
}
