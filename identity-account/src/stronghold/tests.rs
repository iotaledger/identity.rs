// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter;
use futures::executor::block_on;
use iota_stronghold::Location;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;
use rusty_fork::rusty_fork_test;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::error::Error;
use crate::stronghold::Runtime;
use crate::stronghold::Snapshot;
use crate::stronghold::SnapshotStatus;
use crate::stronghold::Store;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;

const ROOT: &str = "./test-storage";
const SIZE: usize = 10;

fn location(name: &str) -> Location {
  Location::generic(name, name)
}

fn rand_string(chars: usize) -> String {
  iter::repeat(())
    .map(|_| OsRng.sample(Alphanumeric))
    .map(char::from)
    .take(chars)
    .collect()
}

fn generate_filename() -> PathBuf {
  AsRef::<Path>::as_ref(ROOT).join(format!("{}.stronghold", rand_string(SIZE)))
}

rusty_fork_test! {
  #[test]
  fn test_password_expiration() {
    block_on(async {
      let interval: Duration = Duration::from_millis(500);

      Runtime::set_password_interval(interval).await;

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();

      thread::sleep(interval * 3);

      let store: Store = snapshot.store("", &[]);
      let error: Error = store.get(location("expires")).await.unwrap_err();

      assert!(
        matches!(error, Error::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );

      assert!(
        matches!(snapshot.status().await, SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }
}

rusty_fork_test! {
  #[test]
  fn test_password_persistence() {
    block_on(async {
      let interval: Duration = Duration::from_millis(900);

      Runtime::set_password_interval(interval).await;

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();

      let store: Store = snapshot.store("", &[]);

      for index in 1..6 {
        let instant: Instant = Instant::now();
        let location: Location = location(&format!("persists{}", index));

        store.set(location, "STRONGHOLD".to_string(), None).await.unwrap();

        let status: SnapshotStatus = snapshot.status().await;

        assert!(
          matches!(status, SnapshotStatus::Unlocked(_)),
          "unexpected snapshot status",
        );

        if let Some(timeout) = interval.checked_sub(instant.elapsed()) {
          thread::sleep(timeout / 2);
        } else {
          // if elapsed > interval, set the password again.
          // this might happen if the test is stopped by another thread
          snapshot.set_password(Default::default()).await;
        }
      }

      let result: Vec<u8> = store.get(location("persists1")).await.unwrap();

      assert_eq!(result, b"STRONGHOLD");

      thread::sleep(interval * 2);

      let error: Error = store.get(location("persists1")).await.unwrap_err();

      assert!(
        matches!(error, Error::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );

      assert!(
        matches!(snapshot.status().await, SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }
}

async fn open_snapshot(path: &Path, password: EncryptionKey) -> Snapshot {
  let snapshot: Snapshot = Snapshot::new(path);

  if path.exists() {
    fs::remove_file(path).unwrap();
  }

  snapshot.load(password).await.unwrap();
  snapshot
}

#[tokio::test]
async fn test_store_basics() {
  let password: EncryptionKey = derive_encryption_key("my-password:test_store_basics");
  let snapshot: Snapshot = open_snapshot(&generate_filename(), password).await;

  let store: Store = snapshot.store(b"store", &[]);

  assert!(store.get(location("A")).await.unwrap().is_empty());
  assert!(store.get(location("B")).await.unwrap().is_empty());
  assert!(store.get(location("C")).await.unwrap().is_empty());

  store.set(location("A"), b"foo".to_vec(), None).await.unwrap();
  store.set(location("B"), b"bar".to_vec(), None).await.unwrap();
  store.set(location("C"), b"baz".to_vec(), None).await.unwrap();

  assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
  assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
  assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());

  store.del(location("A")).await.unwrap();
  store.del(location("C")).await.unwrap();

  assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());

  snapshot.unload(true).await.unwrap();

  fs::remove_file(store.path()).unwrap();
}

#[tokio::test]
async fn test_store_multiple_snapshots() {
  let password: EncryptionKey = derive_encryption_key("my-password:test_store_multiple_snapshots");
  let snapshot1: Snapshot = open_snapshot(&generate_filename(), password).await;
  let snapshot2: Snapshot = open_snapshot(&generate_filename(), password).await;
  let snapshot3: Snapshot = open_snapshot(&generate_filename(), password).await;

  let store1: Store = snapshot1.store(b"store1", &[]);
  let store2: Store = snapshot2.store(b"store2", &[]);
  let store3: Store = snapshot3.store(b"store3", &[]);

  for store in &[&store1, &store2, &store3] {
    assert!(store.get(location("A")).await.unwrap().is_empty());
    assert!(store.get(location("B")).await.unwrap().is_empty());
    assert!(store.get(location("C")).await.unwrap().is_empty());
  }

  for store in &[&store1, &store2, &store3] {
    store.set(location("A"), b"foo".to_vec(), None).await.unwrap();
    store.set(location("B"), b"bar".to_vec(), None).await.unwrap();
    store.set(location("C"), b"baz".to_vec(), None).await.unwrap();
  }

  for store in &[&store1, &store2, &store3] {
    assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
    assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
    assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());
  }

  for store in &[&store1, &store2, &store3] {
    store.del(location("A")).await.unwrap();
    store.del(location("C")).await.unwrap();
  }

  for store in &[&store1, &store2, &store3] {
    assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
  }

  snapshot1.unload(true).await.unwrap();
  snapshot2.unload(true).await.unwrap();
  snapshot3.unload(true).await.unwrap();

  for store in &[&store1, &store2, &store3] {
    fs::remove_file(store.path()).unwrap();
  }
}

#[tokio::test]
async fn test_store_persistence() {
  let password: EncryptionKey = derive_encryption_key("my-password:test_store_persistence");
  let filename: PathBuf = generate_filename();

  {
    let snapshot: Snapshot = open_snapshot(&filename, password).await;
    let store: Store = snapshot.store(b"persistence", &[]);

    assert!(store.get(location("A")).await.unwrap().is_empty());
    assert!(store.get(location("B")).await.unwrap().is_empty());
    assert!(store.get(location("C")).await.unwrap().is_empty());

    store.set(location("A"), b"foo".to_vec(), None).await.unwrap();
    store.set(location("B"), b"bar".to_vec(), None).await.unwrap();
    store.set(location("C"), b"baz".to_vec(), None).await.unwrap();

    assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
    assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
    assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());

    snapshot.unload(true).await.unwrap();
  }

  {
    let snapshot: Snapshot = open_snapshot(&filename, password).await;
    let store: Store = snapshot.store(b"persistence", &[]);

    assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
    assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
    assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());

    fs::remove_file(store.path()).unwrap();
  }
}
