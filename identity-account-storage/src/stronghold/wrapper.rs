// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::path::Path;

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
  pub async fn new<'a, T>(path: &T, mut password: String, dropsave: Option<bool>) -> Result<Self>
  where
    T: AsRef<Path> + ?Sized,
  {
    let stronghold: IotaStronghold = IotaStronghold::default();

    let snapshot_path = SnapshotPath::from_path(path);

    let mut key: EncryptionKey = derive_encryption_key(&password);
    let key_provider = KeyProvider::try_from(key.to_vec()).expect("Failed to load key");

    password.zeroize();
    key.zeroize();

    // TODO: Load the snapshot as a side effect, without caring about the client.
    // Stronghold will add a non-client-loading version with another update.
    match stronghold
      .load_client_from_snapshot(b"".to_vec(), &key_provider, &snapshot_path)
      .await
    {
      Ok(_) | Err(ClientError::ClientDataNotPresent) => {}
      Err(err) => return Err(StrongholdError::SnapshotError(SnapshotOperation::Read, err).into()),
    }

    Ok(Self {
      stronghold,
      snapshot_path,
      key_provider,
      index_lock: RwLock::new(()),
      dropsave: dropsave.unwrap_or(true),
    })
  }

  pub(crate) async fn client(&self, client_path: &ClientPath) -> StrongholdResult<Client> {
    self
      .stronghold
      .load_client(client_path.as_ref())
      .await
      .map_err(|err| StrongholdError::ClientError(ClientOperation::Load, client_path.clone(), err))
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
      .await
      .map_err(|err| StrongholdError::ClientError(ClientOperation::Persist, client_path.clone(), err))?;

    Ok(output)
  }

  pub(crate) async fn persist_snapshot(&self) -> StrongholdResult<()> {
    self
      .stronghold
      .commit(&self.snapshot_path, &self.key_provider)
      .await
      .map_err(|err| StrongholdError::SnapshotError(SnapshotOperation::Write, err))
  }
}

impl std::fmt::Debug for Stronghold {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // TODO: Include stronghold types once they implement Debug.
    f.debug_struct("Stronghold").field("dropsave", &self.dropsave).finish()
  }
}

// #[cfg(test)]
// mod tests {
//   use iota_stronghold::Stronghold;

//   #[tokio::test]
//   async fn test_future_is_send() {
//     let stronghold: Stronghold = Stronghold::default();

//     let future = stronghold.load_client(b"client_path".to_vec());

//     fn assert_send<F: std::future::Future + Send>(f: F) {}

//     assert_send(future);
//   }
// }
