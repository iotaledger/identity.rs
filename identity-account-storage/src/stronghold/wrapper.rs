// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::path::Path;
use std::path::PathBuf;

use identity_iota_core::did::IotaDID;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::KeyProvider;
use iota_stronghold::SnapshotPath;
use iota_stronghold::Stronghold as IotaStronghold;
use tokio::sync::RwLock;
use zeroize::Zeroize;

use crate::stronghold::error::StrongholdError;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;
use crate::Result;

use super::client_path::ClientPath;
use super::ClientOperation;
use super::SnapshotOperation;
use super::StrongholdResult;

// #[derive(Debug)]
pub struct Stronghold {
  pub(crate) stronghold: IotaStronghold,
  pub(crate) snapshot_path: SnapshotPath,
  pub(crate) key_provider: KeyProvider,
  pub(crate) index_lock: RwLock<()>,
  pub(crate) dropsave: bool,
}

impl Stronghold {
  /// Constructs a Stronghold storage instance.
  ///
  /// Arguments:
  ///
  /// * path: path to a local Stronghold file, will be created if it does not exist.
  /// * password: password for the Stronghold file.
  /// * dropsave: save all changes when the instance is dropped. Default: true.
  pub async fn new<T>(path: &T, mut password: String, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
  {
    let stronghold: IotaStronghold = IotaStronghold::default();
    let path: PathBuf = path.as_ref().to_owned();

    let mut key: EncryptionKey = derive_encryption_key(&password);
    password.zeroize();

    let key_provider = KeyProvider::try_from(key.to_vec()).map_err(StrongholdError::Memory)?;
    key.zeroize();

    let snapshot_path: SnapshotPath = if path.exists() {
      let snapshot_path = SnapshotPath::from_path(path);

      stronghold
        .load_snapshot(&key_provider, &snapshot_path)
        .map_err(|err| StrongholdError::Snapshot(SnapshotOperation::Read, snapshot_path.clone(), err))?;

      snapshot_path
    } else {
      SnapshotPath::from_path(path)
    };

    Ok(Self {
      stronghold,
      snapshot_path,
      key_provider,
      index_lock: RwLock::new(()),
      dropsave: dropsave.unwrap_or(true),
    })
  }

  pub(crate) async fn client(&self, client_path: &ClientPath) -> StrongholdResult<Client> {
    match self.stronghold.load_client(client_path.as_ref()) {
      Ok(client) => Ok(client),
      Err(ClientError::ClientDataNotPresent) => self
        .stronghold
        .create_client(client_path.as_ref())
        .map_err(|err| StrongholdError::Client(ClientOperation::Load, client_path.clone(), err)),
      Err(err) => Err(StrongholdError::Client(ClientOperation::Load, client_path.clone(), err)),
    }
  }

  pub(crate) async fn mutate_client<FUN, OUT, FUT>(&self, did: &IotaDID, f: FUN) -> Result<OUT>
  where
    FUN: FnOnce(Client) -> FUT,
    FUT: Future<Output = Result<OUT>>,
  {
    let client_path: ClientPath = ClientPath::from(did);
    let client: Client = self.client(&client_path).await?;

    // Don't need to write client if this operation fails, hence ?.
    let output: OUT = f(client).await?;

    self
      .stronghold
      .write_client(client_path.as_ref())
      .map_err(|err| StrongholdError::Client(ClientOperation::Persist, client_path.clone(), err))?;

    Ok(output)
  }

  // TODO: Does not need to be async as of right now, but it should!
  pub(crate) async fn persist_snapshot(&self) -> StrongholdResult<()> {
    self
      .stronghold
      .commit(&self.snapshot_path, &self.key_provider)
      .map_err(|err| StrongholdError::Snapshot(SnapshotOperation::Write, self.snapshot_path.clone(), err))
  }
}

impl std::fmt::Debug for Stronghold {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // TODO: Include stronghold type(s) once they implement Debug.
    f.debug_struct("Stronghold")
      //.field("stronghold", &self.stronghold)
      .field("snapshot_path", &self.snapshot_path)
      //.field("key_provider", &self.key_provider)
      .field("index_lock", &self.index_lock)
      .field("dropsave", &self.dropsave)
      .finish()
  }
}
