// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use engine::vault::RecordId;
use iota_stronghold::procedures::Procedure;
use iota_stronghold::procedures::StrongholdProcedure;
use iota_stronghold::Location;
use iota_stronghold::RecordHint;
use iota_stronghold::StrongholdFlags;
use iota_stronghold::VaultFlags;
use std::path::Path;

use crate::stronghold_old::Context;
use crate::stronghold_old::IotaStrongholdResult;

use super::ClientPath;

pub type Record = (RecordId, RecordHint);

#[derive(Debug)]
pub struct Vault<'snapshot> {
  path: &'snapshot Path,
  client_path: ClientPath,
  flags: Vec<StrongholdFlags>,
}

impl<'snapshot> Vault<'snapshot> {
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

impl Vault<'_> {
  /// Returns the snapshot path of the vault.
  pub fn path(&self) -> &Path {
    self.path
  }

  /// Returns the client path of the vault.
  pub fn client_path(&self) -> &[u8] {
    self.client_path.as_ref()
  }

  /// Returns the vault policy options.
  pub fn flags(&self) -> &[StrongholdFlags] {
    &self.flags
  }

  /// Inserts a record.
  pub async fn insert<T>(
    &self,
    location: Location,
    payload: T,
    hint: RecordHint,
    flags: &[VaultFlags],
  ) -> IotaStrongholdResult<()>
  where
    T: Into<Vec<u8>>,
  {
    Context::scope(self.path, self.client_path(), &self.flags)
      .await?
      .write_to_vault(location, payload.into(), hint, flags.to_vec())
      .await??;
    Ok(())
  }

  /// Deletes a record.
  pub async fn delete(&self, location: Location, gc: bool) -> IotaStrongholdResult<()> {
    Context::scope(self.path, self.client_path(), &self.flags)
      .await?
      .delete_data(location, gc)
      .await??;
    Ok(())
  }

  /// Returns true if the specified vault record exists.
  pub async fn exists(&self, location: Location) -> IotaStrongholdResult<bool> {
    let scope: _ = Context::scope(self.path, self.client_path(), &self.flags).await?;
    Ok(scope.record_exists(location).await?)
  }

  /// Runs the Stronghold garbage collector.
  pub async fn garbage_collect(&self, vault: &[u8]) -> IotaStrongholdResult<bool> {
    Ok(
      Context::scope(self.path, self.client_path(), &self.flags)
        .await?
        .garbage_collect(vault.to_vec())
        .await?,
    )
  }

  /// Executes a runtime [`procedure`][`Procedure`].
  pub async fn execute<P>(&self, procedure: P) -> IotaStrongholdResult<P::Output>
  where
    P: Procedure + Into<StrongholdProcedure>,
  {
    let result: <P as Procedure>::Output = Context::scope(self.path, self.client_path(), &self.flags)
      .await?
      .runtime_exec(procedure)
      .await??;
    Ok(result)
  }

  /// Returns a list of available records and hints.
  pub async fn records<T>(&self, vault: &T) -> IotaStrongholdResult<Vec<(RecordId, RecordHint)>>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    let scope: _ = Context::scope(self.path, self.client_path(), &self.flags).await?;
    Ok(scope.list_hints_and_ids(vault.as_ref()).await?)
  }
}
