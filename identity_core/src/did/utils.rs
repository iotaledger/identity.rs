use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}

pub trait HasId {
    type Id: Hash + PartialEq + Eq;

    fn id(&self) -> &Self::Id;
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, PartialOrd, Ord, Diff, Default)]
pub struct IdCompare<T: HasId>(pub T);

impl<T> IdCompare<T>
where
    T: HasId,
{
    pub fn new(item: T) -> Self {
        Self(item)
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

pub fn add_unique_to_vec<T: HasId>(item: T, collection: Vec<T>) -> Vec<T> {
    let mut collection: Vec<T> = collection
        .into_iter()
        .filter(|it| it.id() != item.id())
        .collect::<Vec<T>>();

    collection.push(item);

    collection
}
