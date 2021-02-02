// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, creates a VerifiableCredential specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example credential

use identity_core::common::Url;
use identity_core::common::Value;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::json;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Credential;
use identity_credential::credential::Subject;
use identity_credential::credential::VerifiableCredential;
use identity_did::did::DID;
use identity_did::verification::MethodScope;
use identity_iota::client::Client;
use identity_iota::credential::CredentialValidation;
use identity_iota::credential::CredentialValidator;
use identity_iota::did::IotaDocument;
use identity_iota::error::Result;

// A helper function to generate and new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
async fn document(client: &Client) -> Result<(IotaDocument, KeyPair)> {
  let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
    .authentication_tag("key-1")
    .did_network(client.network().as_str())
    .build()?;

  document.sign(keypair.secret())?;

  println!("DID Document (signed) > {:#}", document);
  println!();

  let transaction: _ = client.publish_document(&document).await?;

  println!("DID Document Transaction > {}", client.transaction_url(&transaction));
  println!();

  Ok((document, keypair))
}

fn subject(subject: &DID) -> Result<Subject> {
  let json: Value = json!({
    "id": subject.as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  });

  Subject::from_json_value(json).map_err(Into::into)
}

#[smol_potat::main]
async fn main() -> Result<()> {
  let client: Client = Client::new()?;

  let (doc_iss, key_iss): (IotaDocument, KeyPair) = document(&client).await?;
  let (doc_sub, _key_sub): (IotaDocument, KeyPair) = document(&client).await?;

  // Create a new Credential with claims about "subject", specified by "issuer".
  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(doc_iss.id())?)
    .type_("UniversityDegreeCredential")
    .subject(subject(&doc_sub.id())?)
    .build()?;

  // Extract the default verification method from the authentication scope and
  // create a Verifiable Credential signed by the issuer.
  let vc: VerifiableCredential = credential.sign(&doc_iss, MethodScope::Authentication, key_iss.secret())?;

  println!("Credential > {:#}", vc);
  println!();

  // ====================
  // ====================
  //
  // Out-Of-Band DID/Credential Exchange
  //
  // ====================
  // ====================

  let vc: String = vc.to_json()?;

  // Validate the Credential and resolve all DID Documents.
  let validator: CredentialValidator = CredentialValidator::new(&client);
  let validation: CredentialValidation = validator.check(&vc).await?;

  println!("Credential Validation > {:#?}", validation);
  println!();

  Ok(())
}
