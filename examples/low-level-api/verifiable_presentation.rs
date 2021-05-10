// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A Verifiable Presentation (VP) represents a bundle of one or more Verifiable Credentials.
//! This example demonstrates building and usage of VPs.
//!
//! cargo run --example verifiable_presentation

mod common;

use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::IotaDocument;
use identity::iota::Result;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document/KeyPair for the credential issuer (see previous example).
  let (doc_iss, key_iss): (IotaDocument, KeyPair) = common::create_did_document(&client).await?;

  // Create a signed DID Document/KeyPair for the credential subject (see previous example).
  let (doc_sub, _key_sub): (IotaDocument, KeyPair) = common::create_did_document(&client).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuer secret key - the result is a VerifiableCredential.
  doc_iss.sign_data(&mut credential, key_iss.secret())?;

  // Prepare parameters and build our Presentation.
  let id_url: Url = Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?;
  let holder_url: Url = Url::parse(doc_sub.id().as_ref())?;

  // Build our Verifiable Presentation.
  let mut presentation: Presentation = PresentationBuilder::default() // Create new Presentation using the builder.
    // Note that ::default() already sets the .context() and .type() values for you, so we don't have to do this here
    // anymore.
    .id(id_url) // Optional: Sets a unique identifier for the Presentation.
    .credential(credential) // Adds a Verifiable Credential to the Presentation. Call this multiple times for multiple credentials.
    .holder(holder_url)
    .build()?; // Finally, build the Presentation using the PresentationBuilder configuration.

  // Sign it with the issuer secret key
  doc_iss
    .signer(key_iss.secret())
    .method("#authentication")
    .sign(&mut presentation)?;

  println!("Verifiable Presentation > {:#}", presentation);
  println!();

  Ok(())
}
