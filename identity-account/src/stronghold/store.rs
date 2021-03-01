// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::Location;
use iota_stronghold::StrongholdFlags;
use std::path::Path;
use std::time::Duration;

use crate::error::Error;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::error::Result;
use crate::stronghold::Runtime;

const STRONG_404: &str = "Unable to read from store";

#[derive(Debug)]
pub struct Store<'path> {
  flags: Vec<StrongholdFlags>,
  name: Vec<u8>,
  path: &'path Path,
}

impl<'path> Store<'path> {
  pub(crate) fn new<T>(path: &'path Path, name: &T, flags: &[StrongholdFlags]) -> Self
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Self {
      flags: flags.to_vec(),
      name: name.as_ref().to_vec(),
      path,
    }
  }
}

impl Store<'_> {
  pub fn name(&self) -> &[u8] {
    &self.name
  }

  pub fn path(&self) -> &Path {
    self.path
  }

  pub fn flags(&self) -> &[StrongholdFlags] {
    &self.flags
  }

  pub async fn flush(&self) -> Result<()> {
    Runtime::lock().await?.write_snapshot(self.path).await
  }

  /// Gets a record.
  pub async fn get(&self, location: Location) -> Result<Vec<u8>> {
    match self.get_strict(location).await {
      Ok(data) => Ok(data),
      Err(Error::StrongholdResult(message)) if message == STRONG_404 => Ok(Vec::new()),
      Err(error) => Err(error),
    }
  }

  /// Gets a record.
  pub async fn get_strict(&self, location: Location) -> Result<Vec<u8>> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(self.path).await?;
    runtime.load_actor(self.path, &self.name, &self.flags).await?;

    let (data, status): (Vec<u8>, _) = runtime.read_from_store(location).await;

    status.to_result()?;

    Ok(data)
  }

  /// Adds a record.
  pub async fn set(&self, location: Location, payload: Vec<u8>, ttl: Option<Duration>) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(self.path).await?;
    runtime.load_actor(self.path, &self.name, &self.flags).await?;
    runtime.write_to_store(location, payload, ttl).await.to_result()?;

    Ok(())
  }

  /// Removes a record.
  pub async fn del(&self, location: Location) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(self.path).await?;
    runtime.load_actor(self.path, &self.name, &self.flags).await?;
    runtime.delete_from_store(location).await.to_result()?;

    Ok(())
  }
}
