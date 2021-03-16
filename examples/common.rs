// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file contains helper functions for the examples.

use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Result;

// A helper function to generate a new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
pub async fn create_did_document(client: &Client) -> Result<(Document, KeyPair)> {

  // Generate a new DID Document and public/private key pair.
  // The generated document will have an authentication key associated with
  // the keypair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the Client to publish the DID Document to the Tangle.
  document.publish(client).await?;

  // Return document and keypair.
  Ok((document, keypair))
}
