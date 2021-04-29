// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use identity_iota::did::DID;

use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentityId;
use crate::identity::IdentityKey;
use crate::identity::IdentityTag;

/// An mapping between [IdentityTag]s and [IdentityId]s.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IdentityIndex {
  data: HashMap<IdentityTag, IdentityId>,
}

impl IdentityIndex {
  /// Creates a new `IdentityIndex`.
  pub fn new() -> Self {
    Self { data: HashMap::new() }
  }

  /// Returns the next IdentityId in the sequence.
  ///
  /// # Errors
  ///
  /// Fails if the current id is the maximum supported value.
  pub fn try_next_id(&self) -> Result<IdentityId> {
    self.data.values().max().copied().unwrap_or_default().try_next()
  }

  /// Returns the id of the identity matching the given `key`.
  pub fn get<K: IdentityKey>(&self, key: K) -> Option<IdentityId> {
    key.find_iter(self.data.iter())
  }

  /// Adds a new unnamed identity to the index.
  pub fn set(&mut self, id: IdentityId, did: &DID) -> Result<()> {
    self.insert(id, IdentityTag::new(did.method_id().into()))
  }

  /// Adds a new named identity to the index.
  pub fn set_named(&mut self, id: IdentityId, did: &DID, name: String) -> Result<()> {
    self.insert(id, IdentityTag::named(did.method_id().into(), name))
  }

  /// Returns a list of all tags in the index.
  pub fn tags(&self) -> Vec<IdentityTag> {
    self.data.keys().cloned().collect()
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
