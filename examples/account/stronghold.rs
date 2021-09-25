// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_stronghold

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentityState;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

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
  let identity1: IdentityState = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let did1: &IotaDID = identity1.try_did()?;

  println!("[Example] Local Document = {:#?}", identity1.to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(did1).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Create another new Identity
  let identity2: IdentityState = account.create_identity(IdentityCreate::default()).await?;
  let did2: &IotaDID = identity2.try_did()?;

  // Anndddd delete it
  account.delete_identity(did2).await?;

  Ok(())
}
