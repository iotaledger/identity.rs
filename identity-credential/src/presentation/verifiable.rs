// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::convert::ToJson;
use identity_core::crypto::Signature;
use identity_did::verifiable::SetSignature;
use identity_did::verifiable::TrySignature;
use identity_did::verifiable::TrySignatureMut;
use serde::Serialize;

use crate::presentation::Presentation;

/// A `VerifiablePresentation` represents a `Presentation` with an associated
/// digital proof.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiablePresentation<T = Object, U = Object> {
  #[serde(flatten)]
  presentation: Presentation<T, U>,
  #[serde(skip_serializing_if = "OneOrMany::is_empty")]
  proof: OneOrMany<Signature>,
}

impl<T, U> VerifiablePresentation<T, U> {
  /// Creates a new `VerifiablePresentation`.
  pub fn new<P>(presentation: Presentation<T, U>, proof: P) -> Self
  where
    P: Into<OneOrMany<Signature>>,
  {
    Self {
      presentation,
      proof: proof.into(),
    }
  }

  /// Returns a reference to the `VerifiablePresentation` proof.
  pub fn proof(&self) -> &OneOrMany<Signature> {
    &self.proof
  }

  /// Returns a mutable reference to the `VerifiablePresentation` proof.
  pub fn proof_mut(&mut self) -> &mut OneOrMany<Signature> {
    &mut self.proof
  }
}

impl<T, U> Deref for VerifiablePresentation<T, U> {
  type Target = Presentation<T, U>;

  fn deref(&self) -> &Self::Target {
    &self.presentation
  }
}

impl<T, U> DerefMut for VerifiablePresentation<T, U> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.presentation
  }
}

impl<T, U> Display for VerifiablePresentation<T, U>
where
  T: Serialize,
  U: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

impl<T, U> TrySignature for VerifiablePresentation<T, U> {
  fn signature(&self) -> Option<&Signature> {
    self.proof.get(0)
  }
}

impl<T, U> TrySignatureMut for VerifiablePresentation<T, U> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.get_mut(0)
  }
}

impl<T, U> SetSignature for VerifiablePresentation<T, U> {
  fn set_signature(&mut self, value: Signature) {
    self.proof = OneOrMany::One(value);
  }
}
