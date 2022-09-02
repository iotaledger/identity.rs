// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_manipulate

use std::path::PathBuf;

use identity_account_legacy::account::Account;
use identity_account_legacy::types::IdentitySetup;
use identity_account_legacy::types::MethodContent;
use identity_account_legacy::Result;
use identity_account_storage_legacy::stronghold::Stronghold;
use identity_iota::core::Url;
use identity_iota::did::MethodRelationship;
use identity_iota_client_legacy::tangle::ExplorerUrl;
use identity_iota_core_legacy::did::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // ===========================================================================
  // Create Identity - Similar to create_did example
  // ===========================================================================

  // Stronghold settings
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".to_owned();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;

  // Create a new Account with the default configuration
  let mut account: Account = Account::builder()
    .storage(stronghold)
    .create_identity(IdentitySetup::default())
    .await?;

  // ===========================================================================
  // Identity Manipulation
  // ===========================================================================

  // Add another Ed25519 verification method to the identity
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("my-next-key")
    .apply()
    .await?;

  // Associate the newly created method with additional verification relationships
  account
    .update_identity()
    .attach_method_relationship()
    .fragment("my-next-key")
    .relationships(vec![
      MethodRelationship::CapabilityDelegation,
      MethodRelationship::CapabilityInvocation,
    ])
    .apply()
    .await?;

  // Add a new service to the identity.
  account
    .update_identity()
    .create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .apply()
    .await?;

  // Remove the Ed25519 verification method
  account
    .update_identity()
    .delete_method()
    .fragment("my-next-key")
    .apply()
    .await?;

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
