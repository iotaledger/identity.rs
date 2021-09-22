// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_methods

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::did::MethodScope;
use identity::iota::IotaDID;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let (account, iota_did): (Account, IotaDID) = create_did::create_identity().await?;

  // Add a new Ed25519 (default) verification method to the identity - the
  // verification method is included as an embedded authentication method.
  account
    .update_identity(did)
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
  account
    .update_identity(did)
    .create_method()
    .fragment("my-next-key")
    .apply()
    .await?;

  // Associate the newly created method with additional verification relationships
  account
    .update_identity(did)
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
  account
    .update_identity(did)
    .delete_method()
    .fragment("my-auth-key")
    .apply()
    .await?;

  // Fetch and log the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  println!(
    "[Example] Tangle Document (3) = {:#?}",
    account.resolve_identity(did).await?
  );

  Ok(())
}
