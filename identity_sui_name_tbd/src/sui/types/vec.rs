use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecSet<T: Eq + Hash> {
  pub contents: HashSet<T>,
}

impl<T: Eq + Hash> Deref for VecSet<T> {
  type Target = HashSet<T>;
  fn deref(&self) -> &Self::Target {
    &self.contents
  }
}

impl<T: Eq + Hash> DerefMut for VecSet<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.contents
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry<K, V> {
  pub key: K,
  pub value: V,
}

impl<K, V> From<Entry<K, V>> for (K, V) {
  fn from(entry: Entry<K, V>) -> Self {
    (entry.key, entry.value)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VecMap<K, V> {
  pub contents: Vec<Entry<K, V>>,
}

impl<K, V> Deref for VecMap<K, V> {
  type Target = [Entry<K, V>];
  fn deref(&self) -> &Self::Target {
    &self.contents[..]
  }
}

impl<K: Hash + Eq, V> From<VecMap<K, V>> for HashMap<K, V> {
  fn from(value: VecMap<K, V>) -> Self {
    value.contents.into_iter().map(|e| e.into()).collect()
  }
}

impl<K, V> VecMap<K, V> {
  pub fn into_inner_iter(self) -> impl Iterator<Item = (K, V)> {
    self.contents.into_iter().map(|e| e.into())
  }
}
