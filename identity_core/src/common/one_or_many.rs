// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::hash::Hash;
use core::mem::replace;
use core::ops::Deref;
use std::vec::IntoIter;

use serde;
use serde::Deserialize;
use serde::Serialize;

/// A generic container that stores exactly one or many (0+) values of a given type.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
  /// A single instance of `T`.
  One(T),
  /// Multiple (zero or more) instances of `T`.
  Many(Vec<T>),
}

impl<T> OneOrMany<T> {
  /// Returns the number of elements in the collection
  pub fn len(&self) -> usize {
    match self {
      Self::One(_) => 1,
      Self::Many(inner) => inner.len(),
    }
  }

  /// Returns `true` if the collection is empty
  pub fn is_empty(&self) -> bool {
    match self {
      Self::One(_) => false,
      Self::Many(inner) => inner.is_empty(),
    }
  }

  /// Returns a reference to the element at the given index.
  pub fn get(&self, index: usize) -> Option<&T> {
    match self {
      Self::One(inner) if index == 0 => Some(inner),
      Self::One(_) => None,
      Self::Many(inner) => inner.get(index),
    }
  }

  /// Returns a mutable reference to the element at the given index.
  pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    match self {
      Self::One(ref mut inner) if index == 0 => Some(inner),
      Self::One(_) => None,
      Self::Many(inner) => inner.get_mut(index),
    }
  }

  /// Returns `true` if the collection contains the given value.
  pub fn contains(&self, value: &T) -> bool
  where
    T: PartialEq<T>,
  {
    match self {
      Self::One(inner) => inner == value,
      Self::Many(inner) => inner.contains(value),
    }
  }

  /// Adds a new value to the collection.
  pub fn push(&mut self, value: T) {
    match self {
      Self::One(_) => match replace(self, Self::Many(Vec::new())) {
        Self::One(inner) => *self = Self::Many(vec![inner, value]),
        Self::Many(_) => unreachable!(),
      },
      Self::Many(ref mut inner) => {
        if inner.is_empty() {
          *self = Self::One(value);
        } else {
          inner.push(value);
        }
      }
    }
  }

  /// Returns an `Iterator` that yields items from the collection.
  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    OneOrManyIter::new(self)
  }

  /// Returns a reference to the contents as a slice.
  pub fn as_slice(&self) -> &[T] {
    self
  }

  /// Consumes the [`OneOrMany`] and returns the contents as a [`Vec`].
  pub fn into_vec(self) -> Vec<T> {
    match self {
      Self::One(inner) => vec![inner],
      Self::Many(inner) => inner,
    }
  }
}

impl<T> Debug for OneOrMany<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::One(inner) => Debug::fmt(inner, f),
      Self::Many(inner) => Debug::fmt(inner, f),
    }
  }
}

impl<T> Deref for OneOrMany<T> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    match self {
      Self::One(inner) => core::slice::from_ref(inner),
      Self::Many(inner) => inner,
    }
  }
}

impl<T> AsRef<[T]> for OneOrMany<T> {
  fn as_ref(&self) -> &[T] {
    self
  }
}

impl<T> Default for OneOrMany<T> {
  fn default() -> Self {
    Self::Many(Vec::new())
  }
}

impl<T> From<T> for OneOrMany<T> {
  fn from(other: T) -> Self {
    Self::One(other)
  }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
  fn from(mut other: Vec<T>) -> Self {
    if other.len() == 1 {
      Self::One(other.pop().expect("infallible"))
    } else {
      Self::Many(other)
    }
  }
}

impl<T> From<OneOrMany<T>> for Vec<T> {
  fn from(other: OneOrMany<T>) -> Self {
    other.into_vec()
  }
}

impl<T> FromIterator<T> for OneOrMany<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut iter = iter.into_iter();
    // if the iterator contains one element or less and a correct size hint is provided we can save an allocation
    let size_hint = iter.size_hint();
    if size_hint.1.is_some() && (1, Some(1)) >= size_hint {
      let mut this = iter.next().map(Self::One).unwrap_or_else(|| Self::Many(Vec::new()));
      // if the hinted upper bound was incorrect we need to correct for it
      for next in iter.by_ref() {
        this.push(next);
      }
      this
    } else {
      iter.collect::<Vec<T>>().into()
    }
  }
}

// =============================================================================
// Iterator
// =============================================================================

/// This struct is created by the `iter` method on [`OneOrMany`].
struct OneOrManyIter<'a, T> {
  inner: &'a OneOrMany<T>,
  index: usize,
}

impl<'a, T> OneOrManyIter<'a, T> {
  fn new(inner: &'a OneOrMany<T>) -> Self {
    Self { inner, index: 0 }
  }
}

