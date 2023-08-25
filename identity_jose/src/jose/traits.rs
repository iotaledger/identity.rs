// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwt::JwtHeader;

/// An abstraction over different JOSE headers.
pub trait JoseHeader {
  /// Returns the header common to all [`JoseHeader`]s.
  fn common(&self) -> &JwtHeader;

  /// Returns `true` if the header has the given `claim`, `false` otherwise.
  fn has_claim(&self, claim: &str) -> bool;
}

impl<'a, T: 'a> JoseHeader for &'a T
where
  T: JoseHeader,
{
  fn common(&self) -> &JwtHeader {
    (**self).common()
  }

  fn has_claim(&self, claim: &str) -> bool {
    (**self).has_claim(claim)
  }
}
