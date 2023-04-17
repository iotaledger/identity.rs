// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::StrongholdError;
use super::StrongholdErrorKind;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;
use futures::executor;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::KeyProvider;
use iota_stronghold::SnapshotPath;
use iota_stronghold::Stronghold as IotaStronghold;
use std::path::Path;
use zeroize::Zeroize;

pub type StrongholdResult<T> = Result<T, StrongholdError>;

/// A Wrapper around [Stronghold](https://github.com/iotaledger/stronghold.rs).
///
/// Stronghold is a secure storage for sensitive data. Private keys stored inside a Stronghold
/// can never be read, but only be accessed via cryptographic procedures. Data written into a Stronghold
/// is persisted in snapshots which are encrypted using the provided password.
///
/// This wrapper implements [`JwkStorage`](crate::key_storage::JwkStorage) and
/// [`KeyIdStorage`](crate::key_id_storage::KeyIdStorage).
#[derive(Debug)]
pub struct Stronghold {
  pub(crate) stronghold: IotaStronghold,
  snapshot_path: SnapshotPath,
  key_provider: KeyProvider,
  pub(crate) flush_on_drop: bool,
  pub(crate) client_path: Vec<u8>,
  pub(crate) client: Client,
  pub(crate) vault_path: String,
}

static DEFAULT_VAULT_PATH: &str = "&default_vault_path";
static CLIENT_PATH: &[u8] = b"&client_path";

impl Stronghold {
  /// Constructs a Stronghold storage instance.
  ///
  /// Arguments:
  ///
  /// * `path`: path to a local Stronghold snapshot file. Will be created if it does not exist.
  /// * `password`: password for the Stronghold snapshot file. If this is cloned from a reference,
  /// zeroization of that reference is strongly recommended.
  /// * `flush_on_drop`: calls [`Self::flush()`] when the instance is dropped. Default: true.
  /// * `vault_path`: path to the stronghold vault where the secrets are stored. Default: [`DEFAULT_VAULT_PATH`].
  ///
  /// Note that the snapshot file does not exist, it will be created on first write when
  /// [`Self::flush()`] is called.
  pub async fn new<T>(
    path: &T,
    mut password: String,
    flush_on_drop: Option<bool>,
    vault_path: Option<String>,
  ) -> StrongholdResult<Self>
  where
    T: AsRef<Path> + ?Sized,
  {
    let stronghold: IotaStronghold = IotaStronghold::default();
    let mut key: EncryptionKey = derive_encryption_key(&password);
    password.zeroize();
    let key_provider: KeyProvider =
      KeyProvider::try_from(key.to_vec()).map_err(|_err| StrongholdError::new(StrongholdErrorKind::MemoryError))?;
    key.zeroize();

    // If the snapshot file exists, we load it.
    // If it doesn't we write data into the in memory `Stronghold` and only persist to disk on first write.
    let snapshot_path: SnapshotPath = if path.as_ref().exists() {
      let snapshot_path = SnapshotPath::from_path(path);

      stronghold
        .load_snapshot(&key_provider, &snapshot_path)
        .map_err(|err| StrongholdError::new(StrongholdErrorKind::SnapshotOperation).with_source(err))?;

      snapshot_path
    } else {
      SnapshotPath::from_path(path)
    };

    let client = match stronghold.load_client(CLIENT_PATH) {
      Ok(client) => Ok(client),
      Err(ClientError::ClientDataNotPresent) => stronghold
        .create_client(CLIENT_PATH)
        .map_err(|err| StrongholdError::new(StrongholdErrorKind::ClientError).with_source(err)),
      Err(err) => Err(StrongholdError::new(StrongholdErrorKind::ClientError).with_source(err)),
    }?;

    let vault_path = vault_path.unwrap_or(DEFAULT_VAULT_PATH.to_owned());
    let client_path = CLIENT_PATH.to_vec();
    Ok(Self {
      stronghold,
      snapshot_path,
      key_provider,
      flush_on_drop: flush_on_drop.unwrap_or(true),
      client_path,
      client,
      vault_path,
    })
  }

