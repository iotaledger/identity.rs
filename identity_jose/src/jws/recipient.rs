// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jws::JwsHeader;

#[derive(Clone, Copy)]
pub struct Recipient<'a> {
  /// The integrity-protected JOSE header.
  pub protected: Option<&'a JwsHeader>,
  /// The non integrity-protected JOSE header.
  pub unprotected: Option<&'a JwsHeader>,
}

impl<'a> Default for Recipient<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Recipient<'a> {
  pub fn new() -> Self {
    Self {
      protected: None,
      unprotected: None,
    }
  }

  pub fn protected(mut self, value: &'a JwsHeader) -> Self {
    self.protected = Some(value);
    self
  }

  pub fn unprotected(mut self, value: &'a JwsHeader) -> Self {
    self.unprotected = Some(value);
    self
  }
}
