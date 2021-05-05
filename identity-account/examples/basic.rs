// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::identity::IdentityCreate;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::MemStore;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create an in-memory storage instance for the account
  let storage: MemStore = MemStore::new();

  // Create a new Account with the default configuration
  let account: Account<MemStore> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document: &IotaDID = snapshot.identity().try_document()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Delete the identity and all associated keys
  account.delete_identity(document).await?;

  Ok(())
}
