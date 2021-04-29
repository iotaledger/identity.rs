// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example methods

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::MemStore;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let storage: MemStore = MemStore::new();
  let account: Account<MemStore> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create(Default::default()).await?;

  // Add a new Ed25519 verification method to the identity - the verification
  // method is included as an embedded authentication method.
  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .scope(MethodScope::Authentication)
    .fragment("my-auth-key")
    .finish()?;

  // Process the command and update the identity state.
  account.update(snapshot.id(), command).await?;

  // Fetch and log the DID Document from the Tangle
  println!(
    "[Example] Tangle Document (1) = {:#?}",
    account.resolve(snapshot.id()).await?
  );

  // Add another Ed25519 verification method to the identity
  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .scope(MethodScope::VerificationMethod)
    .fragment("my-next-key")
    .finish()?;

  // Process the command and update the identity state.
  account.update(snapshot.id(), command).await?;

  // Associate the newly created method with additional verification relationships
  let command: Command = Command::attach_method()
    .fragment("my-next-key")
    .scopes(vec![
      MethodScope::CapabilityDelegation,
      MethodScope::CapabilityInvocation,
    ])
    .finish()?;

  // Process the command and update the identity state.
  account.update(snapshot.id(), command).await?;

  // Fetch and log the DID Document from the Tangle
  println!(
    "[Example] Tangle Document (2) = {:#?}",
    account.resolve(snapshot.id()).await?
  );

  // Remove the original Ed25519 verification method
  let command: Command = Command::delete_method().fragment("my-auth-key").finish()?;

  // Process the command and update the identity state.
  account.update(snapshot.id(), command).await?;

  // Fetch and log the DID Document from the Tangle
  println!(
    "[Example] Tangle Document (3) = {:#?}",
    account.resolve(snapshot.id()).await?
  );

  Ok(())
}
