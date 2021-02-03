// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Result;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

// =============================================================================
// =============================================================================

pub trait ResolveMethod<M> {
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, M>>;

  fn try_resolve_method(&self, query: MethodQuery<'_>) -> Result<MethodWrap<'_, M>> {
    self.resolve_method(query).ok_or(Error::QueryMethodNotFound)
  }
}

impl<'a, T, M> ResolveMethod<M> for &'a T
where
  T: ResolveMethod<M>,
{
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, M>> {
    (**self).resolve_method(query)
  }
}
