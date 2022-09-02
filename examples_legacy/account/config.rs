// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_config

use identity_account::account::Account;
use identity_account::account::AccountBuilder;
use identity_account::account::AutoSave;
use identity_account::types::IdentitySetup;
use identity_account::Result;
use identity_account_storage::storage::MemStore;
use identity_iota_client_legacy::tangle::ClientBuilder;
use identity_iota_client_legacy::tangle::ExplorerUrl;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::tangle::Network;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Set-up for a private Tangle
  // You can use https://github.com/iotaledger/one-click-tangle for a local setup.
  // The `network_name` needs to match the id of the network or a part of it.
  // As an example we are treating the devnet as a private tangle, so we use `dev`.
  // When running the local setup, we can use `tangle` since the id of the one-click
  // private tangle is `private-tangle`, but we can only use 6 characters.
  // Keep in mind, there are easier ways to change to devnet via `Network::Devnet`
  let network_name = "dev";
  let network = Network::try_from_name(network_name)?;

  // If you deployed an explorer locally this would usually be `http://127.0.0.1:8082`
  let explorer = ExplorerUrl::parse("https://explorer.iota.org/devnet")?;

  // In a locally running one-click tangle, this would usually be `http://127.0.0.1:14265`
  let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";

  // Create a new Account with explicit configuration
  let mut builder: AccountBuilder = Account::builder()
    .autosave(AutoSave::Never) // never auto-save. rely on the drop save
    .autosave(AutoSave::Every) // save immediately after every action
    .autosave(AutoSave::Batch(10)) // save after every 10 actions
    .autopublish(true) // publish to the tangle automatically on every update
    .storage(MemStore::new()) // use the default in-memory storage
    .client_builder(
      // Configure a client for the private network
      ClientBuilder::new()
        .network(network.clone())
        .primary_node(private_node_url, None, None)?,
      // set a permanode for the same network
      // .permanode(<permanode_url>, None, None)?
    );

  // Create an identity and publish it.
  // The created DID will use the network name configured for the client.
  let identity: Account = match builder.create_identity(IdentitySetup::default()).await {
    Ok(identity) => identity,
    Err(err) => {
      eprintln!("[Example] Error: {:?}", err);
      eprintln!("[Example] Is your Tangle node listening on {}?", private_node_url);
      return Ok(());
    }
  };

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let iota_did: &IotaDID = identity.did();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(iota_did)?
  );

  Ok(())
}
