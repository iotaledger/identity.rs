// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_multiple

use std::path::PathBuf;

use identity_iota::account::Account;
use identity_iota::account::AccountBuilder;
use identity_iota::account::IdentitySetup;
use identity_iota::account::MethodContent;
use identity_iota::account::Result;
use identity_iota::account_storage::Stronghold;
use identity_iota::client::ExplorerUrl;
use identity_iota::iota_core::IotaDID;

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

  // Create an AccountBuilder to make it easier to create multiple identities.
  // Every account created from the builder will use the same storage - stronghold in this case.
  let mut builder: AccountBuilder = Account::builder().storage(stronghold);

  // The creation step generates a keypair, builds an identity
  // and publishes it to the IOTA mainnet.
  let account1: Account = builder.create_identity(IdentitySetup::default()).await?;

  // Create a second identity which uses the same storage.
  let mut account2: Account = builder.create_identity(IdentitySetup::default()).await?;

  // Retrieve the did of the identity that account1 manages.
  let iota_did1: IotaDID = account1.did().to_owned();

  // Suppose we're done with account1 and drop it.
  std::mem::drop(account1);

  // Now we want to modify the iota_did1 identity - how do we do that?
  // We can load the identity from storage into an account using the builder.
  let mut account1: Account = builder.load_identity(iota_did1).await?;

  let account1_did: IotaDID = account1.did().clone();

  // Now we can make modifications to the identity.
  // We can even do so concurrently by spawning tasks.
  let task1 = tokio::spawn(async move {
    account1
      .update_identity()
      .create_method()
      .content(MethodContent::GenerateEd25519)
      .fragment("my-key")
      .apply()
      .await
  });

  let task2 = tokio::spawn(async move {
    account2
      .update_identity()
      .create_method()
      .content(MethodContent::GenerateX25519)
      .fragment("my-other-key")
      .apply()
      .await
  });

  task1.await.expect("task1 failed to execute to completion")?;
  task2.await.expect("task2 failed to execute to completion")?;

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(&account1_did)?
  );

  Ok(())
}
