// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates, publishes and deletes a DID Document
//! to and from a private tangle.
//! It can be run together with a local hornet node.
//! Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
//! for setup instructions.
//!
//! cargo run --example account_private_tangle

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::Network;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // This name needs to match the id of the network or part of it.
  // Since the id of the one-click private tangle is `private-tangle`
  // but we can only use 6 characters, we use just `tangle`.
  let network_name = "tangle";
  let network = Network::try_from_name(network_name)?;

  // Create a new Account with a custom client configuration.
  let private_node_url = "http://127.0.0.1:14265/";
  let account: Account = Account::builder()
    // Configure a client for the private network.
    // Also set the URL that points to the REST API
    // of the locally running hornet node.
    .client(network, |builder| {
      // unwrap is safe, we provided a valid node URL
      builder.node(private_node_url).unwrap()
    })
    .build()
    .await?;

  let id_create = IdentityCreate::new().network(network_name);

  // Create a new Identity with the network name set.
  let snapshot: IdentitySnapshot = match account.create_identity(id_create).await {
    Ok(snapshot) => snapshot,
    Err(err) => {
      eprintln!("[Example] Error: {:?} {}", err, err.to_string());
      eprintln!(
        "[Example] Is your private Tangle node listening on {}?",
        private_node_url
      );
      return Ok(());
    }
  };

  // Retrieve the DID from the newly created Identity state.
  let did: &IotaDID = snapshot.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(did).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Delete the identity and all associated keys
  account.delete_identity(did).await?;

  Ok(())
}
