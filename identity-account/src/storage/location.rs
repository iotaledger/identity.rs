// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use std::borrow::Cow;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct KeyLocation<'a> {
  type_: KeyType,
  identity: u32,
  fragment: Cow<'a, str>,
}

impl<'a> KeyLocation<'a> {
  pub fn borrowed(type_: KeyType, identity: u32, fragment: &'a str) -> Self {
    Self {
      type_,
      identity,
      fragment: Cow::Borrowed(fragment),
    }
  }

  pub fn owned(type_: KeyType, identity: u32, fragment: String) -> Self {
    Self {
      type_,
      identity,
      fragment: Cow::Owned(fragment),
    }
  }

  pub fn type_(&self) -> KeyType {
    self.type_
  }

  pub fn identity(&self) -> u32 {
    self.identity
  }

  pub fn fragment(&self) -> &str {
    &self.fragment
  }
}
