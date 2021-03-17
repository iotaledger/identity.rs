// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::prelude::*;
use identity::iota::Network;
use identity::iota::TangleRef;

// A helper function to generate and new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
pub async fn document(client: &Client) -> Result<(Document, KeyPair)> {
  // Generate a new DID Document and public/private key pair.
  //
  // The generated document will have an authentication key associated with
  // the keypair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  println!("DID Document (signed) > {:#}", document);
  println!();

  document.publish(client).await?;

  let network: Network = document.id().into();
  let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());

  println!("DID Document Transaction > {}", explore);
  println!();

  Ok((document, keypair))
}
