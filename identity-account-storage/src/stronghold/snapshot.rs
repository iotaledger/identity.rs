// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::stronghold::Context;
use crate::stronghold::IotaStrongholdResult;
use crate::stronghold::Password;
use crate::stronghold::SnapshotStatus;
use crate::stronghold::Store;
use crate::stronghold::Vault;

use super::ClientPath;
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

  pub fn vault(&self, client_path: ClientPath) -> Vault<'_> {
    Vault::new(&self.path, client_path, &[])
  }

  pub fn store(&self, client_path: ClientPath) -> Store<'_> {
    Store::new(&self.path, client_path, &[])
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
