// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DID;

#[derive(Clone, Copy, Debug)]
pub enum Key<'a> {
  DID(&'a DID),
  Ident(&'a str),
  Index(u32),
}

impl<'a> From<&'a DID> for Key<'a> {
  fn from(other: &'a DID) -> Self {
    Self::DID(other)
  }
}

impl<'a> From<&'a str> for Key<'a> {
  fn from(other: &'a str) -> Self {
    Self::Ident(other)
  }
}

impl From<u32> for Key<'_> {
  fn from(other: u32) -> Self {
    Self::Index(other)
  }
}
