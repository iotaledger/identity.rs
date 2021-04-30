// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example signing

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::identity::IdentitySnapshot;
use identity_account::storage::MemStore;
use identity_core::common::Url;
use identity_core::convert::SerdeInto;
use identity_core::json;
use identity_credential::credential::Credential;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::Document;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create an in-memory storage instance for the account
  let storage: MemStore = MemStore::new();

  // Create a new Account with the default configuration
  let account: Account<MemStore> = Account::new(storage).await?;

  // Create a new Identity with default settings
  let snapshot: IdentitySnapshot = account.create(Default::default()).await?;

  println!("[Example] Local Snapshot = {:#?}", snapshot);
  println!("[Example] Local Document = {:#?}", snapshot.identity().to_document()?);

  // Add a new Ed25519 verification method to the identity
  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .scope(MethodScope::VerificationMethod)
    .fragment("key-1")
    .finish()?;

  // Process the command and update the identity state.
  account.update(snapshot.id(), command).await?;

  let mut credential: Credential = Credential::builder(Default::default())
    .issuer(Url::parse("https://example.com")?)
    .type_("ExampleCredential")
    .subject(json!({"foo": "bar"}).serde_into()?)
    .build()?;

  // Sign the credential with the previously created verification method
  account.sign(snapshot.id(), "key-1", &mut credential).await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  let resolved: Document = account.resolve(snapshot.id()).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved.verify_data(&credential).is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
