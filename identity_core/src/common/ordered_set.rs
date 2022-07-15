// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::iter::FromIterator;
use core::ops::Deref;
use core::slice::Iter;
use core::slice::IterMut;
use std::vec::IntoIter;

use serde::Deserialize;
use serde::Serialize;

use identity_diff::Diff;
use identity_diff::DiffVec;

use crate::common::KeyComparable;
use crate::error::Error;
use crate::error::Result;

/// An ordered set backed by a `Vec<T>`.
///
/// Note: Ordering is based on insert order and **not** [`Ord`].
///
/// See the [Infra standard definition](https://infra.spec.whatwg.org/#ordered-set).
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(bound(deserialize = "T: KeyComparable + Deserialize<'de>"), try_from = "Vec<T>")]
pub struct OrderedSet<T>(Vec<T>);

impl<T> OrderedSet<T> {
  /// Creates a new `OrderedSet`.
  #[inline]
  pub const fn new() -> Self {
    Self(Vec::new())
  }

  /// Creates a new `OrderedSet` with the specified capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self(Vec::with_capacity(capacity))
  }

  /// Returns the number of elements in the `OrderedSet`.
  #[inline]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns `true` if the `OrderedSet` contains no elements.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns an iterator over the slice of elements.
  #[inline]
  pub fn iter(&self) -> Iter<'_, T> {
    self.0.iter()
  }

  /// Returns an iterator that allows modifying each value.
  ///
  /// WARNING: improper usage of this allows violating the key-uniqueness of the OrderedSet.
  #[inline]
  pub fn iter_mut_unchecked(&mut self) -> IterMut<'_, T> {
    self.0.iter_mut()
  }

  /// Returns the first element in the set, or `None` if the set is empty.
  #[inline]
  pub fn head(&self) -> Option<&T> {
    self.0.first()
  }

  /// Returns a mutable referece to the first element in the set, or `None` if
  /// the set is empty.
  #[inline]
  pub fn head_mut(&mut self) -> Option<&mut T> {
    self.0.first_mut()
  }

  /// Returns the last element in the set, or `None` if the set is empty.
  #[inline]
  pub fn tail(&self) -> Option<&T> {
    self.0.last()
  }

  /// Returns a mutable reference the last element in the set, or `None` if the
  /// set is empty.
  #[inline]
  pub fn tail_mut(&mut self) -> Option<&mut T> {
    self.0.last_mut()
  }

  /// Returns a slice containing all elements in the `OrderedSet`.
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    &self.0
  }

  /// Consumes the `OrderedSet` and returns the elements as a `Vec<T>`.
  #[inline]
  pub fn into_vec(self) -> Vec<T> {
    self.0
  }

  /// Clears the `OrderedSet`, removing all values.
  #[inline]
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// Returns `true` if the `OrderedSet` contains the given value.
  pub fn contains<U>(&self, item: &U) -> bool
  where
    T: KeyComparable,
    U: KeyComparable<Key = T::Key> + ?Sized,
  {
    self.0.iter().any(|other| other.key() == item.key())
  }

  /// Adds a new value to the end of the `OrderedSet`; returns `true` if the
  /// value was successfully added.
  pub fn append(&mut self, item: T) -> bool
  where
    T: KeyComparable,
  {
    if self.contains(&item) {
      false
    } else {
      self.0.push(item);
      true
    }
  }

  /// Adds a new value to the start of the `OrderedSet`; returns `true` if the
  /// value was successfully added.
  pub fn prepend(&mut self, item: T) -> bool
  where
    T: KeyComparable,
  {
    if self.contains(&item) {
      false
    } else {
      self.0.insert(0, item);
      true
    }
  }

  /// Replaces a `current` value with the given `update` value; returns `true`
  /// if the value was successfully replaced.
  #[inline]
  pub fn replace<U>(&mut self, current: &U, update: T) -> bool
  where
    T: KeyComparable,
    U: KeyComparable<Key = T::Key>,
  {
    self.change(update, |item, update| {
      item.key() == current.key() || item.key() == update.key()
    })
  }

  /// Updates an existing value in the `OrderedSet`; returns `true` if the value
  /// was successfully updated.
  #[inline]
  pub fn update(&mut self, update: T) -> bool
  where
    T: KeyComparable,
  {
    self.change(update, |item, update| item.key() == update.key())
  }

  /// Removes all matching items from the set.
  #[inline]
  pub fn remove<U>(&mut self, item: &U) -> bool
  where
    T: KeyComparable,
    U: KeyComparable<Key = T::Key>,
  {
    if self.contains(item) {
      self.0.retain(|this| this.borrow().key() != item.key());
      true
    } else {
      false
    }
  }

  fn change<F>(&mut self, data: T, f: F) -> bool
  where
    F: Fn(&T, &T) -> bool,
  {
    let index: Option<usize> = self.0.iter().position(|item| f(item, &data));

    if let Some(index) = index {
      let keep: Vec<T> = self.0.drain(index..).filter(|item| !f(item, &data)).collect();

      self.0.extend(keep);
      self.0.insert(index, data);
    }

    index.is_some()
  }
}

