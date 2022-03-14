// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example create_vc

use identity::core::ToJson;
use identity::credential::Credential;
use identity::crypto::SignatureOptions;
use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::FailFast;
use identity::iota::Receipt;
use identity::prelude::*;

mod common;
mod create_did;

pub async fn create_vc() -> Result<String> {
  // Create a DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (issuer_doc, issuer_key, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a DID Document/KeyPair for the credential subject (see create_did.rs).
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

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  CredentialValidator::validate(
    &credential,
    &issuer_doc,
    &CredentialValidationOptions::default(),
    FailFast::FirstError,
  )?;

  // The issuer is now sure that the credential they are about to issue satisfies their expectations.
  // The credential is then serialized to JSON and transmitted to the subject in a secure manner.
  // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
  let credential_json: String = credential.to_json()?;

  Ok(credential_json)
}

#[tokio::main]
async fn main() -> Result<()> {
  // Obtain a JSON representation of a credential issued to us
  let _credential_json: String = create_vc().await?;
  Ok(())
}
