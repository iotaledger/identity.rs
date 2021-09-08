// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_methods

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::IdentityUpdater;
use identity::account::Result;
use identity::did::MethodScope;
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

  // Get the updater for the given `did`, so we can run multiple updates on it.
  let did_updater: IdentityUpdater<'_, _> = account.update_identity(did);

  // Add a new Ed25519 (default) verification method to the identity - the
  // verification method is included as an embedded authentication method.
  did_updater
    .create_method()
    .scope(MethodScope::Authentication)
    .fragment("my-auth-key")
    .apply()
    .await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (1) = {:#?}",
    account.resolve_identity(did).await?
  );

  // Add another Ed25519 verification method to the identity
  did_updater.create_method().fragment("my-next-key").apply().await?;

  // Associate the newly created method with additional verification relationships
  did_updater
    .attach_method()
    .fragment("my-next-key")
    .scope(MethodScope::CapabilityDelegation)
    .scope(MethodScope::CapabilityInvocation)
    .apply()
    .await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (2) = {:#?}",
    account.resolve_identity(did).await?
  );

  // Remove the original Ed25519 verification method
  did_updater.delete_method().fragment("my-auth-key").apply().await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (3) = {:#?}",
    account.resolve_identity(did).await?
  );

  Ok(())
}
