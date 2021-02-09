// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use identity_core::common::Object;

use crate::verification::Method;
use crate::verification::MethodScope;

/// A queried `Method` with additional metadata about the query resolution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodWrap<'method, T = Object> {
  pub(crate) method: &'method Method<T>,
  pub(crate) index: usize,
  pub(crate) scope: MethodScope,
}

impl<'method, T> MethodWrap<'method, T> {
  /// Creates a new `MethodWrap`.
  pub(crate) const fn new(method: &'method Method<T>, index: usize, scope: MethodScope) -> Self {
    Self { index, method, scope }
  }

  /// Returns the index of the method within the verification relationship set.
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Returns the scope of the resolved verification method.
  pub const fn scope(&self) -> MethodScope {
    self.scope
  }

  /// Consumes the `MethodWrap` and returns a reference to the resolved `Method`.
  pub const fn into_method(self) -> &'method Method<T> {
    self.method
  }
}

impl<T> Deref for MethodWrap<'_, T> {
  type Target = Method<T>;

  fn deref(&self) -> &Self::Target {
    self.method
  }
}
