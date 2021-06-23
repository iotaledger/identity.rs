// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example create_vc

mod common;
mod create_did;

use identity::core::ToJson;
use identity::credential::Credential;
use identity::crypto::KeyPair;
use identity::iota::ClientMap;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::iota::Receipt;
use identity::prelude::*;

pub async fn issue() -> Result<String> {
  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (doc_iss, key_iss, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (doc_sub, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuers secret key
  doc_iss.sign_data(&mut credential, key_iss.secret())?;

  println!("Credential JSON > {:#}", credential);

  // Convert the Verifiable Credential to JSON and "exchange" with a verifier
  credential.to_json().map_err(Into::into)
}

#[tokio::main]
async fn main() -> Result<()> {
  // Issue a Verifiable Credential with a newly created DID Document.
  let credential_json: String = issue().await?;

  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a `CredentialValidator` instance to fetch and validate all
  // associated DID Documents from the Tangle.
  let validator: CredentialValidator<ClientMap> = CredentialValidator::new(&client);

  // Perform the validation operation.
  let validation: CredentialValidation = validator.check(&credential_json).await?;
  assert!(validation.verified);

  println!("Credential Validation > {:#?}", validation);

  Ok(())
}
