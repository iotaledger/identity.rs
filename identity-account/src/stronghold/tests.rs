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
use crate::stronghold::Snapshot;
use crate::stronghold::SnapshotStatus;
use crate::stronghold::Store;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;

const TEST_DIR: &str = "./test-storage";
const RANDOM_FILENAME_SIZE: usize = 10;

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
  AsRef::<Path>::as_ref(TEST_DIR).join(format!("{}.stronghold", rand_string(RANDOM_FILENAME_SIZE)))
}

async fn open_snapshot(path: &Path, password: EncryptionKey) -> Snapshot {
  if path.exists() {
    fs::remove_file(path).unwrap();
  }

  load_snapshot(path, password).await
}

async fn load_snapshot(path: &Path, password: EncryptionKey) -> Snapshot {
  let snapshot: Snapshot = Snapshot::new(path);
  snapshot.load(password).await.unwrap();
  snapshot
}

rusty_fork_test! {
  #[test]
  fn test_password_expiration() {
    block_on(async {
      let interval: Duration = Duration::from_millis(100);

      Snapshot::set_password_clear(interval).unwrap();

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();

      // Wait for password to be cleared
      thread::sleep(interval * 3);

      let store: Store<'_> = snapshot.store("", &[]);
      let error: Error = store.get(location("expires")).await.unwrap_err();

      assert!(
        matches!(error, Error::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );

      assert!(
        matches!(snapshot.status().unwrap(), SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }

  #[test]
  fn test_password_persistence() {
    block_on(async {
      let interval: Duration = Duration::from_millis(300);

      Snapshot::set_password_clear(interval).unwrap();

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();
      let mut instant: Instant = Instant::now();

      let store: Store<'_> = snapshot.store("", &[]);
      for index in 1..=5u8 {
        let location: Location = location(&format!("persists{}", index));

        let set_result = store.set(location, format!("STRONGHOLD{}", index), None).await;
        let status: SnapshotStatus = snapshot.status().unwrap();

        if let Some(timeout) = interval.checked_sub(instant.elapsed()) {
          // Prior to the expiration time, the password should not be cleared yet
          assert!(
            set_result.is_ok(),
            "set failed"
          );
          assert!(
            matches!(status, SnapshotStatus::Unlocked(_)),
            "unexpected snapshot status",
          );

          thread::sleep(timeout / 2);
        } else {
          // If elapsed > interval, set the password again.
          // This might happen if the test is stopped by another thread.
          snapshot.set_password(Default::default()).unwrap();
          instant = Instant::now();
        }
      }

      let mut result: Result<Vec<u8>, Error> = store.get(location("persists1")).await;

      // Test may have taken too long / been interrupted and cleared the password already, retry
      if matches!(result, Err(Error::StrongholdPasswordNotSet)) && interval.checked_sub(instant.elapsed()).is_none() {
        snapshot.set_password(Default::default()).unwrap();
        result = store.get(location("persists1")).await;
      }
      assert_eq!(result.unwrap(), b"STRONGHOLD1");

      // Wait for password to be cleared
      thread::sleep(interval * 2);

      let error: Error = store.get(location("persists1")).await.unwrap_err();
      assert!(
        matches!(error, Error::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );
      assert!(
        matches!(snapshot.status().unwrap(), SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }

  #[test]
  fn test_store_basics() {
    block_on(async {
      let password: EncryptionKey = derive_encryption_key("my-password:test_store_basics");
      let snapshot: Snapshot = open_snapshot(&generate_filename(), password).await;

      let store: Store<'_> = snapshot.store(b"store", &[]);

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
    })
  }

  #[test]
  fn test_store_multiple_snapshots() {
    block_on(async {
      let password: EncryptionKey = derive_encryption_key("my-password:test_store_multiple_snapshots");
      let snapshot1: Snapshot = open_snapshot(&generate_filename(), password).await;
      let snapshot2: Snapshot = open_snapshot(&generate_filename(), password).await;
      let snapshot3: Snapshot = open_snapshot(&generate_filename(), password).await;

      let store1: Store<'_> = snapshot1.store(b"store1", &[]);
      let store2: Store<'_> = snapshot2.store(b"store2", &[]);
      let store3: Store<'_> = snapshot3.store(b"store3", &[]);
      let stores: &[_] = &[&store1, &store2, &store3];

      for store in stores {
        assert!(store.get(location("A")).await.unwrap().is_empty());
        assert!(store.get(location("B")).await.unwrap().is_empty());
        assert!(store.get(location("C")).await.unwrap().is_empty());
      }

      for store in stores {
        store.set(location("A"), b"foo".to_vec(), None).await.unwrap();
        store.set(location("B"), b"bar".to_vec(), None).await.unwrap();
        store.set(location("C"), b"baz".to_vec(), None).await.unwrap();
      }

      for store in stores {
        assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
        assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
        assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());
      }

      for store in stores {
        store.del(location("A")).await.unwrap();
        store.del(location("C")).await.unwrap();
      }

      for store in stores {
        assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
      }

      snapshot1.unload(true).await.unwrap();
      snapshot2.unload(true).await.unwrap();
      snapshot3.unload(true).await.unwrap();

      for store in stores {
        fs::remove_file(store.path()).unwrap();
      }
    })
  }

  #[test]
  fn test_store_persistence() {
    block_on(async {
      let password: EncryptionKey = derive_encryption_key("my-password:test_store_persistence");
      let filename: PathBuf = generate_filename();

      {
        let snapshot: Snapshot = open_snapshot(&filename, password).await;
        let store: Store<'_> = snapshot.store(b"persistence", &[]);

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
        let snapshot: Snapshot = load_snapshot(&filename, password).await;
        let store: Store<'_> = snapshot.store(b"persistence", &[]);

        assert_eq!(store.get(location("A")).await.unwrap(), b"foo".to_vec());
        assert_eq!(store.get(location("B")).await.unwrap(), b"bar".to_vec());
        assert_eq!(store.get(location("C")).await.unwrap(), b"baz".to_vec());

        fs::remove_file(store.path()).unwrap();
      }
    })
  }
}
