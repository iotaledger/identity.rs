// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_config

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::AutoSave;
use identity::account::IdentityCreate;
use identity::account::IdentityState;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::Network;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Set-up for private Tangle
  // You can use https://github.com/iotaledger/one-click-tangle for a local setup.
  // The `network_name` needs to match the id of the network or a part of it.
  // As an example we are treating the devnet as a `private-tangle`, so we use `dev`.
  // Replace this with `tangle` if you run this against a one-click private tangle.
  let network_name = "dev";
  let network = Network::try_from_name(network_name)?;
  // In a locally running one-click tangle, this would often be `http://127.0.0.1:14265/`
  let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";

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
        // Manipulate this in order to manually appoint nodes
        .node("https://chrysalis-nodes.iota.org")
        .unwrap() // unwrap is safe, we provided a valid node URL
        // Set a permanode from the same network (Important)
        .permanode("https://chrysalis-chronicle.iota.org/api/mainnet/", None, None)
        .unwrap() // unwrap is safe, we provided a valid permanode URL
    })
    // Configure a client for the private network, here `dev`
    // Also set the URL that points to the REST API of the node
    .client(network, |builder| {
      // unwrap is safe, we provided a valid node URL
      builder.node(private_node_url).unwrap()
    })
    .build()
    .await?;

  // Create an Identity specifically on the devnet by passing `network_name`
  // The same applies if we wanted to create an identity on a private tangle
  let id_create = IdentityCreate::new().network(network_name)?;

  // Create a new Identity with the network name set.
  let identity: IdentityState = match account.create_identity(id_create).await {
    Ok(identity) => identity,
    Err(err) => {
      eprintln!("[Example] Error: {:?} {}", err, err.to_string());
      eprintln!("[Example] Is your Tangle node listening on {}?", private_node_url);
      return Ok(());
    }
  };
  let iota_did: &IotaDID = identity.try_did()?;

  // Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  println!(
    "[Example] Explore the DID Document = {}/{}",
    iota_did.network()?.explorer_url().unwrap().to_string(),
    iota_did.to_string()
  );

  Ok(())
}
