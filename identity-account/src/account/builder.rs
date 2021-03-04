// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use crate::account::Account;
use crate::account::AccountStorage;
use crate::error::Error;
use crate::error::Result;
use crate::storage::StorageHandle;
use crate::storage::StrongholdAdapter;
use crate::storage::VaultAdapter;
use crate::utils::derive_encryption_key;
use crate::utils::fs;
use crate::utils::EncryptionKey;

const STORAGE: Option<AccountStorage> = Some(AccountStorage::Stronghold);
const STORAGE_PATH: &str = "./storage";

#[derive(Debug)]
pub struct AccountBuilder {
  storage: Option<AccountStorage>,
  storage_path: PathBuf,
  storage_password: Option<EncryptionKey>,
}

impl AccountBuilder {
  pub fn new() -> Self {
    Self {
      storage: STORAGE,
      storage_path: STORAGE_PATH.into(),
      storage_password: None,
    }
  }

  pub fn storage<'a, P>(mut self, storage: AccountStorage, password: P) -> Self
  where
    P: Into<Option<&'a str>>,
  {
    self.storage = storage.into();
    self.storage_password = password.into().map(derive_encryption_key);
    self
  }

  pub fn storage_path<P>(mut self, storage_path: &P) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    self.storage_path = storage_path.as_ref().into();
    self
  }

  pub async fn build(self) -> Result<Account> {
    let adapter: Box<dyn VaultAdapter> = match self.storage {
      Some(AccountStorage::Stronghold) => {
        let path: PathBuf = fs::database_file(&self.storage_path, "identity.vault");

        fs::ensure_directory(&path)?;

        Box::new(StrongholdAdapter::new(&path, self.storage_password).await?)
      }
      Some(AccountStorage::Custom(adapter)) => {
        if let Some(password) = self.storage_password {
          adapter.set_password(password).await?;
        }

        adapter
      }
      None => {
        return Err(Error::MissingStorageAdapter);
      }
    };

    let storage: StorageHandle = StorageHandle::new(adapter);
    let account: Account = Account::new(storage);

    account.initialize().await?;

    Ok(account)
  }
}
