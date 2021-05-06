// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example config

use identity_account::account::Account;
use identity_account::account::AccountStorage;
use identity_account::account::AutoSave;
use identity_account::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a new Account with explicit configuration
  let account: Account = Account::builder()
    .autosave(AutoSave::Never) // never auto-save. rely on the drop save
    .autosave(AutoSave::Every) // save immediately after every action
    .autosave(AutoSave::Batch(10)) // save after every 10 actions
    .dropsave(false) // save the account state on drop
    .milestone(4) // save a snapshot every 4 actions
    .storage(AccountStorage::Memory) // use the default in-memory storage adapter
    .build()
    .await?;

  println!("[Example] Account = {:#?}", account);

  Ok(())
}