  /// Returns whether flush_on_drop is enabled.
  pub fn flush_on_drop(&self) -> bool {
    self.flush_on_drop
  }

  /// Sets whether flush_on_drop is enabled.
  pub fn set_flush_on_drop(&mut self, flush_on_drop: bool) {
    self.flush_on_drop = flush_on_drop;
  }

  // Returns the vault path.
  pub fn vault_path(&self) -> String {
    self.vault_path.clone()
  }

  /// Persist changes to disk.
  pub async fn flush(&self) -> StrongholdResult<()> {
    if let Some(parent) = self.snapshot_path.as_path().parent() {
      if !parent.exists() {
        tokio::fs::create_dir_all(parent)
          .await
          .map_err(|err| StrongholdError::new(StrongholdErrorKind::SnapshotOperation).with_source(err))?;
      }
    }

    self
      .stronghold
      .write_client(self.client_path.clone())
      .map_err(|err| StrongholdError::new(StrongholdErrorKind::ClientError).with_source(err))?;

    self
      .stronghold
      .commit_with_keyprovider(&self.snapshot_path, &self.key_provider)
      .map_err(|err| StrongholdError::new(StrongholdErrorKind::ClientError).with_source(err))
  }
}

impl Drop for Stronghold {
  fn drop(&mut self) {
    if self.flush_on_drop {
      let _ = executor::block_on(self.flush());
    }
  }
}

#[cfg(test)]
mod tests {

  use identity_verification::jws::JwsAlgorithm;
  use rand::distributions::DistString;
  use rand::rngs::OsRng;

  use crate::stronghold::Stronghold;
  use crate::JwkStorage;
  use crate::KeyType;
  #[tokio::test]
  pub async fn test_flush_on_drop_true() {
    let path: String = random_temporary_path();
    let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(true), None)
      .await
      .unwrap();

    let generate = stronghold
      .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
      .await
      .unwrap();
    let key_id = &generate.key_id;
    assert!(stronghold.exists(key_id).await.unwrap());

    std::mem::drop(stronghold);
    let stronghold_2: Stronghold = Stronghold::new(&path, "pass".to_owned(), None, None).await.unwrap();
    assert!(stronghold_2.exists(key_id).await.unwrap());
  }

  #[tokio::test]
  pub async fn test_flush_on_drop_false() {
    let path: String = random_temporary_path();
    let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(false), None)
      .await
      .unwrap();

    let generate = stronghold
      .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
      .await
      .unwrap();
    let key_id = &generate.key_id;
    assert!(stronghold.exists(key_id).await.unwrap());

    std::mem::drop(stronghold);
    let stronghold_2: Stronghold = Stronghold::new(&path, "pass".to_owned(), None, None).await.unwrap();
    assert!(!stronghold_2.exists(key_id).await.unwrap());
  }

  #[tokio::test]
  pub async fn test_flush() {
    let path: String = random_temporary_path();
    let stronghold: Stronghold = Stronghold::new(&path, "pass".to_owned(), Some(false), None)
      .await
      .unwrap();

    let generate = stronghold
      .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
      .await
      .unwrap();
    let key_id = &generate.key_id;
    assert!(stronghold.exists(key_id).await.unwrap());

    stronghold.flush().await.unwrap();
    std::mem::drop(stronghold);
    let stronghold_2: Stronghold = Stronghold::new(&path, "pass".to_owned(), None, None).await.unwrap();
    assert!(stronghold_2.exists(key_id).await.unwrap());
  }

  pub(crate) fn random_temporary_path() -> String {
    let mut file = std::env::temp_dir();
    file.push("test_strongholds");
    file.push(rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32));
    file.set_extension("stronghold");
    file.to_str().unwrap().to_owned()
  }
}
