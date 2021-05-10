// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::FromIterator;
use core::ops::Deref;
use core::slice::Iter;
use serde::Deserialize;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::utils::DIDKey;
use crate::verification::MethodQuery;

/// An ordered set backed by a `Vec<T>`.
///
/// Note: Ordering is based on insert order and **not** [`Ord`].
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(bound(deserialize = "T: PartialEq + Deserialize<'de>"), try_from = "Vec<T>")]
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

  /// Returns a mutable referece the last element in the set, or `None` if the
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
    T: AsRef<U>,
    U: PartialEq + ?Sized,
  {
    self.0.iter().any(|other| other.as_ref() == item)
  }

  /// Adds a new value to the end of the `OrderedSet`; returns `true` if the
  /// value was successfully added.
  pub fn append(&mut self, item: T) -> bool
  where
    T: PartialEq,
  {
    if self.0.contains(&item) {
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
    T: PartialEq,
  {
    if self.0.contains(&item) {
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
    T: PartialEq + Borrow<U>,
    U: PartialEq + ?Sized,
  {
    self.change(update, |item, update| item.borrow() == current || item == update)
  }

  /// Updates an existing value in the `OrderedSet`; returns `true` if the value
  /// was successfully updated.
  #[inline]
  pub fn update(&mut self, update: T) -> bool
  where
    T: PartialEq,
  {
    self.change(update, |item, update| item == update)
  }

  /// Removes all matching items from the set.
  #[inline]
  pub fn remove<U>(&mut self, item: &U)
  where
    T: PartialEq + Borrow<U>,
    U: PartialEq + ?Sized,
  {
    self.0.retain(|this| this.borrow() != item);
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
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

impl<T> Default for OrderedSet<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T> FromIterator<T> for OrderedSet<T>
where
  T: PartialEq,
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
  T: PartialEq,
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

impl<T> OrderedSet<DIDKey<T>>
where
  T: AsRef<DID>,
{
  pub(crate) fn query<'query, Q>(&self, query: Q) -> Option<&T>
  where
    Q: Into<MethodQuery<'query>>,
  {
    let query: MethodQuery<'query> = query.into();

    self
      .0
      .iter()
      .find(|method| query.matches(method.as_did()))
      .map(|method| &**method)
  }

  pub(crate) fn query_mut<'query, Q>(&mut self, query: Q) -> Option<&mut T>
  where
    Q: Into<MethodQuery<'query>>,
  {
    let query: MethodQuery<'query> = query.into();

    self
      .0
      .iter_mut()
      .find(|method| query.matches(method.as_did()))
      .map(|method| &mut **method)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::did::DID;
  use crate::utils::DIDKey;
  use crate::verification::MethodRef;

  #[test]
  fn test_works() {
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
    let did1: DID = DID::parse("did:example:123").unwrap();
    let did2: DID = DID::parse("did:example:456").unwrap();

    let source: Vec<DIDKey<MethodRef>> = vec![
      DIDKey::new(MethodRef::Refer(did1.clone())),
      DIDKey::new(MethodRef::Refer(did2.clone())),
    ];

    let oset: OrderedSet<DIDKey<MethodRef>> = source.into_iter().collect();

    assert!(oset.contains(&MethodRef::Refer(did1)));
    assert!(oset.contains(&MethodRef::Refer(did2)));
  }
}
