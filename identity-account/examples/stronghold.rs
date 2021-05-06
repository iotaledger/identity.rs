// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example stronghold

use std::path::PathBuf;

use identity_account::account::Account;
use identity_account::account::AccountStorage;
use identity_account::error::Result;
use identity_account::identity::IdentityCreate;
use identity_account::identity::IdentitySnapshot;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let snapshot: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new Account with Stronghold as the storage adapter
  let account: Account = Account::builder()
    .storage(AccountStorage::Stronghold(snapshot, Some(password)))
    .build()
    .await?;

  // Create a new Identity with default settings
  let snapshot1: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document1: &IotaDID = snapshot1.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot1);
  println!("[Example] Local Document = {:#?}", snapshot1.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document1).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Create another new Identity
  let snapshot2: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;
  let document2: &IotaDID = snapshot2.identity().try_did()?;

  // Anndddd delete it
  account.delete_identity(document2).await?;

  Ok(())
}