impl<T> IntoIterator for OrderedSet<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<T> Debug for OrderedSet<T>
where
  T: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_set().entries(self.iter()).finish()
  }
}

impl<T> Deref for OrderedSet<T> {
  type Target = [T];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: KeyComparable> Default for OrderedSet<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T: KeyComparable> FromIterator<T> for OrderedSet<T>
where
  T: KeyComparable,
{
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    let iter: _ = iter.into_iter();
    let size: usize = iter.size_hint().1.unwrap_or(0);

    let mut this: Self = Self::with_capacity(size);

    // Ignore duplicates.
    for item in iter {
      this.append(item);
    }

    this
  }
}

impl<T> TryFrom<Vec<T>> for OrderedSet<T>
where
  T: KeyComparable,
{
  type Error = Error;

  fn try_from(other: Vec<T>) -> Result<Self, Self::Error> {
    let mut this: Self = Self::with_capacity(other.len());

    for item in other {
      if !this.append(item) {
        return Err(Error::OrderedSetDuplicate);
      }
    }

    Ok(this)
  }
}

impl<T> Diff for OrderedSet<T>
where
  T: Diff + KeyComparable + Serialize + for<'de> Deserialize<'de>,
{
  type Type = DiffVec<T>;

  fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
    self.clone().into_vec().diff(&other.clone().into_vec())
  }

  fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
    self
      .clone()
      .into_vec()
      .merge(diff)
      .and_then(|this| Self::try_from(this).map_err(identity_diff::Error::merge))
  }

  fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
    Vec::from_diff(diff).and_then(|this| Self::try_from(this).map_err(identity_diff::Error::convert))
  }

  fn into_diff(self) -> identity_diff::Result<Self::Type> {
    self.into_vec().into_diff()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ordered_set_works() {
    let mut set = OrderedSet::new();

    set.append("a");
    set.append("b");
    set.append("c");

    assert_eq!(set.as_slice(), &["a", "b", "c"]);
    assert_eq!(set.head(), Some(&"a"));
    assert_eq!(set.tail(), Some(&"c"));

    set.replace(&"a", "c");

    assert_eq!(set.as_slice(), &["c", "b"]);

    let mut set = OrderedSet::new();

    set.prepend("a");
    set.prepend("b");
    set.prepend("c");

    assert_eq!(set.as_slice(), &["c", "b", "a"]);
    assert_eq!(set.head(), Some(&"c"));
    assert_eq!(set.tail(), Some(&"a"));

    set.replace(&"a", "c");

    assert_eq!(set.as_slice(), &["c", "b"]);
  }

  #[test]
  fn test_from_vec_valid() {
    let source: Vec<u8> = vec![3, 1, 2, 0];
    let set: OrderedSet<u8> = OrderedSet::try_from(source).unwrap();

    assert_eq!(&*set, &[3, 1, 2, 0]);
  }

  #[test]
  #[should_panic = "OrderedSetDuplicate"]
  fn test_from_vec_invalid() {
    let source: Vec<u8> = vec![1, 2, 2, 5];
    let _: OrderedSet<u8> = OrderedSet::try_from(source).unwrap();
  }

  #[test]
  fn test_collect() {
    let source: Vec<u8> = vec![1, 2, 3, 3, 2, 4, 5, 1, 1];
    let set: OrderedSet<u8> = source.into_iter().collect();

    assert_eq!(&*set, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_contains() {
    let cs1 = ComparableStruct { key: 0, value: 10 };
    let cs2 = ComparableStruct { key: 1, value: 20 };
    let cs3 = ComparableStruct { key: 2, value: 10 };
    let cs4 = ComparableStruct { key: 3, value: 20 };

    let source: Vec<ComparableStruct> = vec![cs1, cs2];
    let set: OrderedSet<ComparableStruct> = source.into_iter().collect();

    assert!(set.contains(&cs1));
    assert!(set.contains(&cs2));
    assert!(!set.contains(&cs3));
    assert!(!set.contains(&cs4));
  }

  #[derive(Clone, Copy, PartialEq, Eq)]
  struct ComparableStruct {
    key: u8,
    value: i32,
  }

  impl KeyComparable for ComparableStruct {
    type Key = u8;

    #[inline]
    fn key(&self) -> &Self::Key {
      &self.key
    }
  }

  #[test]
  fn test_ordered_set_replace() {
    let mut set = OrderedSet::new();

    // Create two structs with the same key.
    let cs1 = ComparableStruct { key: 0, value: 10 };
    let cs2 = ComparableStruct { key: 0, value: 20 };

    // Try replace it with the second.
    // This should succeed because the keys are equivalent.
    assert!(set.append(cs1));
    assert_eq!(set.len(), 1);

    assert!(set.replace(&cs1, cs2));
    assert_eq!(set.len(), 1);
    assert_eq!(set.head().unwrap().key, cs2.key);
    assert_eq!(set.head().unwrap().value, cs2.value);
  }

  #[test]
  fn test_ordered_set_replace_all() {
    let mut set = OrderedSet::new();
    let cs1 = ComparableStruct { key: 0, value: 10 };
    let cs2 = ComparableStruct { key: 1, value: 20 };
    assert!(set.append(cs1));
    assert!(set.append(cs2));
    assert_eq!(set.len(), 2);

    // Now replace cs1 with something that has the same key as cs2.
    // This should replace BOTH cs1 AND cs2.
    let cs3 = ComparableStruct { key: 1, value: 30 };
    assert!(set.replace(&cs1, cs3));
    assert_eq!(set.len(), 1);
    assert_eq!(set.head().unwrap().key, cs3.key);
    assert_eq!(set.head().unwrap().value, cs3.value);
  }

  #[test]
  fn test_ordered_set_update() {
    let mut set = OrderedSet::new();
    let cs1 = ComparableStruct { key: 0, value: 10 };
    assert!(set.append(cs1));
    assert_eq!(set.len(), 1);

    // This should update the value of cs1 since the keys are the same.
    let cs2 = ComparableStruct { key: 0, value: 20 };
    assert!(set.update(cs2));
    assert_eq!(set.len(), 1);
    assert_eq!(set.head().unwrap().key, cs2.key);
    assert_eq!(set.head().unwrap().value, cs2.value);

    // This should NOT update anything since the key does not match.
    let cs3 = ComparableStruct { key: 1, value: 20 };
    assert!(!set.update(cs3));
    assert_eq!(set.len(), 1);
    assert_eq!(set.head().unwrap().key, cs2.key);
    assert_eq!(set.head().unwrap().value, cs2.value);
  }
}
