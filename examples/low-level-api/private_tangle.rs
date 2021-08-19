// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes a DID Document
//! to a private tangle.
//! It can be run together with a local hornet node.
//! Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
//! for setup instructions.
//!
//! cargo run --example private_tangle

use identity::iota::ClientBuilder;
use identity::iota::ClientMap;
use identity::iota::Network;
use identity::iota::Receipt;
use identity::prelude::*;

#[tokio::main]
pub async fn main() -> Result<()> {
  // This is the `networkID` defined in hornet's node configuration.
  let network = Network::from_name("private-tangle")?;

  // Set the network and the URL that points to
  // the REST API of the locally running hornet node.
  let client = ClientBuilder::new()
    .network(network.clone())
    .node("http://127.0.0.1:14265/")?
    .build()
    .await?;

  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::from_client(client);

  // Generate a new ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;

  // Create a DID with the explicitly set network.
  // This will result in a DID prefixed by `did:iota:private-tangle`.
  let mut document: IotaDocument = IotaDocument::from_keypair_with_network(&keypair, &network.as_str())?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  println!("DID Document JSON > {:#}", document);

  // Publish the DID Document to the Tangle.
  let receipt: Receipt = client.publish_document(&document).await?;

  println!("Publish Receipt > {:#?}", receipt);

  Ok(())
}
