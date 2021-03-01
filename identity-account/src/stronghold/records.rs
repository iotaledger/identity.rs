// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use futures::stream::FuturesUnordered;
use futures::stream::TryStreamExt;
use identity_core::utils::encode_b58;
use iota_stronghold::Location;
use iota_stronghold::StrongholdFlags;
use std::path::Path;

use crate::error::Error;
use crate::error::Result;
use crate::stronghold::Store;

pub struct Records<'path> {
  pub(crate) store: Store<'path>,
}

impl<'path> Records<'path> {
  pub(crate) fn new<T>(path: &'path Path, name: &T, flags: &[StrongholdFlags]) -> Self
  where
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

  pub async fn flush(&self) -> Result<()> {
    self.store.flush().await
  }

  pub async fn index(&self) -> Result<RecordIndex> {
    self.store.get(Locations::index()).await.and_then(RecordIndex::try_new)
  }

  pub async fn all(&self) -> Result<Vec<Vec<u8>>> {
    self
      .index()
      .await?
      .iter()
      .map(Locations::record)
      .map(|tag| self.store.get(tag))
      .collect::<FuturesUnordered<_>>()
      .try_collect()
      .await
  }

  pub async fn get(&self, record_id: &[u8]) -> Result<Vec<u8>> {
    let record_tag: RecordTag = RecordIndex::tag(record_id);
    let location: Location = Locations::record(&record_tag);

    self.store.get(location).await
  }

  pub async fn set(&self, record_id: &[u8], record: &[u8]) -> Result<()> {
    let mut index: RecordIndex = self.index().await?;
    let record_tag: RecordTag = RecordIndex::tag(record_id);

    // Add the id to the record index
    if index.insert(&record_tag) {
      self.store.set(Locations::index(), index.into_bytes(), None).await?;
    }

    // Add the record to a namespaced store in the snapshot
    self
      .store
      .set(Locations::record(&record_tag), record.to_vec(), None)
      .await?;

    Ok(())
  }

  pub async fn del(&self, record_id: &[u8]) -> Result<()> {
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

  fn try_new(data: Vec<u8>) -> Result<Self> {
    if data.len() % Self::CHUNK != 0 {
      return Err(Error::InvalidResourceIndex);
    }

    Ok(Self(data))
  }

  fn into_bytes(self) -> Vec<u8> {
    self.0
  }

  fn iter(&self) -> impl Iterator<Item = &[u8]> {
    self.0.chunks_exact(Self::CHUNK)
  }

  fn contains(&self, tag: &[u8]) -> bool {
    self.iter().any(|chunk| chunk == tag)
  }

  fn insert(&mut self, tag: &[u8]) -> bool {
    if self.contains(tag) {
      return false;
    }

    self.0.extend_from_slice(tag);
    true
  }

  fn remove(&mut self, tag: &[u8]) -> bool {
    let index: Option<usize> = self.iter().position(|chunk| chunk == tag);

    if let Some(index) = index {
      self.0.drain(Self::CHUNK * index..Self::CHUNK * (index + 1));
      return true;
    }

    false
  }

  fn tag(id: &[u8]) -> RecordTag {
    Blake2b256::digest(id)
  }
}

impl Debug for RecordIndex {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_set().entries(self.iter().map(encode_b58)).finish()
  }
}

#[cfg(test)]
mod tests {
  use core::iter;
  use rand::distributions::Alphanumeric;
  use rand::rngs::OsRng;
  use rand::Rng;
  use std::fs;
  use std::path::Path;
  use std::path::PathBuf;

  use crate::stronghold::RecordIndex;
  use crate::stronghold::RecordTag;
  use crate::stronghold::Records;
  use crate::stronghold::Snapshot;

  fn rand_string(chars: usize) -> String {
    iter::repeat(())
      .map(|_| OsRng.sample(Alphanumeric))
      .map(char::from)
      .take(chars)
      .collect()
  }

  fn snapshot_path(root: &str, chars: usize) -> PathBuf {
    AsRef::<Path>::as_ref(root).join(format!("{}.stronghold", rand_string(chars)))
  }

  #[tokio::test]
  async fn test_record_get_set() {
    fs::create_dir_all("./test-storage").unwrap();

    let location: PathBuf = snapshot_path("./test-storage", 10);
    let snapshot: Snapshot = Snapshot::new(&location);

    snapshot.load([0; 32]).await.unwrap();

    let records: Records<'_> = snapshot.records("", &[]);

    assert_eq!(records.all().await.unwrap().len(), 0);

    for index in 0..50_u32 {
      records
        .set(&index.to_be_bytes(), &index.pow(2).to_be_bytes())
        .await
        .unwrap();
    }

    for index in 0..50_u32 {
      let data: Vec<u8> = records.get(&index.to_be_bytes()).await.unwrap();
      assert_eq!(data, &index.pow(2).to_be_bytes());
    }

    for index in 50..100_u32 {
      let data: Vec<u8> = records.get(&index.to_be_bytes()).await.unwrap();
      assert_eq!(data, Vec::<u8>::new());
    }

    snapshot.unload(true).await.unwrap();
    snapshot.load([0; 32]).await.unwrap();

    for index in 0..50_u32 {
      let data: Vec<u8> = records.get(&index.to_be_bytes()).await.unwrap();
      assert_eq!(data, &index.pow(2).to_be_bytes());
    }

    for index in 0..50_u32 {
      records.del(&index.to_be_bytes()).await.unwrap();
    }

    for index in 0..50_u32 {
      let data: Vec<u8> = records.get(&index.to_be_bytes()).await.unwrap();
      assert_eq!(data, Vec::<u8>::new());
    }

    fs::remove_file(&location).unwrap();
  }

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
