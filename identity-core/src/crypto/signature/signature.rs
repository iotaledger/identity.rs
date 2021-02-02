// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::crypto::SigVerify;
use crate::crypto::SignatureData;
use crate::crypto::SignatureOptions;
use crate::crypto::SignatureValue;
use crate::error::Result;

/// A DID Document digital signature.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Signature {
  #[serde(rename = "type")]
  type_: String,
  #[serde(flatten, skip_serializing_if = "SignatureValue::is_none")]
  data: SignatureValue,
  #[serde(flatten)]
  options: SignatureOptions,
}

impl Signature {
  /// Creates a new [`Signature`].
  pub fn new(type_: impl Into<String>, options: SignatureOptions) -> Self {
    Self {
      type_: type_.into(),
      options,
      data: SignatureValue::new(),
    }
  }

  /// Returns the `type` property of the signature.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns a reference to the signature `data`.
  pub const fn data(&self) -> &SignatureValue {
    &self.data
  }

  /// Returns a mutable reference to the signature `data`.
  pub fn data_mut(&mut self) -> &mut SignatureValue {
    &mut self.data
  }

  /// Sets the signature `data` to the given `value`.
  pub fn set_data(&mut self, value: SignatureData) {
    self.data.set(value);
  }

  /// Clears the current signature value - all other properties are unchanged.
  pub fn clear_data(&mut self) {
    self.data.clear();
  }

  /// Flag the signature value so it is ignored during serialization
  pub fn hide_value(&self) {
    self.data.hide();
  }

  /// Restore the signature value state so serialization behaves normally
  pub fn show_value(&self) {
    self.data.show();
  }

  /// Verifies `self` with the given signature `suite` and `public` key.
  pub fn verify<S, M>(&self, suite: &S, message: &M, public: &[u8]) -> Result<()>
  where
    S: SigVerify,
    M: Serialize,
  {
    self.verifiable(|data| suite.verify(message, data, public))
  }

  #[doc(hidden)]
  pub fn verifiable<T, F>(&self, f: F) -> T
  where
    F: FnOnce(&SignatureValue) -> T,
  {
    self.data.hide();

    let output: T = f(self.data());

    self.data.show();

    output
  }
}

impl Debug for Signature {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Signature")
      .field("type_", &self.type_)
      .field("data", &self.data)
      .field("verification_method", &self.options.verification_method)
      .field("proof_purpose", &self.options.proof_purpose)
      .field("created", &self.options.created)
      .field("nonce", &self.options.nonce)
      .field("domain", &self.options.domain)
      .finish()
  }
}

impl Deref for Signature {
  type Target = SignatureOptions;

  fn deref(&self) -> &Self::Target {
    &self.options
  }
}

impl DerefMut for Signature {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.options
  }
}
