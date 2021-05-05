// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example signing

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::identity::IdentityCreate;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::MemStore;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::json;
use identity_credential::credential::Credential;
use identity_credential::credential::Subject;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create an in-memory storage instance for the account
  let storage: MemStore = MemStore::new();

  // Create a new Account with the default configuration
  let account: Account<MemStore> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document: &IotaDID = snapshot.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);

  // Add a new Ed25519 Verification Method to the identity
  let command: Command = Command::create_method().fragment("key-1").finish()?;

  // Process the command and update the identity state.
  account.update_identity(document, command).await?;

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
    .issuer(Url::parse(document.as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account.sign(document, "key-1", &mut credential).await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved.verify_data(&credential).is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
