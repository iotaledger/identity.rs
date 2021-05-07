// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use zeroize::Zeroize;

use crate::account::Account;
use crate::account::AutoSave;
use crate::account::Config;
use crate::error::Result;
use crate::storage::MemStore;
use crate::storage::Storage;
use crate::storage::Stronghold;

/// The storage adapter used by an [Account].
#[derive(Debug)]
pub enum AccountStorage {
  Memory,
  Stronghold(PathBuf, Option<String>),
  Custom(Box<dyn Storage>),
}

/// An [Account] builder for easier account configuration.
#[derive(Debug)]
pub struct AccountBuilder {
  config: Config,
  storage: AccountStorage,
}

impl AccountBuilder {
  /// Creates a new `AccountBuilder`.
  pub fn new() -> Self {
    Self {
      config: Config::new(),
      storage: AccountStorage::Memory,
    }
  }

  /// Sets the account auto-save behaviour.
  pub fn autosave(mut self, value: AutoSave) -> Self {
    self.config = self.config.autosave(value);
    self
  }

  /// Save the account state on drop.
  pub fn dropsave(mut self, value: bool) -> Self {
    self.config = self.config.dropsave(value);
    self
  }

  /// Save a state snapshot every N actions.
  pub fn milestone(mut self, value: u32) -> Self {
    self.config = self.config.milestone(value);
    self
  }

  /// Sets the account storage adapter.
  pub fn storage(mut self, value: AccountStorage) -> Self {
    self.storage = value;
    self
  }

  /// Creates a new [Account] based on the builder configuration.
  pub async fn build(self) -> Result<Account> {
    match self.storage {
      AccountStorage::Memory => Account::with_config(MemStore::new(), self.config).await,
      AccountStorage::Stronghold(snapshot, password) => {
        let passref: Option<&str> = password.as_deref();
        let adapter: Stronghold = Stronghold::new(&snapshot, passref).await?;

        if let Some(mut password) = password {
          password.zeroize();
        }

        Account::with_config(adapter, self.config).await
      }
      AccountStorage::Custom(adapter) => Account::with_config(adapter, self.config).await,
    }
  }
}

impl Default for AccountBuilder {
  fn default() -> Self {
    Self::new()
  }
}
