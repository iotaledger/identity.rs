// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use identity_iota_core::did::IotaDID;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::KeyProvider;
use iota_stronghold::SnapshotPath;
use iota_stronghold::Stronghold as IotaStronghold;
use tokio::sync::RwLock;
use zeroize::Zeroize;

use crate::stronghold::error::ClientOperation;
use crate::stronghold::error::SnapshotOperation;
use crate::stronghold::error::StrongholdError;
use crate::stronghold::ClientPath;
use crate::stronghold::StrongholdResult;
use crate::utils::derive_encryption_key;
use crate::utils::EncryptionKey;
use crate::Result;

/// The implementation of the `Storage` interface using `Stronghold`.
///
/// Stronghold is a secure storage for sensitive data. Secrets that are stored inside a Stronghold
/// can never be read, but only be accessed via cryptographic procedures. Data written into a Stronghold
/// is persisted in snapshots which are encrypted using the provided password.
#[derive(Debug)]
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
  /// * `path`: path to a local Stronghold snapshot file. Will be created if it does not exist.
  /// * `password`: password for the Stronghold snapshot file. If this is cloned from a reference,
  /// zeroization of that reference is strongly recommended.
  /// * `dropsave`: persist all changes when the instance is dropped. Default: true.
  pub async fn new<T>(path: &T, mut password: String, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
  {
    let stronghold: IotaStronghold = IotaStronghold::default();

    let mut key: EncryptionKey = derive_encryption_key(&password);
    password.zeroize();

    let key_provider = KeyProvider::try_from(key.to_vec()).map_err(StrongholdError::Memory)?;
    key.zeroize();

    // If the snapshot file exists, we load it.
    // If it doesn't we write data into the in memory `Stronghold` and only persist to disk on first write.
    let snapshot_path: SnapshotPath = if path.as_ref().exists() {
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

  /// Load the client identified by the given `client_path`.
  pub(crate) fn client(&self, client_path: &ClientPath) -> StrongholdResult<Client> {
    match self.stronghold.load_client(client_path.as_ref()) {
      Ok(client) => Ok(client),
      Err(ClientError::ClientDataNotPresent) => self
        .stronghold
        .create_client(client_path.as_ref())
        .map_err(|err| StrongholdError::Client(ClientOperation::Load, client_path.clone(), err)),
      Err(err) => Err(StrongholdError::Client(ClientOperation::Load, client_path.clone(), err)),
    }
  }

  /// Load the client for the given `did` and apply function `f` to it.
  /// The (potentially) modified client is then written to the stronghold's snapshot state.
  pub(crate) fn mutate_client<FUN, OUT>(&self, did: &IotaDID, f: FUN) -> Result<OUT>
  where
    FUN: FnOnce(Client) -> Result<OUT>,
  {
    let client_path: ClientPath = ClientPath::from(did);
    let client: Client = self.client(&client_path)?;

    // Don't need to write client if this fails, so bailing early (`?` operator) is okay.
    let output: OUT = f(client)?;

    self
      .stronghold
      .write_client(client_path.as_ref())
      .map_err(|err| StrongholdError::Client(ClientOperation::Persist, client_path.clone(), err))?;

    Ok(output)
  }

  /// Encrypt the snapshot with the internal key provider and persist it to disk.
  // TODO: Does not need to be async as of now, but will be eventually, so we already make it async.
  pub(crate) async fn persist_snapshot(&self) -> StrongholdResult<()> {
    // TODO: Create parent dirs if they don't exist.
    self
      .stronghold
      .commit(&self.snapshot_path, &self.key_provider)
      .map_err(|err| StrongholdError::Snapshot(SnapshotOperation::Write, self.snapshot_path.clone(), err))
  }
}
