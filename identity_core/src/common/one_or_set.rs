// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::hash::Hash;
use core::iter;
use core::mem::replace;
use core::ops::Deref;
use core::slice::from_ref;

use serde::de;
use serde::Deserialize;
use serde::Serialize;

use crate::common::KeyComparable;
use crate::common::OrderedSet;
use crate::error::Error;
use crate::error::Result;

/// A generic container that stores exactly one or more unique instances of a given type.
///
/// Similar to [`OneOrMany`](crate::common::OneOrMany) except instances are guaranteed to be unique,
/// and only immutable references are allowed.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OneOrSet<T>(OneOrSetInner<T>)
where
  T: KeyComparable;

// Private to prevent creations of empty `Set` variants.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
enum OneOrSetInner<T>
where
  T: KeyComparable,
{
  /// A single instance of `T`.
  One(T),
  /// Multiple (one or more) unique instances of `T`.
  #[serde(deserialize_with = "deserialize_non_empty_set")]
  Set(OrderedSet<T>),
}

/// Deserializes an [`OrderedSet`] while enforcing that it is non-empty.
fn deserialize_non_empty_set<'de, D, T: serde::Deserialize<'de> + KeyComparable>(
  deserializer: D,
) -> Result<OrderedSet<T>, D::Error>
where
  D: de::Deserializer<'de>,
{
  let set: OrderedSet<T> = OrderedSet::deserialize(deserializer)?;
  if set.is_empty() {
    return Err(de::Error::custom(Error::OneOrSetEmpty));
  }

  Ok(set)
}

impl<T> OneOrSet<T>
where
  T: KeyComparable,
{
  /// Constructs a new instance with a single item.
  pub fn new_one(item: T) -> Self {
    Self(OneOrSetInner::One(item))
  }

  /// Constructs a new instance from a set of unique items.
  ///
  /// Errors if the given set is empty.
  pub fn new_set(set: OrderedSet<T>) -> Result<Self> {
    if set.is_empty() {
      return Err(Error::OneOrSetEmpty);
    }
    if set.len() == 1 {
      Ok(Self::new_one(
        set.into_vec().pop().expect("infallible OneOrSet new_set"),
      ))
    } else {
      Ok(Self(OneOrSetInner::Set(set)))
    }
  }

  /// Apply a map function to convert this into a new `OneOrSet<S>`.
  pub fn map<S, F>(self, mut f: F) -> OneOrSet<S>
  where
    S: KeyComparable,
    F: FnMut(T) -> S,
  {
    OneOrSet(match self.0 {
      OneOrSetInner::One(item) => OneOrSetInner::One(f(item)),
      OneOrSetInner::Set(set_t) => {
        let set_s: OrderedSet<S> = set_t.into_vec().into_iter().map(f).collect();
        // Key equivalence could differ between T and S.
        if set_s.len() == 1 {
          OneOrSetInner::One(set_s.into_vec().pop().expect("OneOrSet::map infallible"))
        } else {
          OneOrSetInner::Set(set_s)
        }
      }
    })
  }

  /// Apply a map function to convert this into a new `OneOrSet<S>`.
  pub fn try_map<S, F, E>(self, mut f: F) -> Result<OneOrSet<S>, E>
  where
    S: KeyComparable,
    F: FnMut(T) -> Result<S, E>,
  {
    Ok(OneOrSet(match self.0 {
      OneOrSetInner::One(item) => OneOrSetInner::One(f(item)?),
      OneOrSetInner::Set(set_t) => {
        let set_s: OrderedSet<S> = set_t
          .into_vec()
          .into_iter()
          .map(f)
          .collect::<Result<OrderedSet<S>, E>>()?;
        // Key equivalence could differ between T and S.
        if set_s.len() == 1 {
          OneOrSetInner::One(set_s.into_vec().pop().expect("OneOrSet::try_map infallible"))
        } else {
          OneOrSetInner::Set(set_s)
        }
      }
    }))
  }

  /// Returns the number of elements in the collection.
  #[allow(clippy::len_without_is_empty)]
  pub fn len(&self) -> usize {
    match &self.0 {
      OneOrSetInner::One(_) => 1,
      OneOrSetInner::Set(inner) => inner.len(),
    }
  }

  /// Returns a reference to the element at the given index.
  pub fn get(&self, index: usize) -> Option<&T> {
    match &self.0 {
      OneOrSetInner::One(inner) if index == 0 => Some(inner),
      OneOrSetInner::One(_) => None,
      OneOrSetInner::Set(inner) => inner.get(index),
    }
  }

  /// Returns `true` if the collection contains the given item's key.
  pub fn contains<U>(&self, item: &U) -> bool
  where
    T: KeyComparable,
    U: KeyComparable<Key = T::Key> + ?Sized,
  {
    match &self.0 {
      OneOrSetInner::One(inner) => inner.key() == item.key(),
      OneOrSetInner::Set(inner) => inner.contains(item),
    }
  }

  /// Appends a new item to the end of the collection if its key is not present already.
  ///
  /// Returns whether or not the value was successfully inserted.
  pub fn append(&mut self, item: T) -> bool
  where
    T: KeyComparable,
  {
    match &mut self.0 {
      OneOrSetInner::One(inner) if inner.key() == item.key() => false,
      OneOrSetInner::One(_) => match replace(&mut self.0, OneOrSetInner::Set(OrderedSet::new())) {
        OneOrSetInner::One(inner) => {
          self.0 = OneOrSetInner::Set(OrderedSet::from_iter([inner, item]));
          true
        }
        OneOrSetInner::Set(_) => unreachable!(),
      },
      OneOrSetInner::Set(inner) => inner.append(item),
    }
  }

  /// Returns an `Iterator` that yields items from the collection.
  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    OneOrSetIter::new(self)
  }

  /// Returns a reference to the contents as a slice.
  pub fn as_slice(&self) -> &[T] {
    self
  }

  /// Consumes the [`OneOrSet`] and returns the contents as a [`Vec`].
  pub fn into_vec(self) -> Vec<T> {
    match self.0 {
      OneOrSetInner::One(inner) => vec![inner],
      OneOrSetInner::Set(inner) => inner.into_vec(),
    }
  }
}

