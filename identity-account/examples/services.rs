// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example services

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::identity::IdentityCreate;
use identity_account::identity::IdentitySnapshot;
use identity_core::common::Url;
use identity_iota::did::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a new Account with the default configuration
  let account: Account = Account::builder().build().await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document: &IotaDID = snapshot.identity().try_did()?;

  let command: Command = Command::create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .finish()?;

  // Process the command and update the identity state.
  account.update_identity(document, command).await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (1) = {:#?}",
    account.resolve_identity(document).await?
  );

  Ok(())
}
