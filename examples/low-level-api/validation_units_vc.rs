// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Timestamp;
use identity::credential::Credential;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifierOptions;
use identity::iota::CredentialValidator;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;

use identity::prelude::*;

mod common;
mod create_did;

const TWO_WEEKS_IN_SECONDS: i64 = 1209600;

pub async fn validation_units_vc() -> Result<()> {
  // Create a client instance to send messages to the Tangle.

  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (issuer_doc, issuer_key, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (subject_doc, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&issuer_doc, &subject_doc)?;

  // define a timestamp representing two weeks from now
  let two_weeks_from_now_unix: i64 = Timestamp::now_utc().to_unix() + TWO_WEEKS_IN_SECONDS;
  let two_weeks_from_now_timestamp: Timestamp = Timestamp::from_unix(two_weeks_from_now_unix)?;

  // set the expiration date of the credential to be two weeks from now
  credential.expiration_date = Some(two_weeks_from_now_timestamp);

  // Sign the Credential with the issuer's private key.
  issuer_doc.sign_data(
    &mut credential,
    issuer_key.private(),
    issuer_doc.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  println!("Credential JSON > {:#}", credential);

  // validate that the credential is valid two weeks from now
  CredentialValidator::check_expires_on_or_after(&credential, two_weeks_from_now_timestamp)?;

  // define a timestamp representing two weeks ago
  let two_weeks_ago_unix: i64 = Timestamp::now_utc().to_unix() - TWO_WEEKS_IN_SECONDS;
  let two_weeks_ago_timestamp: Timestamp = Timestamp::from_unix(two_weeks_ago_unix)?;

  // validate that the credential is active now
  CredentialValidator::check_is_issued_on_or_before(&credential, Timestamp::now_utc())?;

  // validate whether the credential has been active for at least two weeks
  if CredentialValidator::check_is_issued_on_or_before(&credential, two_weeks_ago_timestamp).is_err() {
    println!("the credential has been active for less than two weeks!");
  }

  // finally we validate the credential signature
  // the validation unit that verifies the issuer's signature needs a list of resolved DID documents of trusted issuers.
  // since we trust our issuer in this case we create a list consisting of this issuer's resolved DID document.

  //Todo: Use the new Resolver to get the necessary DID documents once that becomes available.
  let trusted_issuers: Vec<ResolvedIotaDocument> = common::resolve_documents(&[issuer_doc]).await?;
  CredentialValidator::verify_signature(&credential, trusted_issuers.as_slice(), &VerifierOptions::default())?;
  println!("the credential signature has been verified");
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  validation_units_vc().await
}
