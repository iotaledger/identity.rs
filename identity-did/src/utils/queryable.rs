// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::KeyComparable;
use identity_core::common::OrderedSet;

use crate::did::CoreDIDUrl;
use crate::utils::DIDUrlQuery;

pub trait Queryable<T, Q> {
  fn query(&self, query: Q) -> Option<&T>;
  fn query_mut(&mut self, query: Q) -> Option<&mut T>;
}

impl<'query, T, Q> Queryable<T, Q> for OrderedSet<T>
where
  T: KeyComparable,
  <T as KeyComparable>::Key: AsRef<CoreDIDUrl>,
  Q: Into<DIDUrlQuery<'query>>,
{
  fn query(&self, query: Q) -> Option<&T> {
    let query: DIDUrlQuery<'query> = query.into();
    self.iter().find(|entry| query.matches(entry.key().as_ref()))
  }

  /// WARNING: improper usage of this allows violating the key-uniqueness of the OrderedSet.
  fn query_mut(&mut self, query: Q) -> Option<&mut T> {
    let query: DIDUrlQuery<'query> = query.into();
    self
      .iter_mut_unchecked()
      .find(|entry| query.matches(entry.key().as_ref()))
  }
}
