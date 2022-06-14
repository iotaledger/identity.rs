// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_signing

use std::path::PathBuf;

use identity_iota::account::Account;
use identity_iota::account::IdentitySetup;
use identity_iota::account::MethodContent;
use identity_iota::account::Result;
use identity_iota::account_storage::Stronghold;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::Subject;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::verifiable::VerifierOptions;
use identity_iota::did::DID;
use identity_iota::iota::ExplorerUrl;
use identity_iota::iota::ResolvedIotaDocument;
use identity_iota::iota_core::IotaDID;
use identity_iota::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // ===========================================================================
  // Create Identity - Similar to create_did example
  // ===========================================================================

  // Stronghold settings
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".to_owned();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;

  // Create a new Account with stronghold storage.
  let mut account: Account = Account::builder()
    .storage(stronghold)
    .create_identity(IdentitySetup::default())
    .await?;

  // ===========================================================================
  // Signing Example
  // ===========================================================================

  // Add a new Ed25519 Verification Method to the identity
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("key-1")
    .apply()
    .await?;

  // Create a subject DID for the recipient of a `UniversityDegree` credential.
  let subject_key: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let subject_did: IotaDID = IotaDID::new(subject_key.public().as_ref())?;

  // Create the actual Verifiable Credential subject.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject_did.as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  }))?;

  // Issue an unsigned Credential...
  let mut credential: Credential = Credential::builder(Default::default())
    .issuer(Url::parse(account.did().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account.sign("key-1", &mut credential, ProofOptions::default()).await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: ResolvedIotaDocument = account.resolve_identity().await?;

  // Retrieve the DID from the newly created identity.
  let iota_did: &IotaDID = account.did();

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(iota_did)?
  );

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved
    .document
    .verify_data(&credential, &VerifierOptions::default())
    .is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
