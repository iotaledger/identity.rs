// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates and publishes subject and issuer DID
//! Documents, then creates a Verifiable Credential (vc) specifying claims about the
//! subject, and retrieves information through the CredentialValidator API.
//!
//! cargo run --example account_create_vc

use identity_iota::account::Account;
use identity_iota::account::AccountBuilder;
use identity_iota::account::IdentitySetup;
use identity_iota::account::MethodContent;
use identity_iota::account::Result;

use identity_iota::client::CredentialValidationOptions;
use identity_iota::client::CredentialValidator;
use identity_iota::client::FailFast;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::Subject;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DID;

pub async fn create_vc() -> Result<String> {
  // Create an account builder with in-memory storage for simplicity.
  // See `create_did` example to configure Stronghold storage.
  let mut builder: AccountBuilder = Account::builder();

  // Create an identity for the issuer.
  let mut issuer: Account = builder.create_identity(IdentitySetup::default()).await?;

  // Add verification method to the issuer.
  issuer
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("issuerKey")
    .apply()
    .await?;

  // Create an identity for the holder, in this case also the subject.
  let alice: Account = builder.create_identity(IdentitySetup::default()).await?;

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": alice.document().id(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Build credential using subject above and issuer.
  let mut credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer.did().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // Sign the Credential with the issuer's verification method.
  issuer
    .sign("#issuerKey", &mut credential, ProofOptions::default())
    .await?;

  println!("Credential JSON > {:#}", credential);

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  CredentialValidator::validate(
    &credential,
    issuer.document(),
    &CredentialValidationOptions::default(),
    FailFast::FirstError,
  )
  .unwrap();

  println!("VC successfully validated");

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
