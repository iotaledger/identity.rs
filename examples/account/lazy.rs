// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_lazy
use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentityState;
use identity::account::Result;
use identity::core::Url;
use identity::iota::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Stronghold settings
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new Account with auto publishing set to false.
  // This means updates are not pushed to the tangle automatically.
  // Rather, when we publish, multiple updates are batched together.
  let account: Account = Account::builder()
    .storage(AccountStorage::Stronghold(stronghold_path, Some(password)))
    .autopublish(false)
    .build()
    .await?;

  // Create a new Identity with default settings.
  // The identity will only be written to the local storage - not published to the tangle.
  let identity: IdentityState = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let iota_did: &IotaDID = identity.try_did()?;

  account
    .update_identity(iota_did)
    .create_service()
    .fragment("example-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Publish the newly created DID document,
  // including the new service, to the tangle.
  account.publish_updates(iota_did).await?;

  // Add another service.
  account
    .update_identity(iota_did)
    .create_service()
    .fragment("another-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Delete the previously added service.
  account
    .update_identity(iota_did)
    .delete_service()
    .fragment("example-service")
    .apply()
    .await?;

  // Publish the updates as one message to the tangle.
  account.publish_updates(iota_did).await?;

  // Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  println!(
    "[Example] Explore the DID Document = {}/{}",
    iota_did.network()?.explorer_url().unwrap().to_string(),
    iota_did.to_string()
  );

  Ok(())
}
