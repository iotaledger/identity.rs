// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_basic

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::iota::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Sets the location and password for the Stronghold
  //
  // Stronghold is an encrypted file that manages private keys.
  // It implements all security recommendation and is the recommended way of handling private keys.
  let snapshot: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new Account with the default configuration
  let account: Account = Account::builder()
    .storage(AccountStorage::Stronghold(snapshot, Some(password)))
    .build()
    .await?;

  // Create a new Identity with default settings
  //
  // This step generates a keypair, creates an identity and publishes it too the IOTA mainnet.
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let did: &IotaDID = snapshot.identity().try_did()?;

  // Print the local state of the DID Document
  println!("[Example] Local Document from {} = {:#?}", did, snapshot.identity().to_document()?);
  println!("[Example] Explore the DID Document = {}", format!("{}/{}",did.network()?.explorer_url().unwrap().to_string(), did));
  Ok(())
}
