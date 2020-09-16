mod authentication;
mod context;
pub mod helpers;
mod keys;
mod service;
mod service_serialize;
mod subject;

use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub use authentication::Authentication;
pub use context::Context;
pub use keys::{KeyData, PublicKey, PublicKeyTypes};
pub use service::{Service, ServiceEndpoint};
pub use subject::Subject;

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
