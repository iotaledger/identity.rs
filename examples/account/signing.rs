// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_signing

use identity::account::Account;
use identity::account::Result;
use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Calls the create_identity function from the create_did example - this is not part of the framework, but rather reusing the previous example.
  let (account, iota_did): (Account, IotaDID) = create_did::run().await?;
  
  // Add a new Ed25519 Verification Method to the identity
  account
    .update_identity(&iota_did)
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
    .issuer(Url::parse(&iota_did.as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account.sign(&iota_did, "key-1", &mut credential).await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(&iota_did).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved.verify_data(&credential).is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
