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
use identity::iota::Network;
use identity::iota::Receipt;
use identity::prelude::*;

#[tokio::main]
pub async fn main() -> Result<()> {
  // This name needs to match the id of the network or part of it.
  // Since the id of the one-click private tangle is `private-tangle`
  // but we can only use 6 characters, we use just `tangle`.
  let network_name = "tangle";
  let network = Network::try_from_name(network_name)?;

  // Set the network and the URL that points to
  // the REST API of the locally running hornet node.
  let private_node_url = "http://127.0.0.1:14265/";
  let client = ClientBuilder::new()
    .network(network)
    .node(private_node_url)?
    .build()
    .await?;

  // Generate a new ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;

  // Create a DID with the network set explicitly.
  // This will result in a DID prefixed by `did:iota:tangle`.
  let mut document: IotaDocument = IotaDocument::from_keypair_with_network(&keypair, network_name)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.private())?;

  println!("DID Document JSON > {:#}", document);

  // Publish the DID Document to the Tangle.
  let receipt: Receipt = match client.publish_document(&document).await {
    Ok(receipt) => receipt,
    Err(err) => {
      eprintln!("Error > {:?} {}", err, err.to_string());
      eprintln!("Is your private Tangle node listening on {}?", private_node_url);
      return Ok(());
    }
  };

  println!("Publish Receipt > {:#?}", receipt);

  Ok(())
}
