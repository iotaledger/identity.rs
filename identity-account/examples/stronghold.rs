// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example stronghold

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::Stronghold;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a Stronghold storage instance for the account
  let storage: Stronghold = Stronghold::new("./example-strong.hodl", "my-password").await?;

  // Create a new Account with the default configuration
  let account: Account<Stronghold> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot1: IdentitySnapshot = account.create(Default::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document1: &IotaDID = snapshot1.identity().try_document()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot1);
  println!("[Example] Local Document = {:#?}", snapshot1.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve(document1).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Create another new Identity
  let snapshot2: IdentitySnapshot = account.create(Default::default()).await?;
  let document2: &IotaDID = snapshot2.identity().try_document()?;

  // Anndddd delete it
  account.delete(document2).await?;

  Ok(())
}
