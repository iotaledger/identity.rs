// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Network;
use identity::iota::TangleRef;
use identity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new client connected to the Testnet (Chrysalis).
  let client: Client = Client::new().await?;

  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the client to publish the DID Document to the Tangle.
  document.publish(&client).await?;

  let network: Network = document.id().into();
  let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());

  println!("DID Document Transaction > {}", explore);

  Ok(())
}
