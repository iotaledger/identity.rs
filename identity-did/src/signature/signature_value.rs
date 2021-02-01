// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::cell::Cell;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::signature::SignatureData;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureValue {
  data: SignatureData,
  #[serde(skip)]
  hide: Cell<bool>,
}

impl SignatureValue {
  pub const fn new() -> Self {
    Self {
      data: SignatureData::None,
      hide: Cell::new(false),
    }
  }

  pub fn is_none(&self) -> bool {
    self.data.is_none() || self.hide.get()
  }

  pub fn set(&mut self, value: SignatureData) {
    self.data = value;
  }

  pub fn clear(&mut self) {
    self.set(SignatureData::None);
  }

  pub fn hide(&self) {
    self.hide.set(true);
  }

  pub fn show(&self) {
    self.hide.set(false);
  }
}

impl Debug for SignatureValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.data, f)
  }
}

impl Default for SignatureValue {
  fn default() -> Self {
    Self::new()
  }
}

impl Deref for SignatureValue {
  type Target = SignatureData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for SignatureValue {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}
