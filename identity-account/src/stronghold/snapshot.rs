// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::StrongholdFlags;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::error::Result;
use crate::stronghold::Context;
use crate::stronghold::Password;
use crate::stronghold::Records;
use crate::stronghold::SnapshotStatus;
use crate::stronghold::Store;
use crate::stronghold::Vault;

#[derive(Debug)]
pub struct Snapshot {
  path: PathBuf,
}

impl Snapshot {
  pub fn set_password_clear(interval: Duration) -> Result<()> {
    Context::set_password_clear(interval)
  }

  pub fn on_change<T>(listener: T) -> Result<()>
  where
    T: FnMut(&Path, &SnapshotStatus) + Send + 'static,
  {
    Context::on_change(listener)
  }

  pub fn new<P>(path: &P) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    Self {
      path: path.as_ref().to_path_buf(),
    }
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn vault<T>(&self, name: &T, flags: &[StrongholdFlags]) -> Vault<'_>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Vault::new(&self.path, name, flags)
  }

  pub fn store<T>(&self, name: &T, flags: &[StrongholdFlags]) -> Store<'_>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Store::new(&self.path, name, flags)
  }

  pub fn records<T>(&self, name: &T, flags: &[StrongholdFlags]) -> Records<'_>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Records::new(&self.path, name, flags)
  }

  pub fn status(&self) -> Result<SnapshotStatus> {
    Context::snapshot_status(&self.path)
  }

  pub fn set_password(&self, password: Password) -> Result<()> {
    Context::set_password(&self.path, password)
  }

  pub async fn load(&self, password: Password) -> Result<()> {
    Context::load(&self.path, password).await
  }

  pub async fn unload(&self, persist: bool) -> Result<()> {
    Context::unload(&self.path, persist).await
  }

  pub async fn save(&self) -> Result<()> {
    Context::save(&self.path).await
  }
}
