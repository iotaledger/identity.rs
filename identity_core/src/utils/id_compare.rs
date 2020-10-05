use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use core::hash::Hasher;
use core::hash::Hash;
use core::cmp::Ordering;

pub trait HasId {
    type Id: Hash + PartialEq + Eq + PartialOrd + Ord;

    fn id(&self) -> &Self::Id;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Diff)]
pub struct IdCompare<T>(pub T) where T: HasId;

impl<T> IdCompare<T> where T: HasId {
    pub fn new(item: T) -> Self {
        Self(item)
    }
}

impl<T> From<T> for IdCompare<T> where T: HasId {
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

impl<T> Hash for IdCompare<T> where T: HasId {
  fn hash<H>(&self, hasher: &mut H) where H: Hasher {
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

impl<T> Eq for IdCompare<T> where T: HasId {}

impl<T> PartialOrd for IdCompare<T>
where
  T: HasId,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.id().partial_cmp(other.id())
  }
}

impl<T> Ord for IdCompare<T> where T: HasId {
  fn cmp(&self, other: &Self) -> Ordering {
    self.id().cmp(other.id())
  }
}
