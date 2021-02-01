// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_url::DID;

/// Index or identifier used to identify the target verification method of a
/// `MethodQuery`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodIdent<'a> {
  Index(usize),
  Ident(&'a str),
}

impl<'a> MethodIdent<'a> {
  /// Returns a `bool` indicating if the given `DID` matches the identifier.
  pub fn matches(&self, did: &DID) -> bool {
    match self {
      Self::Index(_) => false,
      Self::Ident(ident) if ident.starts_with(DID::SCHEME) && !ident.ends_with('#') => ident
        .rfind('#')
        .map_or(false, |index| Self::matches_fragment(did, &ident[index + 1..])),
      Self::Ident(ident) if ident.starts_with('#') => Self::matches_fragment(did, &ident[1..]),
      Self::Ident(ident) => Self::matches_fragment(did, *ident),
    }
  }

  fn matches_fragment(did: &DID, ident: &str) -> bool {
    matches!(did.fragment(), Some(fragment) if fragment == ident)
  }
}

impl<'a> From<&'a str> for MethodIdent<'a> {
  fn from(other: &'a str) -> Self {
    Self::Ident(other)
  }
}

impl From<usize> for MethodIdent<'_> {
  fn from(other: usize) -> Self {
    Self::Index(other)
  }
}

impl PartialEq<usize> for MethodIdent<'_> {
  fn eq(&self, other: &usize) -> bool {
    matches!(self, Self::Index(index) if index == other)
  }
}

impl PartialEq<&'_ str> for MethodIdent<'_> {
  fn eq(&self, other: &&'_ str) -> bool {
    matches!(self, Self::Ident(ident) if ident == other)
  }
}
