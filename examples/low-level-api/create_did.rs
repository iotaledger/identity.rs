// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes a DID Document,
//! the fundamental building block for decentralized identity.
//!
//! cargo run --example create_did

use identity_iota::client::ExplorerUrl;
use identity_iota::client::Receipt;
use identity_iota::prelude::*;

pub async fn run() -> Result<(IotaDocument, KeyPair, Receipt)> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // Generate a new Ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;

  // Create a DID Document (an identity) from the generated key pair.
  let mut document: IotaDocument = IotaDocument::new(&keypair)?;

  // Sign the DID Document with the default signing method.
  document.sign_self(keypair.private(), document.default_signing_method()?.id().clone())?;

  println!("DID Document JSON > {:#}", document);

  // Publish the DID Document to the Tangle.
  let receipt: Receipt = client.publish_document(&document).await?;

  println!("Publish Receipt > {:#?}", receipt);

  // Display the web explorer url that shows the published message.
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "DID Document Transaction > {}",
    explorer.message_url(receipt.message_id())?
  );
  println!("Explore the DID Document > {}", explorer.resolver_url(document.id())?);

  Ok((document, keypair, receipt))
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;

  Ok(())
}
