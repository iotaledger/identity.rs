// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::IotaDocument;
use identity::iota::Result;
use identity::iota::TangleRef;

// A helper function to generate and new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
pub async fn document(client: &Client) -> Result<(IotaDocument, KeyPair)> {
  // Generate a new DID Document and public/private key pair.
  //
  // The generated document will have an authentication key associated with
  // the keypair.
  let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
    .authentication_tag("key-1")
    .did_network(client.network().as_str())
    .build()?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  println!("DID Document (signed) > {:#}", document);
  println!();

  // Use the client to publish the DID Document to the Tangle.
  let transaction: _ = client.publish_document(&document).await?;
  let message_id: String = client.transaction_hash(&transaction);

  document.set_message_id(message_id.into());

  println!("DID Document Transaction > {}", client.transaction_url(&transaction));
  println!();

  Ok((document, keypair))
}
