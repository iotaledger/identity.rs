// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use fxhash::FxBuildHasher;
use hashbrown::hash_map;
use hashbrown::HashMap;

use crate::types::Index;

pub type Entry<'a, T> = hash_map::Entry<'a, Index, T, FxBuildHasher>;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct IndexMap<T>(HashMap<Index, T, FxBuildHasher>);

impl<T> IndexMap<T> {
  pub fn new() -> Self {
    Self(HashMap::with_hasher(Default::default()))
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn get(&self, index: Index) -> Option<&T> {
    self.0.get(&index)
  }

  pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
    self.0.get_mut(&index)
  }

  pub fn entry(&mut self, index: Index) -> Entry<'_, T> {
    self.0.entry(index)
  }

  pub fn exists(&self, index: Index) -> bool {
    self.0.contains_key(&index)
  }

  pub fn insert(&mut self, index: Index, item: T) -> bool {
    let insert: bool = !self.full_capacity();

    if insert {
      self.0.insert(index, item);
    }

    insert
  }

  fn full_capacity(&self) -> bool {
    self.0.len() >= Index::MAX.to_u32() as usize
  }
}

impl<T: Debug> Debug for IndexMap<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_map().entries(self.0.iter()).finish()
  }
}

impl<T> Default for IndexMap<T> {
  fn default() -> Self {
    Self::new()
  }
}
