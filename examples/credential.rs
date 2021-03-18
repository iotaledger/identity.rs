// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, creates a Credential specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example credential

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
use identity::iota::Document;
use identity::iota::Result;

fn issue_degree(issuer: &Document, subject: &Document) -> Result<Credential> {
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  }))?;

  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  Ok(credential)
}

#[smol_potat::main]
async fn main() -> Result<()> {
  // Initialize a `Client` to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Create a DID Document/KeyPair for the credential issuer.
  let (doc_iss, key_iss): (Document, KeyPair) = common::document(&client).await?;

  // Create a DID Document/KeyPair for the credential subject.
  let (doc_sub, _key_sub): (Document, KeyPair) = common::document(&client).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let credential: Credential = issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuer secret key - the result is a Credential.
  let credential: Credential = credential.sign(&doc_iss, "#authentication".into(), key_iss.secret())?;

  println!("Credential > {:#}", credential);
  println!();

  // Convert the Credential to JSON and "exchange" with a verifier
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
