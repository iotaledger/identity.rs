// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;

#[derive(Clone, Copy, Debug)]
pub struct KeyLocation<'a> {
  identity: u32,
  fragment: &'a str,
  type_: KeyType,
}

impl<'a> KeyLocation<'a> {
  pub fn new(identity: u32, fragment: &'a str, type_: KeyType) -> Self {
    Self {
      identity,
      fragment,
      type_,
    }
  }

  pub fn identity(&self) -> u32 {
    self.identity
  }

  pub fn fragment(&self) -> &str {
    self.fragment
  }

  pub fn type_(&self) -> KeyType {
    self.type_
  }

  pub fn location(&self) -> String {
    format!("{}/{:?}/{}", self.identity, self.type_, self.fragment)
  }
}
