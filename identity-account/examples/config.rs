// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example config

use identity_account::account::Account;
use identity_account::account::AutoSave;
use identity_account::account::Config;
use identity_account::error::Result;
use identity_account::storage::MemStore;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let config: Config = Config::new()
    .autosave(AutoSave::Never) // never auto-save. rely on the drop save
    .autosave(AutoSave::Every) // save immediately after every action
    .autosave(AutoSave::Batch(10)) // save after every 10 actions
    .dropsave(false) // save the account state on drop
    .milestone(4); // save a snapshot every 4 actions

  // Create an in-memory storage instance for the account
  let storage: MemStore = MemStore::new();

  // Create a new Account with explicit configuration
  let account: Account<MemStore> = Account::with_config(storage, config).await?;

  println!("[Example] Account = {:#?}", account);

  Ok(())
}
