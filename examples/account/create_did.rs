// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_create

use std::path::PathBuf;

use identity::account::Account;
use identity::account::IdentitySetup;
use identity::account::Result;
use identity::account_storage::Stronghold;
use identity::iota::ExplorerUrl;
use identity::iota_core::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Sets the location and password for the Stronghold
  //
  // Stronghold is an encrypted file that manages private keys.
  // It implements best practices for security and is the recommended way of handling private keys.
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".to_owned();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;

  // Create a new identity using Stronghold as local storage.
  //
  // The creation step generates a keypair, builds an identity
  // and publishes it to the IOTA mainnet.
  let account: Account = Account::builder()
    .storage(stronghold)
    .create_identity(IdentitySetup::default())
    .await?;

  // Retrieve the did of the newly created identity.
  let iota_did: &IotaDID = account.did();

  // Print the local state of the DID Document
  println!("[Example] Local Document from {} = {:#?}", iota_did, account.document());

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(iota_did)?
  );

  Ok(())
}
