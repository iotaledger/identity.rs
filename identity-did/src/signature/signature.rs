// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::error::Result;
use crate::signature::SignatureData;
use crate::signature::SignatureOptions;
use crate::signature::SignatureValue;
use crate::signature::Verify;
use crate::verification::MethodIdent;
use crate::verification::MethodQuery;

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
  pub fn new(type_: impl Into<String>, options: SignatureOptions) -> Self {
    Self {
      type_: type_.into(),
      options,
      data: SignatureValue::new(),
    }
  }

  pub fn type_(&self) -> &str {
    &*self.type_
  }

  pub const fn data(&self) -> &SignatureValue {
    &self.data
  }

  pub fn data_mut(&mut self) -> &mut SignatureValue {
    &mut self.data
  }

  pub fn set_data(&mut self, value: SignatureData) {
    self.data.set(value);
  }

  pub fn clear_data(&mut self) {
    self.data.clear();
  }

  pub fn hide_value(&self) {
    self.data.hide();
  }

  pub fn show_value(&self) {
    self.data.show();
  }

  pub fn to_query(&self) -> Result<MethodQuery<'_>> {
    let ident: MethodIdent<'_> = (&*self.verification_method).into();

    if let Some(scope) = self.proof_purpose.as_deref() {
      Ok(MethodQuery::with_scope(ident, scope.parse()?))
    } else {
      Ok(MethodQuery::new(ident))
    }
  }

  pub fn verify<S, M>(&self, suite: &S, message: &M, public: &[u8]) -> Result<()>
  where
    S: Verify,
    M: Serialize,
  {
    self.verifiable(|data| suite.verify(message, data, public))
  }

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
