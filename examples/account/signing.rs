// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_signing

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a new Account with the default configuration
  let account: Account = Account::builder().build().await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let did: &IotaDID = snapshot.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);

  // Add a new Ed25519 Verification Method to the identity
  account
    .update_identity(did)
    .create_method()
    .fragment("key-1")
    .apply()
    .await?;

  // Create a subject DID for the recipient of a `UniversityDegree` credential.
  let subject_key: KeyPair = KeyPair::new_ed25519()?;
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
    .issuer(Url::parse(did.as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account.sign(did, "key-1", &mut credential).await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(did).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved.verify_data(&credential).is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
