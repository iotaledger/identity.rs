// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_services

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::core::Url;
use identity::iota::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a new Account with the default configuration
  let account: Account = Account::builder().build().await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let did: &IotaDID = snapshot.identity().try_did()?;

  // Add a new service to the identity.
  account
    .update_identity(did)
    .create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .apply()
    .await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (1) = {:#?}",
    account.resolve_identity(did).await?
  );

  Ok(())
}
