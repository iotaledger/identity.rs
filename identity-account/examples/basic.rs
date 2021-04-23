// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::storage::MemStore;
use identity_account::types::ChainId;
use identity_account::types::IdentityConfig;
use identity_iota::chain::DocumentChain;
use identity_iota::did::Document;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let storage: MemStore = MemStore::import_or_default("example-basic.json");
  let account: Account<_> = Account::new(storage).await?;

  // Create a new Identity chain
  let chain: ChainId = account.create(IdentityConfig::new()).await?;
  let document: Document = account.get(chain).await?;

  println!("[Account] Document = {:#?}", document);

  // Fetch the DID Document from the Tangle
  let resolved: DocumentChain = account.resolve(document.id()).await?;

  println!("[Tangle] Document = {:#?}", resolved.current());

  // Export the current state of the account
  account.store().export("example-basic.json")?;

  Ok(())
}
