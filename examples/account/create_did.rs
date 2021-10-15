// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_create

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::Result;
use identity::iota::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Sets the location and password for the Stronghold
  //
  // Stronghold is an encrypted file that manages private keys.
  // It implements best practices for security and is the recommended way of handling private keys.
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new `AccountBuilder` that can produce any number of accounts.
  // Multiple accounts can be built from it, and will share the storage and config.
  let builder: AccountBuilder = Account::builder()
    .storage(AccountStorage::Stronghold(stronghold_path, Some(password)))
    .await?;

  // Create a new Identity with default settings
  //
  // This step generates a keypair, creates an identity and publishes it to the IOTA mainnet.
  let account: Account = builder.create_identity(IdentityCreate::default()).await?;

  // Retrieve the did of the newly created identity.
  let iota_did: &IotaDID = account.did();

  // Print the local state of the DID Document
  println!(
    "[Example] Local Document from {} = {:#?}",
    iota_did,
    account.state().await?.to_document()
  );

  // Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  println!(
    "[Example] Explore the DID Document = {}/{}",
    iota_did.network()?.explorer_url().unwrap().to_string(),
    iota_did.to_string()
  );
  Ok(())
}
