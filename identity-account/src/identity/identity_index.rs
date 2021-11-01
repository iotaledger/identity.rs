// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use tokio::sync::RwLock;

use identity_did::did::DID;
use identity_iota::did::IotaDID;

use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentityId;
use crate::identity::IdentityKey;
use crate::identity::IdentityTag;

pub(crate) type IdentityLock = Arc<RwLock<IdentityId>>;

/// An mapping between [IdentityTag]s and [IdentityId]s.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdentityIndex {
  data: HashMap<IdentityTag, IdentityId>,
  #[serde(skip)]
  locks: HashMap<IdentityId, IdentityLock>,
}

impl IdentityIndex {
  /// Creates a new `IdentityIndex`.
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
      locks: HashMap::new(),
    }
  }

  /// Returns the next IdentityId in the sequence.
  ///
  /// # Errors
  ///
  /// Fails if the current id is the maximum supported value.
  pub fn try_next_id(&self) -> Result<IdentityId> {
    self.data.values().max().copied().unwrap_or_default().try_next()
  }

  /// Returns a list of all tags in the index.
  pub fn tags(&self) -> Vec<IdentityTag> {
    self.data.keys().cloned().collect()
  }

  /// Returns the id of the identity matching the given `key`.
  pub fn get<K: IdentityKey>(&self, key: &K) -> Option<IdentityId> {
    key.scan(self.data.iter())
  }

  /// Returns the id of the identity matching the given `key` wrapped in a lock.
  ///
  /// Should be used to synchronize write access to the given `key`.
  pub fn get_lock<K: IdentityKey>(&mut self, key: K) -> Option<IdentityLock> {
    if let Some(identity_id) = key.scan(self.data.iter()) {
      match self.locks.entry(identity_id) {
        Entry::Occupied(lock) => Some(Arc::clone(lock.get())),
        Entry::Vacant(entry) => {
          let lock = entry.insert(Arc::new(RwLock::new(identity_id)));
          Some(Arc::clone(lock))
        }
      }
    } else {
      None
    }
  }

  /// Adds a new unnamed identity to the index.
  pub fn set(&mut self, id: IdentityId, did: &IotaDID) -> Result<()> {
    self.insert(id, IdentityTag::new(did.method_id().into()))
  }

  /// Adds a new named identity to the index.
  pub fn set_named(&mut self, id: IdentityId, did: &IotaDID, name: String) -> Result<()> {
    self.insert(id, IdentityTag::named(did.method_id().into(), name))
  }

  /// Removes the identity specified by `key` from the index.
  pub fn del<K: IdentityKey>(&mut self, key: K) -> Result<(IdentityTag, IdentityId)> {
    let removed_id = self
      .data
      .drain_filter(|tag, id| key.equals(tag, *id))
      .next()
      .ok_or(Error::IdentityNotFound)?;

    self.locks.remove(&removed_id.1);

    Ok(removed_id)
  }

  fn insert(&mut self, id: IdentityId, tag: IdentityTag) -> Result<()> {
    match self.data.entry(tag) {
      Entry::Occupied(_) => Err(Error::IdentityAlreadyExists),
      Entry::Vacant(entry) => {
        entry.insert(id);
        Ok(())
      }
    }
  }
}

impl Default for IdentityIndex {
  fn default() -> Self {
    Self::new()
  }
}

impl PartialEq for IdentityIndex {
  fn eq(&self, other: &Self) -> bool {
    self.data == other.data
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basics() {
    let mut index: IdentityIndex = IdentityIndex::new();
    assert!(index.tags().is_empty());

    let target1: IotaDID = format!("did:iota:{}", IotaDID::encode_key(b"123")).parse().unwrap();
    let target2: IotaDID = format!("did:iota:{}", IotaDID::encode_key(b"456")).parse().unwrap();
    let target3: IotaDID = format!("did:iota:{}", IotaDID::encode_key(b"789")).parse().unwrap();

    index.set(1.into(), &target1).unwrap();
    index.set(2.into(), &target2).unwrap();
    index.set(3.into(), &target3).unwrap();

    assert_eq!(index.tags().len(), 3);

    assert_eq!(index.get(&target1).unwrap().to_u32(), 1);
    assert_eq!(index.get(&target2).unwrap().to_u32(), 2);
    assert_eq!(index.get(&target3).unwrap().to_u32(), 3);

    assert_eq!(index.del(&target1).unwrap().1.to_u32(), 1);
    assert_eq!(index.del(&target2).unwrap().1.to_u32(), 2);

    assert_eq!(index.tags().len(), 1);
  }

  #[test]
  fn test_next_id() {
    let mut index: IdentityIndex = IdentityIndex::new();
    assert_eq!(index.try_next_id().unwrap().to_u32(), 1);

    index.insert(1.into(), IdentityTag::new("foo-1".into())).unwrap();
    assert_eq!(index.try_next_id().unwrap().to_u32(), 2);

    index.insert(2.into(), IdentityTag::new("foo-2".into())).unwrap();
    assert_eq!(index.try_next_id().unwrap().to_u32(), 3);

    let target: IdentityId = IdentityId::from(1);
    let (tag, id): (IdentityTag, IdentityId) = index.del(target).unwrap();
    assert_eq!(tag.name(), None);
    assert_eq!(tag.method_id(), "foo-1");
    assert_eq!(id, target);

    assert_eq!(index.try_next_id().unwrap().to_u32(), 3);
  }
}
