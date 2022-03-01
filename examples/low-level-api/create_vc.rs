// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example create_vc

use identity::core::FromJson;
use identity::core::ToJson;
use identity::credential::Credential;
use identity::crypto::SignatureOptions;
use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::FailFast;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
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

  // Before passing this credential on the issuer wants to validate that some properties
  // of the credential satisfy their expectations.
  // In order to validate a credential the issuer's DID Document needs to be resolved.
  // Since the issuer wants to issue and verify several credentials without publishing updates to their DID Document
  // the issuer decides to resolve their DID Document up front now so they can re-use it.

  let resolver: Resolver = Resolver::new().await?;
  let resolved_issuer_doc: ResolvedIotaDocument = resolver.resolve(issuer_doc.id()).await?;
  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure
  // that the issuance date is not in the future and that the expiration date is not in the past:
  CredentialValidator::validate(
    &credential,
    &CredentialValidationOptions::default(),
    &resolved_issuer_doc,
    FailFast::Yes,
  )?;
  // The issuer is now sure that the credential they are about to issue satisfies their expectations
  // hence the credential is now serialized to JSON before passing it to the subject in a secure manner.
  // This means that the credential is NOT published to the tangle where it can be accessed by anyone.
  let credential_json: String = credential.to_json()?;

  Ok(credential_json)
}

#[tokio::main]
async fn main() -> Result<()> {
  // Obtain a JSON representation of a credential issued to us
  let credential_json: String = create_vc().await?;
  // Run some basic validations on this credential so we can immediately contact the issuer if something is not right
  let credential: Credential = Credential::from_json(credential_json.as_str())?;
  // We can use the Resolver to conveniently validate basic properties of the credential.
  let resolver: Resolver = Resolver::new().await?;
  // We use FailFast::No so all errors get accumulated if there are any. Then we can later inform the issuer about
  // everything that is wrong with the credential they issued to us and they will hopefully send us a new correct
  // credential :)
  resolver
    .verify_credential(&credential, &CredentialValidationOptions::default(), FailFast::No)
    .await?;
  Ok(())
}
