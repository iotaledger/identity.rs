// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::cell::Cell;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::crypto::SignatureData;

/// A [`SignatureData`] wrapper with a visiblity toggle.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureValue {
  data: SignatureData,
  #[serde(skip)]
  hide: Cell<bool>,
}

impl SignatureValue {
  /// Creates a new [`SignatureValue`].
  pub const fn new() -> Self {
    Self {
      data: SignatureData::None,
      hide: Cell::new(false),
    }
  }

  /// Returns `true` if the value is empty.
  pub fn is_none(&self) -> bool {
    self.data.is_none() || self.hide.get()
  }

  /// Sets the value of the underlying [`SignatureData`].
  pub fn set(&mut self, value: SignatureData) {
    self.data = value;
  }

  /// Clears the value of the underlying [`SignatureData`].
  pub fn clear(&mut self) {
    self.set(SignatureData::None);
  }

  /// Flag the signature value as "hidden".
  pub fn hide(&self) {
    self.hide.set(true);
  }

  /// Flag the signature value as "visible".
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
