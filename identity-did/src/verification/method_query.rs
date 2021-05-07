// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::Signature;

use crate::did::DID;

/// Specifies the conditions of a DID document method resolution query.
///
/// See `Document::resolve`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MethodQuery<'query>(&'query str);

impl<'query> MethodQuery<'query> {
  pub(crate) fn matches(&self, did: &DID) -> bool {
    match self.fragment().zip(did.fragment()) {
      Some((a, b)) => a == b,
      None => false,
    }
  }

  fn fragment(&self) -> Option<&str> {
    if self.0.starts_with(DID::SCHEME) && !self.0.ends_with('#') {
      // Extract the fragment from a full DID-like string
      self.0.rfind('#').map(|index| &self.0[index + 1..])
    } else if self.0.starts_with('#') {
      // Remove the leading `#` if it was in the query
      Some(&self.0[1..])
    } else {
      Some(self.0)
    }
  }
}

impl<'query> From<&'query str> for MethodQuery<'query> {
  fn from(other: &'query str) -> Self {
    Self(other)
  }
}

impl<'query> From<&'query String> for MethodQuery<'query> {
  fn from(other: &'query String) -> Self {
    Self(&**other)
  }
}

impl<'query> From<&'query DID> for MethodQuery<'query> {
  fn from(other: &'query DID) -> Self {
    Self(other.as_str())
  }
}

impl<'query> From<&'query Signature> for MethodQuery<'query> {
  fn from(other: &'query Signature) -> Self {
    Self(other.verification_method())
  }
}
