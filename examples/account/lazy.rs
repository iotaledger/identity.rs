// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_lazy
use std::path::PathBuf;

use identity::account::Account;
use identity::account::IdentitySetup;
use identity::account::Result;
use identity::account_storage::Stronghold;
use identity::core::Url;
use identity::iota::ExplorerUrl;
use identity::iota_core::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Stronghold settings
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, Some(password), None).await?;

  // Create a new Account with auto publishing set to false.
  // This means updates are not pushed to the tangle automatically.
  // Rather, when we publish, multiple updates are batched together.
  let mut account: Account = Account::builder()
    .storage(stronghold)
    .autopublish(false)
    .create_identity(IdentitySetup::default())
    .await?;

  // Add a new service to the local DID document.
  account
    .update_identity()
    .create_service()
    .fragment("example-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Publish the newly created DID document,
  // including the new service, to the tangle.
  account.publish().await?;

  // Add another service.
  account
    .update_identity()
    .create_service()
    .fragment("another-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Delete the previously added service.
  account
    .update_identity()
    .delete_service()
    .fragment("example-service")
    .apply()
    .await?;

  // Publish the updates as one message to the tangle.
  account.publish().await?;

  // Retrieve the DID from the newly created identity.
  let iota_did: &IotaDID = account.did();

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(iota_did)?
  );

  Ok(())
}
