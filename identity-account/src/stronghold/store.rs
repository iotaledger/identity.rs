// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::Location;
use iota_stronghold::StrongholdFlags;
use std::path::Path;
use std::time::Duration;

use crate::stronghold::error::IotaStrongholdResult;
use crate::stronghold::Context;
use crate::stronghold::StrongholdError;

const STRONG_404: &str = "Unable to read from store";

#[derive(Debug)]
pub struct Store<'snapshot> {
  path: &'snapshot Path,
  name: Vec<u8>,
  flags: Vec<StrongholdFlags>,
}

impl<'snapshot> Store<'snapshot> {
  pub(crate) fn new<P, T>(path: &'snapshot P, name: &T, flags: &[StrongholdFlags]) -> Self
  where
    P: AsRef<Path> + ?Sized,
    T: AsRef<[u8]> + ?Sized,
  {
    Self {
      path: path.as_ref(),
      name: name.as_ref().to_vec(),
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
  pub fn name(&self) -> &[u8] {
    &self.name
  }

  /// Returns the store policy options.
  pub fn flags(&self) -> &[StrongholdFlags] {
    &self.flags
  }

  /// Gets a record.
  pub async fn get(&self, location: Location) -> IotaStrongholdResult<Option<Vec<u8>>> {
    match self.get_strict(location.vault_path().to_vec()).await {
      Ok(data) => Ok(data),
      Err(StrongholdError::StrongholdResult(message)) if message == STRONG_404 => Ok(Some(Vec::new())),
      Err(error) => Err(error),
    }
  }

  /// Gets a record.
  pub async fn get_strict(&self, location: Vec<u8>) -> IotaStrongholdResult<Option<Vec<u8>>> {
    let scope: _ = Context::scope(self.path, &self.name, &self.flags).await?;
    Ok(scope.read_from_store(location).await?)
  }

  /// Adds a record.
  pub async fn set<T>(&self, location: Location, payload: T, ttl: Option<Duration>) -> IotaStrongholdResult<()>
  where
    T: Into<Vec<u8>>,
  {
    let location = location.vault_path().to_vec();
    let _a = Context::scope(self.path, &self.name, &self.flags)
      .await?
      .write_to_store(location, payload.into(), ttl)
      .await?;
    Ok(())
  }

  /// Removes a record.
  pub async fn del(&self, location: Location) -> IotaStrongholdResult<()> {
    Context::scope(self.path, &self.name, &self.flags)
      .await?
      .delete_from_store(location.vault_path().to_vec())
      .await?;
    Ok(())
  }

  /// Returns true if the specified location exists.
  pub async fn exists(&self, location: Location) -> IotaStrongholdResult<bool> {
    let scope: _ = Context::scope(self.path, &self.name, &self.flags).await?;
    Ok(scope.record_exists(location).await?)
  }
}