impl<'a, T> Iterator for OneOrManyIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    self.inner.get(self.index - 1)
  }
}

// =============================================================================
// IntoIterator
// =============================================================================

enum Either<L, R> {
  Left(L),
  Right(R),
}

/// This struct is created by the `into_iter` method on [`OneOrMany`]
/// (provided by the [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) trait).
pub struct OneOrManyIntoIterator<T> {
  iter: Either<Option<T>, IntoIter<T>>,
}

impl<T> OneOrManyIntoIterator<T> {
  fn new(inner: OneOrMany<T>) -> Self {
    let iter = match inner {
      OneOrMany::One(item) => Either::Left(Some(item)),
      OneOrMany::Many(vec) => Either::Right(vec.into_iter()),
    };

    Self { iter }
  }
}

impl<T> Iterator for OneOrManyIntoIterator<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.iter {
      Either::Left(ref mut item_opt) => item_opt.take(),
      Either::Right(ref mut iter) => iter.next(),
    }
  }
}

impl<T> IntoIterator for OneOrMany<T> {
  type Item = T;

  type IntoIter = OneOrManyIntoIterator<T>;

  fn into_iter(self) -> Self::IntoIter {
    OneOrManyIntoIterator::new(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_iterator_empty() {
    let empty_vec = Vec::<u32>::new();
    assert_eq!(OneOrMany::from_iter(empty_vec.clone()), OneOrMany::Many(empty_vec));
  }

  #[test]
  fn from_iterator_single() {
    let single_item = [1];
    assert_eq!(OneOrMany::from_iter(single_item), OneOrMany::One(1));
  }

  #[test]
  fn from_iterator_many() {
    let letters = ["a", "b", "c", "d"];
    assert_eq!(OneOrMany::from_iter(letters), OneOrMany::Many(vec!["a", "b", "c", "d"]));
  }

  #[test]
  fn from_iterator_iter() {
    let none = OneOrMany::Many(Vec::<u32>::new());
    assert_eq!(OneOrMany::from_iter(none.iter()), OneOrMany::Many(Vec::<&u32>::new()));

    let one = OneOrMany::One(42);
    assert_eq!(OneOrMany::from_iter(one.iter()), OneOrMany::One(&42));

    let two = OneOrMany::Many(vec![0, 1]);
    assert_eq!(OneOrMany::from_iter(two.iter()), OneOrMany::Many(vec![&0, &1]));
  }

  #[test]
  fn push_from_zero_elements() {
    let mut collection = OneOrMany::Many(Vec::<u32>::new());
    collection.push(42);
    assert_eq!(collection, OneOrMany::One(42));
  }

  #[test]
  fn push_one_element() {
    let mut collection = OneOrMany::One(42);
    collection.push(42);
    assert_eq!(collection, OneOrMany::Many(vec![42, 42]));
  }

  #[test]
  fn push_many_elements() {
    let v: Vec<i32> = (0..42).collect();
    let mut collection = OneOrMany::Many(v);
    collection.push(42);
    assert_eq!(collection, OneOrMany::Many((0..=42).collect()));
  }

  #[test]
  fn test_iter() {
    let one_or_many = OneOrMany::Many(Vec::<u32>::new());
    assert!(one_or_many.iter().next().is_none());

    let one_or_many = OneOrMany::One(1u32);
    assert_eq!(one_or_many.iter().next().unwrap(), &1);

    let one_or_many = OneOrMany::Many(vec![42u32]);
    let mut iter = one_or_many.iter();
    assert_eq!(iter.next().unwrap(), &42);
    assert!(iter.next().is_none());

    let one_or_many = OneOrMany::Many(vec![42u32, 1337u32]);
    let mut iter = one_or_many.iter();
    assert_eq!(iter.next().unwrap(), &42);
    assert_eq!(iter.next().unwrap(), &1337);
    assert!(iter.next().is_none());
  }

  #[test]
  fn test_into_iter() {
    let one_or_many = OneOrMany::Many(Vec::<u32>::new());
    assert!(one_or_many.into_iter().next().is_none());

    let one_or_many = OneOrMany::One(1u32);
    assert_eq!(one_or_many.into_iter().next().unwrap(), 1);

    let one_or_many = OneOrMany::Many(vec![42u32]);
    let mut into_iter = one_or_many.into_iter();
    assert_eq!(into_iter.next().unwrap(), 42);
    assert!(into_iter.next().is_none());

    let one_or_many = OneOrMany::Many(vec![42u32, 1337u32]);
    let mut into_iter = one_or_many.into_iter();
    assert_eq!(into_iter.next().unwrap(), 42);
    assert_eq!(into_iter.next().unwrap(), 1337);
    assert!(into_iter.next().is_none());
  }
}
