// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes a DID Document,
//! the fundamental building block for decentralized identity.
//!
//! cargo run --example create_did_document

use identity::iota::Network;
use identity::iota::TangleRef;
use identity::prelude::*;

#[tokio::main] // Using this allows us to have an async main function.
async fn main() -> Result<()> {
  // Create a DID Document (an identity).
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Create a new client connected to the Testnet (Chrysalis).
  let client: Client = Client::new().await?;

  // Use the client to publish the DID Document to the Tangle.
  document.publish(&client).await?;

  // Print the DID Document IOTA transaction link.
  let network = Network::from_did(document.id());
  let explore: String = format!("{}/message/{}", network.explorer_url(), document.message_id());
  println!("DID Document Transaction > {}", explore);

  Ok(())
}
