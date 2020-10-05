use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::Deref,
};
use identity_diff::{hashset::DiffHashSet, Diff, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// Has ID
// =============================================================================

pub trait HasId {
    type Id: Hash + PartialEq + Eq + PartialOrd + Ord;

    fn id(&self) -> &Self::Id;
}

// =============================================================================
// ID Set
// =============================================================================

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdSet<T>(HashSet<T>)
where
    T: Eq + Hash;

impl<T> IdSet<T>
where
    T: Eq + Hash,
{
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, item: T) -> bool {
        self.0.insert(item)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T> Deref for IdSet<T>
where
    T: Eq + Hash,
{
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Diff for IdSet<T>
where
    T: Hash + Eq + Ord + Diff + for<'de> Deserialize<'de> + Serialize,
{
    type Type = DiffHashSet<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type, Error> {
        self.0.diff(&other.0)
    }

    fn merge(&self, diff: Self::Type) -> Result<Self, Error> {
        self.0.merge(diff).map(Self)
    }

    fn from_diff(diff: Self::Type) -> Result<Self, Error> {
        HashSet::from_diff(diff).map(Self)
    }

    fn into_diff(self) -> Result<Self::Type, Error> {
        self.0.into_diff()
    }
}

// =============================================================================
// ID Compare
// =============================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, Diff)]
pub struct IdCompare<T>(pub T)
where
    T: HasId;

impl<T> IdCompare<T>
where
    T: HasId,
{
    pub fn new(item: T) -> Self {
        Self(item)
    }
}

impl<T> From<T> for IdCompare<T>
where
    T: HasId,
{
    fn from(other: T) -> Self {
        Self::new(other)
    }
}

impl<T> HasId for IdCompare<T>
where
    T: HasId,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        self.0.id()
    }
}

impl<T> Hash for IdCompare<T>
where
    T: HasId,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.id().hash(hasher)
    }
}

impl<T> PartialEq for IdCompare<T>
where
    T: HasId,
{
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(other.id())
    }
}

impl<T> PartialEq<T> for IdCompare<T>
where
    T: HasId,
{
    fn eq(&self, other: &T) -> bool {
        self.id().eq(other.id())
    }
}

impl<T> Eq for IdCompare<T> where T: HasId {}

impl<T> PartialOrd for IdCompare<T>
where
    T: HasId,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id().partial_cmp(other.id())
    }
}

impl<T> Ord for IdCompare<T>
where
    T: HasId,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(other.id())
    }
}
