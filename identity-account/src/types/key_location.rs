// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use identity_core::common::Fragment;
use identity_did::verification::MethodType;

use crate::types::Generation;

/// The storage location of a verification method key.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyLocation {
  pub(crate) method: MethodType,
  pub(crate) fragment: Fragment,
  pub(crate) integration_generation: Generation,
  pub(crate) diff_generation: Generation,
}

impl KeyLocation {
  pub(crate) const AUTH: &'static str = "_sign-";

  // Creates a new KeyLocation for an authentication method.
  pub fn new_authentication(method: MethodType, generation: Generation) -> Self {
    let fragment: String = format!("{}{}", Self::AUTH, generation.to_u32());

    Self {
      method,
      fragment: Fragment::new(fragment),
      integration_generation: generation,
      diff_generation: Generation::new(),
    }
  }

  /// Returns the method type of the key location.
  pub fn method(&self) -> MethodType {
    self.method
  }

  /// Returns the fragment name of the key location.
  pub fn fragment(&self) -> &str {
    self.fragment.name()
  }

  /// Returns the integration generation when this key was created.
  pub fn integration_generation(&self) -> Generation {
    self.integration_generation
  }

  /// Returns the diff generation when this key was created.
  pub fn diff_generation(&self) -> Generation {
    self.diff_generation
  }

  /// Returns true if the key location points to an authentication method.
  pub fn is_authentication(&self) -> bool {
    Self::is_authentication_fragment(&self.fragment)
  }

  /// Returns true if the fragment points to an authentication method.
  pub fn is_authentication_fragment(fragment: &Fragment) -> bool {
    fragment.name().starts_with(Self::AUTH)
  }
}

impl Display for KeyLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!(
      "({}:{}:{}:{})",
      self.integration_generation,
      self.diff_generation,
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
