// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_unchecked

use std::path::PathBuf;

use identity_iota::account::Account;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::account_storage::Stronghold;
use identity_iota::client::ExplorerUrl;
use identity_iota::core::Timestamp;
use identity_iota::iota_core::IotaDID;
use identity_iota::prelude::IotaDocument;

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
  let mut account: Account = Account::builder()
    .storage(stronghold)
    .create_identity(IdentitySetup::default())
    .await?;

  // Get a copy of the document this account manages.
  // We will apply updates to the document, and overwrite the account's current document.
  let mut document: IotaDocument = account.document().clone();

  // Add a custom property to the document.
  document
    .properties_mut()
    .insert("myCustomPropertyKey".into(), "value".into());

  // Override the updated field timestamp to 24 hours (= 86400 seconds) in the future,
  // because we can. This is usually set automatically by Account::update_identity.
  let timestamp: Timestamp = Timestamp::from_unix(Timestamp::now_utc().to_unix() + 86400)?;
  document.metadata.updated = Some(timestamp);

  // Update the identity without validation and publish the result to the Tangle
  // (depending on the account's autopublish setting).
  // The responsibility is on the caller to provide a valid document which the account
  // can continue to use. Failing to do so can corrupt the identity; use with caution!
  account.update_document_unchecked(document).await?;

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