impl<T> Debug for OneOrSet<T>
where
  T: Debug + KeyComparable,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match &self.0 {
      OneOrSetInner::One(inner) => Debug::fmt(inner, f),
      OneOrSetInner::Set(inner) => Debug::fmt(inner, f),
    }
  }
}

impl<T> Deref for OneOrSet<T>
where
  T: KeyComparable,
{
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    match &self.0 {
      OneOrSetInner::One(inner) => from_ref(inner),
      OneOrSetInner::Set(inner) => inner.as_slice(),
    }
  }
}

impl<T> AsRef<[T]> for OneOrSet<T>
where
  T: KeyComparable,
{
  fn as_ref(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T> From<T> for OneOrSet<T>
where
  T: KeyComparable,
{
  fn from(other: T) -> Self {
    OneOrSet::new_one(other)
  }
}

impl<T> TryFrom<Vec<T>> for OneOrSet<T>
where
  T: KeyComparable,
{
  type Error = Error;

  fn try_from(other: Vec<T>) -> std::result::Result<Self, Self::Error> {
    let set: OrderedSet<T> = OrderedSet::try_from(other)?;
    OneOrSet::new_set(set)
  }
}

impl<T> TryFrom<OrderedSet<T>> for OneOrSet<T>
where
  T: KeyComparable,
{
  type Error = Error;

  fn try_from(other: OrderedSet<T>) -> std::result::Result<Self, Self::Error> {
    OneOrSet::new_set(other)
  }
}

impl<T> From<OneOrSet<T>> for Vec<T>
where
  T: KeyComparable,
{
  fn from(other: OneOrSet<T>) -> Self {
    other.into_vec()
  }
}

impl<T> From<OneOrSet<T>> for OrderedSet<T>
where
  T: KeyComparable,
{
  fn from(other: OneOrSet<T>) -> Self {
    match other.0 {
      OneOrSetInner::One(item) => OrderedSet::from_iter(iter::once(item)),
      OneOrSetInner::Set(set) => set,
    }
  }
}

// =============================================================================
// Iterator
// =============================================================================

struct OneOrSetIter<'a, T>
where
  T: KeyComparable,
{
  inner: &'a OneOrSet<T>,
  index: usize,
}

impl<'a, T> OneOrSetIter<'a, T>
where
  T: KeyComparable,
{
  fn new(inner: &'a OneOrSet<T>) -> Self {
    Self { inner, index: 0 }
  }
}

impl<'a, T> Iterator for OneOrSetIter<'a, T>
where
  T: KeyComparable,
{
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    self.inner.get(self.index - 1)
  }
}

#[cfg(test)]
mod tests {
  use crate::convert::FromJson;
  use crate::convert::ToJson;

  use super::*;

  #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
  struct MockKeyU8(u8);

  #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
  struct MockKeyBool(bool);

  impl KeyComparable for MockKeyU8 {
    type Key = u8;

    fn key(&self) -> &Self::Key {
      &self.0
    }
  }

  impl KeyComparable for MockKeyBool {
    type Key = bool;

    fn key(&self) -> &Self::Key {
      &self.0
    }
  }

  #[test]
  fn test_new_set() {
    // VALID: non-empty set.
    let ordered_set: OrderedSet<MockKeyU8> = OrderedSet::from_iter([1, 2, 3].map(MockKeyU8));
    let new_set: OneOrSet<MockKeyU8> = OneOrSet::new_set(ordered_set.clone()).unwrap();
    let try_from_set: OneOrSet<MockKeyU8> = OneOrSet::try_from(ordered_set.clone()).unwrap();
    assert_eq!(new_set, try_from_set);
    assert_eq!(OrderedSet::from(new_set), ordered_set);

    // INVALID: empty set.
    let empty: OrderedSet<MockKeyU8> = OrderedSet::new();
    assert!(matches!(OneOrSet::new_set(empty.clone()), Err(Error::OneOrSetEmpty)));
    assert!(matches!(OneOrSet::try_from(empty), Err(Error::OneOrSetEmpty)));
  }

  #[test]
  fn test_append_from_one() {
    let mut collection: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(42));
    assert_eq!(collection.len(), 1);

    // Ignores duplicates.
    collection.append(MockKeyU8(42));
    assert_eq!(collection, OneOrSet::new_one(MockKeyU8(42)));
    assert_eq!(collection.len(), 1);

    // Becomes Set.
    collection.append(MockKeyU8(128));
    assert_eq!(
      collection,
      OneOrSet::new_set(OrderedSet::from_iter([42, 128].map(MockKeyU8).into_iter())).unwrap()
    );
    assert_eq!(collection.len(), 2);

    collection.append(MockKeyU8(200));
    assert_eq!(
      collection,
      OneOrSet::new_set(OrderedSet::from_iter([42, 128, 200].map(MockKeyU8).into_iter())).unwrap()
    );
    assert_eq!(collection.len(), 3);
  }

  #[test]
  fn test_append_from_set() {
    let mut collection: OneOrSet<MockKeyU8> = OneOrSet::new_set((0..42).map(MockKeyU8).collect()).unwrap();
    assert_eq!(collection.len(), 42);

    // Appends to end.
    collection.append(MockKeyU8(42));
    let expected: OneOrSet<MockKeyU8> = OneOrSet::new_set((0..=42).map(MockKeyU8).collect()).unwrap();
    assert_eq!(collection, expected);
    assert_eq!(collection.len(), 43);

    // Ignores duplicates.
    for i in 0..=42 {
      collection.append(MockKeyU8(i));
      assert_eq!(collection, expected);
      assert_eq!(collection.len(), 43);
    }
  }

  #[test]
  fn test_contains() {
    // One.
    let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    assert!(one.contains(&1u8));
    assert!(!one.contains(&2u8));
    assert!(!one.contains(&3u8));

    // Set.
    let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    assert!(set.contains(&1u8));
    assert!(set.contains(&2u8));
    assert!(set.contains(&3u8));
    assert!(!set.contains(&4u8));
  }

  #[test]
  fn test_get() {
    // One.
    let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    assert_eq!(one.get(0), Some(&MockKeyU8(1)));
    assert_eq!(one.get(1), None);
    assert_eq!(one.get(2), None);

    // Set.
    let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    assert_eq!(set.get(0), Some(&MockKeyU8(1)));
    assert_eq!(set.get(1), Some(&MockKeyU8(2)));
    assert_eq!(set.get(2), Some(&MockKeyU8(3)));
    assert_eq!(set.get(3), None);
  }

  #[test]
  fn test_map() {
    // One.
    let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    let one_add: OneOrSet<MockKeyU8> = one.map(|item| MockKeyU8(item.0 + 1));
    assert_eq!(one_add, OneOrSet::new_one(MockKeyU8(2)));

    // Set.
    let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    let set_add: OneOrSet<MockKeyU8> = set.map(|item| MockKeyU8(item.0 + 10));
    assert_eq!(set_add, OneOrSet::new_set((11..=13).map(MockKeyU8).collect()).unwrap());

    // Set reduced to one.
    let set_many: OneOrSet<MockKeyU8> = OneOrSet::new_set([2, 4, 6, 8].into_iter().map(MockKeyU8).collect()).unwrap();
    assert_eq!(set_many.len(), 4);
    let set_bool: OneOrSet<MockKeyBool> = set_many.map(|item| MockKeyBool(item.0 % 2 == 0));
    assert_eq!(set_bool, OneOrSet::new_one(MockKeyBool(true)));
    assert_eq!(set_bool.0, OneOrSetInner::One(MockKeyBool(true)));
    assert_eq!(set_bool.len(), 1);
  }

  #[test]
  fn test_try_map() {
    // One - OK
    let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    let one_add: OneOrSet<MockKeyU8> = one
      .try_map(|item| {
        if item.key() == &1 {
          Ok(MockKeyU8(item.0 + 1))
        } else {
          Err(Error::OneOrSetEmpty)
        }
      })
      .unwrap();
    assert_eq!(one_add, OneOrSet::new_one(MockKeyU8(2)));

    // One - ERROR
    let one_err: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    let result_one: Result<OneOrSet<MockKeyBool>> = one_err.try_map(|item| {
      if item.key() == &1 {
        Err(Error::OneOrSetEmpty)
      } else {
        Ok(MockKeyBool(false))
      }
    });
    assert!(matches!(result_one, Err(Error::OneOrSetEmpty)));

    // Set - OK
    let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    let set_add: OneOrSet<MockKeyU8> = set
      .try_map(|item| {
        if item.key() < &4 {
          Ok(MockKeyU8(item.0 + 10))
        } else {
          Err(Error::OneOrSetEmpty)
        }
      })
      .unwrap();
    assert_eq!(set_add, OneOrSet::new_set((11..=13).map(MockKeyU8).collect()).unwrap());

    // Set - ERROR
    let set_err: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    let result_set: Result<OneOrSet<MockKeyU8>> = set_err.try_map(|item| {
      if item.key() < &4 {
        Err(Error::OneOrSetEmpty)
      } else {
        Ok(MockKeyU8(item.0))
      }
    });
    assert!(matches!(result_set, Err(Error::OneOrSetEmpty)));

    // Set reduced to one - OK
    let set_many: OneOrSet<MockKeyU8> = OneOrSet::new_set([2, 4, 6, 8].into_iter().map(MockKeyU8).collect()).unwrap();
    assert_eq!(set_many.len(), 4);
    let set_bool: OneOrSet<MockKeyBool> = set_many
      .try_map(|item| {
        if item.key() % 2 == 0 {
          Ok(MockKeyBool(item.0 % 2 == 0))
        } else {
          Err(Error::OneOrSetEmpty)
        }
      })
      .unwrap();
    assert_eq!(set_bool, OneOrSet::new_one(MockKeyBool(true)));
    assert_eq!(set_bool.0, OneOrSetInner::One(MockKeyBool(true)));
    assert_eq!(set_bool.len(), 1);
  }

  #[test]
  fn test_iter() {
    // One.
    let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
    let mut one_iter = one.iter();
    assert_eq!(one_iter.next(), Some(&MockKeyU8(1)));
    assert_eq!(one_iter.next(), None);
    assert_eq!(one_iter.next(), None);

    // Set.
    let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
    let mut set_iter = set.iter();
    assert_eq!(set_iter.next(), Some(&MockKeyU8(1)));
    assert_eq!(set_iter.next(), Some(&MockKeyU8(2)));
    assert_eq!(set_iter.next(), Some(&MockKeyU8(3)));
    assert_eq!(set_iter.next(), None);
  }

  #[test]
  fn test_serde() {
    // VALID: one.
    {
      let one: OneOrSet<MockKeyU8> = OneOrSet::new_one(MockKeyU8(1));
      let ser: String = one.to_json().unwrap();
      let de: OneOrSet<MockKeyU8> = OneOrSet::from_json(&ser).unwrap();
      assert_eq!(ser, "1");
      assert_eq!(de, one);
    }

    // VALID: set.
    {
      let set: OneOrSet<MockKeyU8> = OneOrSet::new_set((1..=3).map(MockKeyU8).collect()).unwrap();
      let ser: String = set.to_json().unwrap();
      let de: OneOrSet<MockKeyU8> = OneOrSet::from_json(&ser).unwrap();
      assert_eq!(ser, "[1,2,3]");
      assert_eq!(de, set);
    }

    // INVALID: empty.
    {
      let empty: Result<OneOrSet<MockKeyU8>> = OneOrSet::from_json("");
      assert!(empty.is_err());
      let empty_set: Result<OneOrSet<MockKeyU8>> = OneOrSet::from_json("[]");
      assert!(empty_set.is_err());
      let empty_space: Result<OneOrSet<MockKeyU8>> = OneOrSet::from_json("[ ]");
      assert!(empty_space.is_err());
    }
  }
}
