// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_config

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::AutoSave;
use identity::account::Result;
use identity::iota::Network;

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
    // configure the mainnet Tangle client
    .client(Network::Mainnet, |scope| {
      scope
        .node("https://chrysalis-nodes.iota.org")
        .unwrap() // unwrap is safe, we provided a valid node URL
        .permanode("https://chrysalis-chronicle.iota.org/api/mainnet/", None, None)
        .unwrap() // unwrap is safe, we provided a valid permanode URL
    })
    .build()
    .await?;

  println!("[Example] Account = {:#?}", account);

  Ok(())
}
