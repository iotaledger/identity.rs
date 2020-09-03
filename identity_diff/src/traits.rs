use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Diff: Clone + Debug + PartialEq {
    type Type: Sized + Clone + Debug + PartialEq + for<'de> Deserialize<'de> + Serialize + Default;

    fn diff(&self, other: &Self) -> Self::Type;

    fn merge(&self, diff: Self::Type) -> Self;

    fn from_diff(diff: Self::Type) -> Self;

    fn into_diff(self) -> Self::Type;
}
