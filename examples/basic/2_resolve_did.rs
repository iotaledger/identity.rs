// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! See also https://www.w3.org/TR/did-core/#did-resolution and https://www.w3.org/TR/did-core/#did-url-dereferencing
//!
//! cargo run --example resolve_did

use identity::account::IdentitySetup;
use identity::account::Result;
use identity::account_storage::Stronghold;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::iota_core::IotaDID;
use identity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
   // Sets the location and password for the Stronghold
  //
  // Stronghold is an encrypted file that manages private keys.
  // It implements best practices for security and is the recommended way of handling private keys.
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".to_owned();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;

  // Create a new identity using Stronghold as local storage.
  //
  // The creation step generates a keypair, builds an identity
  // and publishes it to the IOTA mainnet.
  let account: Account = Account::builder()
    .storage(stronghold)
    .create_identity(IdentitySetup::default())
    .await?;

  // Retrieve the did of the newly created identity.
  let doc_did: &IotaDID = account.did();

  // Retrieve the published DID Document from the Tangle.
  let resolver: Resolver = Resolver::new().await?;
  let resolved_did_document: ResolvedIotaDocument = resolver.resolve(doc_did).await?;

  println!("Resolved DID Document > {:#?}", resolved_did_document);

  // The resolved document should be the same as what we published.
  assert_eq!(resolved_did_document.document, document);

  Ok(())
}
