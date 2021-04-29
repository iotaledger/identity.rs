// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::MemStore;
use identity_iota::did::Document;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let storage: MemStore = MemStore::new();
  let account: Account<MemStore> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create(Default::default()).await?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list().await);

  // Fetch the DID Document from the Tangle
  let resolved: Document = account.resolve(snapshot.id()).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // // Delete the identity
  // account.delete(snapshot.id()).await?;

  Ok(())
}
