// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::Signature;

use crate::verification::MethodIdent;
use crate::verification::MethodScope;

/// Specifies the  conditions of a DID document method resolution query.
///
/// See `Document::resolve`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodQuery<'ident> {
  pub(crate) ident: MethodIdent<'ident>,
  pub(crate) scope: MethodScope,
}

impl<'ident> MethodQuery<'ident> {
  /// Creates a new `MethodQuery`.
  pub fn new<T>(ident: T) -> Self
  where
    T: Into<MethodIdent<'ident>>,
  {
    Self::with_scope(ident, MethodScope::default())
  }

  /// Creates a new `MethodQuery` with the given `MethodScope`.
  pub fn with_scope<T>(ident: T, scope: MethodScope) -> Self
  where
    T: Into<MethodIdent<'ident>>,
  {
    Self {
      ident: ident.into(),
      scope,
    }
  }

  pub(crate) fn scoped(self, scope: MethodScope) -> Self {
    Self {
      ident: self.ident,
      scope,
    }
  }
}

impl<'ident> From<&'ident str> for MethodQuery<'ident> {
  fn from(other: &'ident str) -> Self {
    Self::new(other)
  }
}

impl<'ident> From<&'ident String> for MethodQuery<'ident> {
  fn from(other: &'ident String) -> Self {
    Self::new(&**other)
  }
}

impl From<usize> for MethodQuery<'_> {
  fn from(other: usize) -> Self {
    Self::new(other)
  }
}

impl<'ident> From<(&'ident str, MethodScope)> for MethodQuery<'ident> {
  fn from(other: (&'ident str, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl From<(usize, MethodScope)> for MethodQuery<'_> {
  fn from(other: (usize, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl<'ident> From<(MethodIdent<'ident>, MethodScope)> for MethodQuery<'ident> {
  fn from(other: (MethodIdent<'ident>, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl<'ident> From<MethodScope> for MethodQuery<'ident> {
  fn from(other: MethodScope) -> Self {
    Self::with_scope(0, other)
  }
}

impl<'ident> From<&'ident Signature> for MethodQuery<'ident> {
  fn from(other: &'ident Signature) -> Self {
    Self::new(other.verification_method())
  }
}
