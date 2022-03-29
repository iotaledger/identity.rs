// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_stronghold::StrongholdFlags;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::stronghold::Context;
use crate::stronghold::IotaStrongholdResult;
use crate::stronghold::Password;
use crate::stronghold::SnapshotStatus;
use crate::stronghold::Store;
use crate::stronghold::Vault;

use super::Database;

#[derive(Debug)]
pub struct Snapshot {
  path: PathBuf,
}

impl Snapshot {
  pub async fn set_password_clear(interval: Duration) -> IotaStrongholdResult<()> {
    Context::set_password_clear(interval).await
  }

  pub async fn on_change<T>(listener: T) -> IotaStrongholdResult<()>
  where
    T: FnMut(&Path, &SnapshotStatus) + Send + 'static,
  {
    Context::on_change(listener).await
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

  pub async fn stronghold(&self) -> IotaStrongholdResult<tokio::sync::MutexGuard<'static, Database>> {
    Context::scope(&self.path, "doesntmatter".as_ref(), &[]).await
  }

  pub async fn status(&self) -> IotaStrongholdResult<SnapshotStatus> {
    Context::snapshot_status(&self.path).await
  }

  pub async fn set_password(&self, password: Password) -> IotaStrongholdResult<()> {
    Context::set_password(&self.path, password).await
  }

  pub async fn load(&self, password: Password) -> IotaStrongholdResult<()> {
    Context::load(&self.path, password).await
  }

  pub async fn unload(&self, persist: bool) -> IotaStrongholdResult<()> {
    Context::unload(&self.path, persist).await
  }

  pub async fn save(&self) -> IotaStrongholdResult<()> {
    Context::save(&self.path).await
  }
}
