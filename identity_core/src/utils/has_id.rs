use core::hash::Hash;

pub trait HasId {
    type Id: Hash + PartialEq + Eq + PartialOrd + Ord;

    fn id(&self) -> &Self::Id;
}
