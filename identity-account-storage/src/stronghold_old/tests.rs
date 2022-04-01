// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter;
use futures::executor::block_on;
use iota_stronghold_old::procedures::KeyType;
use iota_stronghold_old::Location;
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

use crate::stronghold_old::default_hint;
use crate::stronghold_old::IotaStrongholdResult;
use crate::stronghold_old::Snapshot;
use crate::stronghold_old::SnapshotStatus;
use crate::stronghold_old::Store;
use crate::stronghold_old::StrongholdError;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;
use identity_core::crypto::KeyPair;

const TEST_DIR: &str = "./test-storage";
const RANDOM_FILENAME_SIZE: usize = 10;

fn location(name: &str) -> Location {
  Location::generic("$fixed_vault_path", name)
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

      Snapshot::set_password_clear(interval).await.unwrap();

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();

      // Wait for password to be cleared
      thread::sleep(interval * 3);

      let store: Store<'_> = snapshot.store("".into());
      let error: StrongholdError = store.get("expires").await.unwrap_err();

      assert!(
        matches!(error, StrongholdError::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );

      assert!(
        matches!(snapshot.status().await.unwrap(), SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }

  #[test]
  fn test_password_persistence() {
    block_on(async {
      let interval: Duration = Duration::from_millis(300);

      Snapshot::set_password_clear(interval).await.unwrap();

      let filename: PathBuf = generate_filename();
      let snapshot: Snapshot = Snapshot::new(&filename);

      snapshot.load(Default::default()).await.unwrap();
      let mut instant: Instant = Instant::now();

      let store: Store<'_> = snapshot.store("".into());
      for index in 1..=5u8 {
        let location: String = format!("persists{}", index);

        let set_result = store.set(location, format!("STRONGHOLD{}", index), None).await;
        let status: SnapshotStatus = snapshot.status().await.unwrap();

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
          snapshot.set_password(Default::default()).await.unwrap();
          instant = Instant::now();
        }
      }

      let mut result: IotaStrongholdResult<Option<Vec<u8>>> = store.get("persists1").await;

      // Test may have taken too long / been interrupted and cleared the password already, retry
      if matches!(result, Err(StrongholdError::StrongholdPasswordNotSet)) && interval.checked_sub(instant.elapsed()).is_none() {
        snapshot.set_password(Default::default()).await.unwrap();
        result = store.get("persists1").await;
      }
      assert_eq!(result.unwrap(), Some(b"STRONGHOLD1".to_vec()));

      // Wait for password to be cleared
      thread::sleep(interval * 2);

      let error: StrongholdError = store.get("persists1").await.unwrap_err();
      assert!(
        matches!(error, StrongholdError::StrongholdPasswordNotSet),
        "unexpected error: {:?}",
        error
      );
      assert!(
        matches!(snapshot.status().await.unwrap(), SnapshotStatus::Locked),
        "unexpected snapshot status",
      );
    })
  }

  #[test]
  fn test_store_basics() {
    block_on(async {
      let password: EncryptionKey = derive_encryption_key("my-password:test_store_basics");
      let snapshot: Snapshot = open_snapshot(&generate_filename(), password).await;

      let store: Store<'_> = snapshot.store("store".into());

      assert!(store.get("A").await.unwrap().is_none());
      assert!(store.get("B").await.unwrap().is_none());
      assert!(store.get("C").await.unwrap().is_none());

      store.set("A", b"foo".to_vec(), None).await.unwrap();
      store.set("B", b"bar".to_vec(), None).await.unwrap();
      store.set("C", b"baz".to_vec(), None).await.unwrap();

      assert_eq!(store.get("A").await.unwrap(), Some(b"foo".to_vec()));
      assert_eq!(store.get("B").await.unwrap(), Some(b"bar".to_vec()));
      assert_eq!(store.get("C").await.unwrap(), Some(b"baz".to_vec()));

      store.del("A").await.unwrap();
      store.del("C").await.unwrap();

      assert_eq!(store.get("B").await.unwrap(), Some(b"bar".to_vec()));

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

      let store1: Store<'_> = snapshot1.store("store1".into());
      let store2: Store<'_> = snapshot2.store("store2".into());
      let store3: Store<'_> = snapshot3.store("store3".into());
      let stores: &[_] = &[&store1, &store2, &store3];

      for store in stores {
        assert!(store.get("A").await.unwrap().is_none());
        assert!(store.get("B").await.unwrap().is_none());
        assert!(store.get("C").await.unwrap().is_none());
      }

      for store in stores {
        store.set("A", b"foo".to_vec(), None).await.unwrap();
        store.set("B", b"bar".to_vec(), None).await.unwrap();
        store.set("C", b"baz".to_vec(), None).await.unwrap();
      }

      for store in stores {
        assert_eq!(store.get("A").await.unwrap(), Some(b"foo".to_vec()));
        assert_eq!(store.get("B").await.unwrap(), Some(b"bar".to_vec()));
        assert_eq!(store.get("C").await.unwrap(), Some(b"baz".to_vec()));
      }

      for store in stores {
        store.del("A").await.unwrap();
        store.del("C").await.unwrap();
      }

      for store in stores {
        assert_eq!(store.get("B").await.unwrap(), Some(b"bar".to_vec()));
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
        let store: Store<'_> = snapshot.store("persistence".into());

        assert!(store.get("A").await.unwrap().is_none());
        assert!(store.get("B").await.unwrap().is_none());
        assert!(store.get("C").await.unwrap().is_none());

        store.set("A", b"foo".to_vec(), None).await.unwrap();
        store.set("B", b"bar".to_vec(), None).await.unwrap();
        store.set("C", b"baz".to_vec(), None).await.unwrap();

        assert_eq!(store.get("A").await.unwrap(), Some(b"foo".to_vec()));
        assert_eq!(store.get("B").await.unwrap(), Some( b"bar".to_vec()));
        assert_eq!(store.get("C").await.unwrap(), Some(b"baz".to_vec()));

        snapshot.unload(true).await.unwrap();
      }

      {
        let snapshot: Snapshot = load_snapshot(&filename, password).await;
        let store: Store<'_> = snapshot.store("persistence".into());

        assert_eq!(store.get("A").await.unwrap(), Some(b"foo".to_vec()));
        assert_eq!(store.get("B").await.unwrap(), Some(b"bar".to_vec()));
        assert_eq!(store.get("C").await.unwrap(), Some(b"baz".to_vec()));

        fs::remove_file(store.path()).unwrap();
      }
    })
  }


  #[test]
  fn test_store_private_key() {
    block_on(async {
      let password: EncryptionKey = derive_encryption_key("my-password:test_vault_persistence");
      let filename: PathBuf = generate_filename();

      let keypair = KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();

      {
        let snapshot: Snapshot = open_snapshot(&filename, password).await;
        let vault = snapshot.vault("persistence".into());

        vault.insert(location("A"), keypair.private().as_ref(), default_hint(), &[]).await.unwrap();

        snapshot.unload(true).await.unwrap();
      }

      {
        let snapshot: Snapshot = load_snapshot(&filename, password).await;

        let vault = snapshot.vault("persistence".into());
        assert!(vault.exists(location("A")).await.unwrap());

        let procedure = iota_stronghold_old::procedures::PublicKey{
          private_key: location("A"),
          ty: KeyType::Ed25519
        };
        let pubkey = vault.execute(procedure).await.unwrap();

        assert_eq!(pubkey, keypair.public().as_ref());

        fs::remove_file(filename).unwrap();
      }
    })
  }
}
