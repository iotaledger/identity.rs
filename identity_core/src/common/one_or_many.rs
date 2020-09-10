use serde::{Deserialize, Serialize};
use std::fmt;

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
