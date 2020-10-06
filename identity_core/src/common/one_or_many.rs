use core::{fmt, hash::Hash, slice::from_ref};
use identity_diff::{Diff, Error};
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// A generic container that stores one or many values of a given type.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
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

    /// Returns `true` if the given value is represented in the collection.
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq<T>,
    {
        match self {
            Self::One(inner) => inner == value,
            Self::Many(inner) => inner.contains(value),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        OneOrManyIter::new(self)
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::One(inner) => from_ref(inner),
            Self::Many(inner) => inner.as_slice(),
        }
    }

    /// Consumes the `OneOrMany`, returning the contents as a `Vec`.
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::One(inner) => vec![inner],
            Self::Many(inner) => inner,
        }
    }
}

impl<T> fmt::Debug for OneOrMany<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::One(inner) => fmt::Debug::fmt(inner, f),
            Self::Many(inner) => fmt::Debug::fmt(inner, f),
        }
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

// =============================================================================
// Iterator
// =============================================================================

struct OneOrManyIter<'a, T> {
    inner: &'a OneOrMany<T>,
    index: usize,
}

impl<'a, T> OneOrManyIter<'a, T> {
    pub fn new(inner: &'a OneOrMany<T>) -> Self {
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
// Diff Support
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = "DiffOneOrMany<T>: Clone + fmt::Debug + PartialEq"))]
// #[serde(from = "OneOrMany<T>", into = "OneOrMany<T>")]
#[serde(untagged)]
pub enum DiffOneOrMany<T>
where
    T: Diff + for<'a> Deserialize<'a> + Serialize,
{
    One(#[serde(skip_serializing_if = "Option::is_none")] Option<<T as Diff>::Type>),
    Many(#[serde(skip_serializing_if = "Option::is_none")] Option<<Vec<T> as Diff>::Type>),
}

impl<T> Diff for OneOrMany<T>
where
    T: Clone + fmt::Debug + PartialEq + Diff + for<'a> Deserialize<'a> + Serialize,
{
    type Type = DiffOneOrMany<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type, Error> {
        match (self, other) {
            (Self::One(lhs), Self::One(rhs)) if lhs == rhs => Ok(DiffOneOrMany::One(None)),
            (Self::One(lhs), Self::One(rhs)) => lhs.diff(rhs).map(Some).map(DiffOneOrMany::One),
            (Self::Many(lhs), Self::Many(rhs)) if lhs == rhs => Ok(DiffOneOrMany::Many(None)),
            (Self::Many(lhs), Self::Many(rhs)) => lhs.diff(rhs).map(Some).map(DiffOneOrMany::Many),
            (_, diff) => diff.clone().into_diff(),
        }
    }

    fn merge(&self, diff: Self::Type) -> Result<Self, Error> {
        match (self, diff) {
            (Self::One(lhs), Self::Type::One(Some(ref rhs))) => Ok(Self::One(lhs.merge(rhs.clone())?)),
            (lhs @ Self::One(_), Self::Type::One(None)) => Ok(lhs.clone()),
            (Self::Many(lhs), Self::Type::Many(Some(ref rhs))) => Ok(Self::Many(lhs.merge(rhs.clone())?)),
            (lhs @ Self::Many(_), Self::Type::Many(None)) => Ok(lhs.clone()),
            (_, diff) => Self::from_diff(diff.clone()),
        }
    }

    fn from_diff(diff: Self::Type) -> Result<Self, Error> {
        match diff {
            DiffOneOrMany::One(Some(inner)) => T::from_diff(inner).map(Self::One),
            DiffOneOrMany::One(None) => Ok(Default::default()),
            DiffOneOrMany::Many(Some(inner)) => <Vec<T>>::from_diff(inner).map(Self::Many),
            DiffOneOrMany::Many(None) => Ok(Default::default()),
        }
    }

    fn into_diff(self) -> Result<Self::Type, Error> {
        match self {
            Self::One(inner) => inner.into_diff().map(Some).map(DiffOneOrMany::One),
            Self::Many(inner) => inner.into_diff().map(Some).map(DiffOneOrMany::Many),
        }
    }
}
