// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes a DID Document,
//! the fundamental building block for decentralized identity.
//!
//! cargo run --example create_did

use identity::iota::ClientMap;
use identity::iota::ExplorerUrl;
use identity::iota::Receipt;
use identity::iota::TangleRef;
use identity::prelude::*;

pub async fn run() -> Result<(IotaDocument, KeyPair, Receipt)> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Generate a new Ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;

  // Create a DID Document (an identity) from the generated key pair.
  let mut document: IotaDocument = IotaDocument::new(&keypair)?;

  // Sign the DID Document with the default signing method.
  document.sign_self(keypair.private(), &document.default_signing_method()?.id())?;

  println!("DID Document JSON > {:#}", document);

  // Publish the DID Document to the Tangle.
  let receipt: Receipt = client.publish_document(&document).await?;
  document.set_message_id(*receipt.message_id());

  println!("Publish Receipt > {:#?}", receipt);

  // Display the web explorer url that shows the published message.
  let explorer: &ExplorerUrl = ExplorerUrl::main();
  println!(
    "DID Document Transaction > {}",
    explorer.message_url(receipt.message_id())?
  );
  println!("Explore the DID Document > {}", explorer.resolver_url(document.did())?);

  Ok((document, keypair, receipt))
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;

  Ok(())
}
