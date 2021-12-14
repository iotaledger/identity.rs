// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example create_vc

use identity::credential::Credential;
use identity::crypto::SignatureOptions;
use identity::iota::ClientMap;
use identity::iota::CredentialValidation;
use identity::iota::Receipt;
use identity::prelude::*;

mod common;
mod create_did;

pub async fn create_vc() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (issuer_doc, issuer_key, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (subject_doc, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&issuer_doc, &subject_doc)?;

  // Sign the Credential with the issuer's private key.
  issuer_doc.sign_data(
    &mut credential,
    issuer_key.private(),
    issuer_doc.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  println!("Credential JSON > {:#}", credential);

  // Validate the verifiable credential
  let validation: CredentialValidation = common::check_credential(&client, &credential).await?;
  println!("Credential Validation > {:#?}", validation);
  assert!(validation.verified);

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let _ = create_vc().await?;
  Ok(())
}
