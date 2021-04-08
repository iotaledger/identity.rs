// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example verifiable_credential

mod common;

use identity::core::json;
use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::prelude::*;

// Helper that takes two DID Documents (identities) for issuer and subject, and
// creates a credential with claims about subject by issuer.
fn issue_degree(issuer: &Document, subject: &Document) -> Result<Credential> {
  // Create VC "subject" field containing subject ID and claims about it.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  Ok(credential)
}

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document/KeyPair for the credential issuer (see previous example).
  let (doc_iss, key_iss): (Document, KeyPair) = common::create_did_document(&client).await?;

  // Create a signed DID Document/KeyPair for the credential subject (see previous example).
  let (doc_sub, _key_sub): (Document, KeyPair) = common::create_did_document(&client).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuer secret key - the result is a Verifiable Credential.
  doc_iss.sign_data(&mut credential, key_iss.secret())?;

  println!("Credential > {:#}", credential);
  println!();

  // Convert the Verifiable Credential to JSON and "exchange" with a verifier
  let message: String = credential.to_json()?;

  // Create a `CredentialValidator` instance that will fetch
  // and validate all associated documents from the IOTA Tangle.
  let validator: CredentialValidator = CredentialValidator::new(&client);

  // Perform the validation operation.
  let validation: CredentialValidation = validator.check(&message).await?;

  println!("Credential Validation > {:#?}", validation);
  println!();

  Ok(())
}
