// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_core::crypto::Signature;

use crate::did::CoreDID;
use crate::did::CoreDIDUrl;
use crate::did::DIDUrl;
use crate::did::RelativeDIDUrl;
use crate::did::DID;

/// Specifies the conditions of a DID document method resolution query.
///
/// See `Document::resolve`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MethodQuery<'query>(Cow<'query, str>);

impl<'query> MethodQuery<'query> {
  pub(crate) fn matches(&self, did: &CoreDIDUrl) -> bool {
    match self.fragment().zip(did.fragment()) {
      Some((a, b)) => a == b,
      None => false,
    }
  }

  fn fragment(&self) -> Option<&str> {
    let query = self.0.as_ref();
    if query.starts_with(CoreDID::SCHEME) && !query.ends_with('#') {
      // Extract the fragment from a full DID-like string
      query.rfind('#').map(|index| &query[index + 1..])
    } else if let Some(stripped) = query.strip_prefix('#') {
      // Remove the leading `#` if it was in the query
      Some(stripped)
    } else {
      Some(query)
    }
  }
}

impl<'query> From<&'query str> for MethodQuery<'query> {
  fn from(other: &'query str) -> Self {
    Self(Cow::Borrowed(other))
  }
}

impl<'query> From<&'query String> for MethodQuery<'query> {
  fn from(other: &'query String) -> Self {
    Self(Cow::Borrowed(&**other))
  }
}

impl<'query, T> From<&'query DIDUrl<T>> for MethodQuery<'query>
where
  T: DID,
{
  fn from(other: &'query DIDUrl<T>) -> Self {
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query, T> From<DIDUrl<T>> for MethodQuery<'query>
where
  T: DID,
{
  fn from(other: DIDUrl<T>) -> Self {
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query> From<&'query RelativeDIDUrl> for MethodQuery<'query> {
  fn from(other: &'query RelativeDIDUrl) -> Self {
    // TODO: improve RelativeDIDUrl performance - internal string segments representation
    Self(Cow::Owned(other.to_string()))
  }
}

impl<'query> From<&'query Signature> for MethodQuery<'query> {
  fn from(other: &'query Signature) -> Self {
    Self(Cow::Borrowed(other.verification_method()))
  }
}
