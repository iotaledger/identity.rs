// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_config

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentityState;
use identity::account::AutoSave;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::Network;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Set-up for private Tangle
  // This name needs to match the id of the network or part of it.
  // The id of the one-click private tangle is `private-tangle` but we can only use 6 characters, we use recommend `tangle`.
  // As an example we are treating the devnet as a `private-tangle`, there are easier ways to change to devnet via `Network::Devnet`
  let network_name = "dev"; //Replace this with a `tangle` for the one-click private tangle.
  let network = Network::try_from_name(network_name)?;
  let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe"; //Replace this with a private tangle node

  // Create a new Account with explicit configuration
  let account: Account = Account::builder()
    .autosave(AutoSave::Never) // never auto-save. rely on the drop save
    .autosave(AutoSave::Every) // save immediately after every action
    .autosave(AutoSave::Batch(10)) // save after every 10 actions
    .dropsave(false) // save the account state on drop
    .milestone(4) // save a snapshot every 4 actions
    .storage(AccountStorage::Memory) // use the default in-memory storage adapter
    // configure a mainnet Tangle client with node and permanode
    .client(Network::Mainnet, |builder| {
      builder
        .node("https://chrysalis-nodes.iota.org") // Manipulate this in order to manually appoint nodes
        .unwrap() // unwrap is safe, we provided a valid node URL
        .permanode("https://chrysalis-chronicle.iota.org/api/mainnet/", None, None) // Set a permanode from the same network (Important)
        .unwrap() // unwrap is safe, we provided a valid permanode URL
    })
    // Configure a client for the private network, overwrites the mainnet client above
    // Also set the URL that points to the REST API of the node
    .client(network, |builder| {
      // unwrap is safe, we provided a valid node URL
      builder.node(private_node_url).unwrap()
    })
    .build()
    .await?;

    // Create an Identity specifically for the mainnet, replace with network_name variable for private Tangle
    let id_create = IdentityCreate::new().network(network_name)?;

    // Create a new Identity with the network name set.
    let identity: IdentityState = match account.create_identity(id_create).await {
      Ok(identity) => identity,
      Err(err) => {
        eprintln!("[Example] Error: {:?} {}", err, err.to_string());
        eprintln!(
          "[Example] Is your Tangle node listening on {}?",
          private_node_url
        );
        return Ok(());
      }
    };
    let iota_did: &IotaDID = identity.try_did()?;

      // Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  println!(
    "[Example] Explore the DID Document = {}",
    format!(
      "{}/{}",
      iota_did.network()?.explorer_url().unwrap().to_string(),
      iota_did.to_string()
    )
  );

  Ok(())
}
