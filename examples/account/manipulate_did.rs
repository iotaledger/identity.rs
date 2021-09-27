// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_manipulate

use identity::account::Account;
use identity::account::Result;
use identity::did::MethodScope;
use identity::iota::IotaDID;
use identity::core::Url;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  //Calls the create_identity function from the create_did example - this is not part of the framework, but rather reusing the previous example.
  let (account, iota_did): (Account, IotaDID) = create_did::run().await?;

  // Add another Ed25519 verification method to the identity
  account
    .update_identity(&iota_did)
    .create_method()
    .fragment("my-next-key")
    .apply()
    .await?;

  // Associate the newly created method with additional verification relationships
  account
    .update_identity(&iota_did)
    .attach_method()
    .fragment("my-next-key")
    .scope(MethodScope::CapabilityDelegation)
    .scope(MethodScope::CapabilityInvocation)
    .apply()
    .await?;

  // Add a new service to the identity.
    account
    .update_identity(&iota_did)
    .create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .apply()
    .await?;

  // Remove the Ed25519 verification method
  account
    .update_identity(&iota_did)
    .delete_method()
    .fragment("my-next-key")
    .apply()
    .await?;

  //Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  println!("[Example] Explore the DID Document = {}", format!("{}/{}", iota_did.network()?.explorer_url().unwrap().to_string(), iota_did.to_string()));
  Ok(())
}
