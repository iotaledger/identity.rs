// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use futures::stream::FuturesUnordered;
use futures::stream::TryStreamExt;
use identity_core::utils::encode_b58;
use iota_stronghold::Location;
use iota_stronghold::StrongholdFlags;
use std::path::Path;

use crate::stronghold::IotaStrongholdResult;
use crate::stronghold::Store;
use crate::stronghold::StrongholdError;

pub struct Records<'snapshot> {
  pub(crate) store: Store<'snapshot>,
}

impl<'snapshot> Records<'snapshot> {
  pub(crate) fn new<P, T>(path: &'snapshot P, name: &T, flags: &[StrongholdFlags]) -> Self
  where
    P: AsRef<Path> + ?Sized,
    T: AsRef<[u8]> + ?Sized,
  {
    Self {
      store: Store::new(path, name, flags),
    }
  }
}

impl Records<'_> {
  pub fn name(&self) -> &[u8] {
    self.store.name()
  }

  pub fn path(&self) -> &Path {
    self.store.path()
  }

  pub fn flags(&self) -> &[StrongholdFlags] {
    self.store.flags()
  }

  pub async fn index(&self) -> IotaStrongholdResult<RecordIndex> {
    self.store.get(Locations::index()).await?;
    let record: Option<Vec<u8>> = self.store.get(Locations::index()).await?;
    match record {
      None => Err(StrongholdError::RecordError),
      Some(record) => Ok(RecordIndex::try_new(record)?),
    }
  }

  pub async fn all(&self) -> IotaStrongholdResult<Vec<Option<Vec<u8>>>> {
    self.index().await?.load_all(&self.store).await
  }

  pub async fn get(&self, record_id: &[u8]) -> IotaStrongholdResult<Option<Vec<u8>>> {
    let record_tag: RecordTag = RecordIndex::tag(record_id);
    let location: Location = Locations::record(&record_tag);

    self.store.get(location).await
  }

  pub async fn set(&self, record_id: &[u8], record: &[u8]) -> IotaStrongholdResult<()> {
    self.try_set(record_id, record, true).await.map(|_| ())
  }

  pub async fn try_set(&self, record_id: &[u8], record: &[u8], replace: bool) -> IotaStrongholdResult<bool> {
    let mut index: RecordIndex = self.index().await?;
    let record_tag: RecordTag = RecordIndex::tag(record_id);
    let inserted: bool = index.insert(&record_tag);

    // Add the id to the record index
    if inserted {
      self.store.set(Locations::index(), index.into_bytes(), None).await?;
    }

    if inserted || replace {
      // Add the record to a namespaced store in the snapshot
      self
        .store
        .set(Locations::record(&record_tag), record.to_vec(), None)
        .await?;

      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub async fn del(&self, record_id: &[u8]) -> IotaStrongholdResult<()> {
    let mut index: RecordIndex = self.index().await?;
    let record_tag: RecordTag = RecordIndex::tag(record_id);

    // Remove the id from the record index
    if index.remove(&record_tag) {
      self.store.set(Locations::index(), index.into_bytes(), None).await?;
    }

    // Remove the record from the snapshot store
    self.store.del(Locations::record(&record_tag)).await?;

    Ok(())
  }
}

// =============================================================================
// =============================================================================

struct Locations;

impl Locations {
  fn index() -> Location {
    Location::generic("__index", "")
  }

  fn record(record_tag: &[u8]) -> Location {
    Location::generic(format!("__record:{}", encode_b58(record_tag)), "")
  }
}

// =============================================================================
// =============================================================================

pub type RecordTag = Output<Blake2b256>;

pub struct RecordIndex(Vec<u8>);

impl RecordIndex {
  const CHUNK: usize = 32;

  pub(crate) fn try_new(data: Vec<u8>) -> IotaStrongholdResult<Self> {
    if data.len() % Self::CHUNK != 0 {
      return Err(StrongholdError::InvalidResourceIndex);
    }

    Ok(Self(data))
  }

  pub fn iter(&self) -> impl Iterator<Item = &[u8]> {
    self.0.chunks_exact(Self::CHUNK)
  }

  pub fn contains(&self, tag: &[u8]) -> bool {
    self.iter().any(|chunk| chunk == tag)
  }

  pub(crate) async fn load_all(&self, store: &Store<'_>) -> IotaStrongholdResult<Vec<Option<Vec<u8>>>> {
    self
      .iter()
      .map(Locations::record)
      .map(|tag| store.get(tag))
      .collect::<FuturesUnordered<_>>()
      .try_collect()
      .await
  }

  pub(crate) fn into_bytes(self) -> Vec<u8> {
    self.0
  }

  pub(crate) fn insert(&mut self, tag: &[u8]) -> bool {
    if self.contains(tag) {
      return false;
    }

    self.0.extend_from_slice(tag);
    true
  }

  pub(crate) fn remove(&mut self, tag: &[u8]) -> bool {
    let index: Option<usize> = self.iter().position(|chunk| chunk == tag);

    if let Some(index) = index {
      self.0.drain(Self::CHUNK * index..Self::CHUNK * (index + 1));
      return true;
    }

    false
  }

  pub(crate) fn tag(id: &[u8]) -> RecordTag {
    Blake2b256::digest(id)
  }
}

impl Debug for RecordIndex {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_set().entries(self.iter().map(encode_b58)).finish()
  }
}

#[cfg(test)]
mod tests {
  use crate::stronghold::RecordIndex;
  use crate::stronghold::RecordTag;

  #[test]
  fn test_record_index() {
    let mut index: RecordIndex = RecordIndex::try_new(Vec::new()).unwrap();

    let tag_a: RecordTag = RecordIndex::tag(b"A");
    let tag_b: RecordTag = RecordIndex::tag(b"B");
    let tag_c: RecordTag = RecordIndex::tag(b"C");

    assert!(index.insert(&tag_a));
    assert!(index.insert(&tag_b));
    assert!(index.insert(&tag_c));

    assert!(index.contains(&tag_a));
    assert!(index.contains(&tag_b));
    assert!(index.contains(&tag_c));

    assert_eq!(index.iter().count(), 3);

    assert!(!index.insert(&tag_a));
    assert!(!index.insert(&tag_b));
    assert!(!index.insert(&tag_c));

    assert!(index.remove(&tag_b));
    assert!(index.contains(&tag_a));
    assert!(index.contains(&tag_c));

    assert_eq!(index.iter().count(), 2);

    assert!(!index.remove(&tag_b));
    assert!(index.remove(&tag_c));
    assert!(index.contains(&tag_a));
    assert!(!index.contains(&tag_c));

    assert_eq!(index.iter().count(), 1);
    assert_eq!(index.iter().next().unwrap(), tag_a.as_slice());
  }
}
