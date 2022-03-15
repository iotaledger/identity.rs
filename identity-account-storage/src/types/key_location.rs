// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::common::Fragment;
use identity_did::verification::MethodType;
use serde::Deserialize;
use serde::Serialize;

use crate::types::Generation;

/// The storage location of a verification method key.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyLocation {
  method: MethodType,
  fragment: Fragment,
  generation: Generation,
}

impl KeyLocation {
  /// Creates a new `KeyLocation`.
  pub fn new(method: MethodType, fragment: String, generation: Generation) -> Self {
    Self {
      method,
      fragment: Fragment::new(fragment),

      generation,
    }
  }

  /// Returns the method type of the key location.
  pub fn method(&self) -> MethodType {
    self.method
  }

  /// Returns the fragment name of the key location.
  pub fn fragment(&self) -> &Fragment {
    &self.fragment
  }

  /// Returns the fragment name of the key location.
  pub fn fragment_name(&self) -> &str {
    self.fragment.name()
  }

  /// Returns the integration generation when this key was created.
  pub fn generation(&self) -> Generation {
    self.generation
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!(
      "({}:{}:{})",
      self.generation,
      self.fragment,
      self.method.as_u32()
    ))
  }
}

impl Debug for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("KeyLocation{}", self))
  }
}
