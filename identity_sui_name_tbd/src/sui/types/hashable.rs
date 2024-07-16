use std::hash::Hash;

use serde::Deserialize;
use serde::Serialize;
use iota_sdk::types::id::ID;
use iota_sdk::types::id::UID;

pub trait IntoHash {
  fn as_hash(&self) -> impl Hash;
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[repr(transparent)]
#[serde(transparent)]
pub(crate) struct Hashable<T>(pub T);

impl<T: IntoHash> Hash for Hashable<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.as_hash().hash(state);
  }
}

impl IntoHash for ID {
  fn as_hash(&self) -> impl Hash {
    &self.bytes
  }
}

impl IntoHash for UID {
  fn as_hash(&self) -> impl Hash {
    self.id.as_hash()
  }
}
