// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::StrongholdFlags;
use std::path::Path;
use std::time::Duration;

use crate::stronghold::error::IotaStrongholdResult;
use crate::stronghold::Context;

use super::ClientPath;

#[derive(Debug)]
pub struct Store<'snapshot> {
  path: &'snapshot Path,
  client_path: ClientPath,
  flags: Vec<StrongholdFlags>,
}

impl<'snapshot> Store<'snapshot> {
  pub(crate) fn new<P>(path: &'snapshot P, client_path: ClientPath, flags: &[StrongholdFlags]) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    Self {
      path: path.as_ref(),
      client_path,
      flags: flags.to_vec(),
    }
  }
}

impl Store<'_> {
  /// Returns the snapshot path of the store.
  pub fn path(&self) -> &Path {
    self.path
  }

  /// Returns the name of the store.
  pub fn client_path(&self) -> &[u8] {
    self.client_path.0.as_ref()
  }

  /// Returns the store policy options.
  pub fn flags(&self) -> &[StrongholdFlags] {
    &self.flags
  }

  /// Gets a record.
  pub async fn get(&self, key: impl Into<Vec<u8>>) -> IotaStrongholdResult<Option<Vec<u8>>> {
    let scope: _ = Context::scope(self.path, self.client_path(), &self.flags).await?;
    Ok(scope.read_from_store(key.into()).await?)
  }

  /// Adds a record.
  pub async fn set<T>(&self, key: impl Into<Vec<u8>>, payload: T, ttl: Option<Duration>) -> IotaStrongholdResult<()>
  where
    T: Into<Vec<u8>>,
  {
    Context::scope(self.path, self.client_path(), &self.flags)
      .await?
      .write_to_store(key.into(), payload.into(), ttl)
      .await?;
    Ok(())
  }

  /// Removes a record.
  pub async fn del(&self, key: impl Into<Vec<u8>>) -> IotaStrongholdResult<()> {
    Context::scope(self.path, self.client_path(), &self.flags)
      .await?
      .delete_from_store(key.into())
      .await?;
    Ok(())
  }
}
