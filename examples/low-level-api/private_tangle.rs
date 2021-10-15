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
use identity::iota::IotaDID;
use identity::iota::Network;
use identity::iota::Receipt;
use identity::iota::TangleRef;
use identity::prelude::*;

#[tokio::main]
pub async fn main() -> Result<()> {
  // This name needs to match the id of the network or part of it.
  // Since the id of the one-click private tangle is `private-tangle`
  // but we can only use 6 characters, we use just `tangle`.
  // As an example we are treating the devnet as a `private-tangle`,
  // there are easier ways to change to devnet via `Network::Devnet`
  let network = Network::try_from_name("dev")?;

  // Set the network and the URL that points to
  // the REST API of the node.
  // In a locally running private tangle, this would often be `http://127.0.0.1:14265/`
  let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";
  let client = ClientBuilder::new()
    .network(network)
    .node(private_node_url)?
    .build()
    .await?;

  // Generate a new Ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;

  // Create a DID with the network set explicitly.
  // This will result in a DID prefixed by `did:iota:tangle`.
  let mut document: IotaDocument = IotaDocument::new_with_options(&keypair, Some(client.network().name()), None)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.private())?;

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

  // Prints the Identity Resolver Explorer URL, the entire history can be observed on this page by "Loading History".
  let iota_did: &IotaDID = document.did();
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
