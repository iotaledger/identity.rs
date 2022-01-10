// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Formatter;

use core::iter::FromIterator;
use core::ops::Deref;
use core::slice::Iter;
use serde::Deserialize;

use crate::did::CoreDIDUrl;
use crate::error::Error;
use crate::error::Result;
use crate::utils::KeyComparable;
use crate::verification::MethodQuery;

/// An ordered set backed by a `Vec<T>`.
///
/// Note: Ordering is based on insert order and **not** [`Ord`].
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
    U: KeyComparable<Key = T::Key>,
  {
    self.0.iter().any(|other| other.as_key() == item.as_key())
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
      item.as_key() == current.as_key() || item.as_key() == update.as_key()
    })
  }

  /// Updates an existing value in the `OrderedSet`; returns `true` if the value
  /// was successfully updated.
  #[inline]
  pub fn update(&mut self, update: T) -> bool
  where
    T: KeyComparable,
  {
    self.change(update, |item, update| item.as_key() == update.as_key())
  }

  /// Removes all matching items from the set.
  #[inline]
  pub fn remove<U>(&mut self, item: &U) -> bool
  where
    T: KeyComparable,
    U: KeyComparable<Key = T::Key>,
  {
    if self.contains(item) {
      self.0.retain(|this| this.borrow().as_key() != item.as_key());
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

impl<T> OrderedSet<T>
where
  T: AsRef<CoreDIDUrl>,
{
  pub fn query<'query, Q>(&self, query: Q) -> Option<&T>
  where
    Q: Into<MethodQuery<'query>>,
  {
    let query: MethodQuery<'query> = query.into();

    self.0.iter().find(|method| query.matches(method.as_ref()))
  }

  pub(crate) fn query_mut<'query, Q>(&mut self, query: Q) -> Option<&mut T>
  where
    Q: Into<MethodQuery<'query>>,
  {
    let query: MethodQuery<'query> = query.into();

    self.0.iter_mut().find(|method| query.matches(method.as_ref()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::did::CoreDIDUrl;
  use crate::verification::MethodRef;

  impl KeyComparable for &str {
    type Key = str;

    fn as_key(&self) -> &Self::Key {
      self
    }
  }

  impl KeyComparable for u8 {
    type Key = u8;

    fn as_key(&self) -> &Self::Key {
      self
    }
  }

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
    let oset: OrderedSet<u8> = OrderedSet::try_from(source).unwrap();

    assert_eq!(&*oset, &[3, 1, 2, 0]);
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
    let oset: OrderedSet<u8> = source.into_iter().collect();

    assert_eq!(&*oset, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_contains() {
    let did1: CoreDIDUrl = CoreDIDUrl::parse("did:example:123").unwrap();
    let did2: CoreDIDUrl = CoreDIDUrl::parse("did:example:456").unwrap();

    let source: Vec<MethodRef> = vec![MethodRef::Refer(did1.clone()), MethodRef::Refer(did2.clone())];

    let oset: OrderedSet<MethodRef> = source.into_iter().collect();

    assert!(oset.contains(&MethodRef::<()>::Refer(did1)));
    assert!(oset.contains(&MethodRef::<()>::Refer(did2)));
  }

  #[derive(Clone, Copy, PartialEq, Eq)]
  struct ComparableStruct {
    key: u8,
    value: i32,
  }

  impl KeyComparable for ComparableStruct {
    type Key = u8;

    fn as_key(&self) -> &Self::Key {
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
