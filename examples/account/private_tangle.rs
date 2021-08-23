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

  // This is an arbitrarily defined network name
  let network_name = "atoi";

  // Unwrap is fine since we provided a non-empty string.
  let network = Network::from_name(network_name)?;

  // Create a new Account with the default configuration
  let account: Account = Account::builder()
    // Configure a client for the private network.
    // Also set the URL that points to the REST API
    // of the locally running hornet node.
    .client(network, |builder| {
      // unwrap is safe, we provided a valid node URL
      builder.node("http://127.0.0.1:14265/").unwrap()
    })
    .build()
    .await?;

  let id_create = IdentityCreate::new().network(network_name);

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(id_create).await?;

  // Retrieve the DID from the newly created Identity state.
  let document: &IotaDID = snapshot.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Delete the identity and all associated keys
  account.delete_identity(document).await?;

  Ok(())
}
