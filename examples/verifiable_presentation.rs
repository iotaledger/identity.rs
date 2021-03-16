// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A Verifiable Presentation (VP) represents a bundle of one or more Verifiable Credentials.
//! This example demonstrates building and usage of VPs.
//! 
//! cargo run --example verifiable_presentation

mod common;

use identity::core::json;
use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::credential::VerifiableCredential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::credential::VerifiablePresentation;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::iota::Document;
use identity::iota::Result;

#[smol_potat::main]
async fn main() -> Result<()> {

  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Create a signed DID Document/KeyPair for the credential issuer (see previous example).
  let (doc_iss, key_iss): (Document, KeyPair) = common::create_did_document(&client).await?;

  // Create a signed DID Document/KeyPair for the credential subject (see previous example).
  let (doc_sub, _key_sub): (Document, KeyPair) = common::create_did_document(&client).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuer secret key - the result is a VerifiableCredential.
  let credential: VerifiableCredential = credential.sign(&doc_iss, "#authentication".into(), key_iss.secret())?;

  // Prepare parameters and build our Presentation.
  let id_url: Url = Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?;
  let holder_url: Url = Url::parse(doc_sub.id().as_ref())?;

  // Build our presentation.
  let presentation: Presentation = PresentationBuilder::default() // Create new Presentation using the builder.
  // Note that ::default() already sets the .context() and .type() values for you, so we don't have to do this here anymore.
    .id(id_url) // Optional: Sets a unique identifier for the Presentation.
    .credential(credential) // Adds a Verifiable Credential to the Presentation. Call this multiple times for multiple credentials.
    .holder(holder_url)
    // .refresh()
    // .policy()
    .build()?; // Finally, build the Presentation using the PresentationBuilder configuration.

    println!("Credential > {:#}", presentation);
    println!();


// WHY IS THERE vc.sign but not pres.sign??


  Ok(())
}
