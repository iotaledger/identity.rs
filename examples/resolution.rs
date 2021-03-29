// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! cargo run --example resolution

use identity::core::SerdeInto;
use identity::did::resolution;
use identity::did::resolution::Resource;
use identity::did::resolution::SecondaryResource;
use identity::iota::DID;
use identity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new client connected to the Testnet (Chrysalis).
  let client: Client = Client::new().await?;
  // Create a pair of Ed25519 public/secret keys.
  let key: KeyPair = KeyPair::new_ed25519()?;

  // Create a DID Document using the Ed25519 public key as the authentication
  // method. The DID URL is also derived from the public key value.
  let mut doc: Document = Document::from_keypair(&key)?;

  // Sign the document with our Ed25519 secret key.
  doc.sign(key.secret())?;

  // Publish the DID Document to the Tangle - Use a custom Client instance.
  doc.publish(&client).await?;

  // ===========================================================================
  // DID Resolution
  // ===========================================================================

  // Retrieve the published DID Document from the Tangle.
  let future: _ = resolution::resolve(doc.id().as_str(), Default::default(), &client);
  let data: _ = future.await?;

  println!("Resolution: {:#?}", data);

  // The resolved Document should be the same as what we published.
  assert_eq!(data.document.unwrap(), doc.serde_into().unwrap());

  // ===========================================================================
  // DID Dereferencing
  // ===========================================================================

  let url: DID = doc.id().join("#authentication")?;

  // Retrieve a subset of the published DID Document properties.
  let future: _ = resolution::dereference(url.as_str(), Default::default(), &client);
  let data: _ = future.await?;

  println!("Dereference: {:#?}", data);

  // The resolved resource should be the DID Document authentication method.
  assert!(matches!(
    data.content.unwrap(),
    Resource::Secondary(SecondaryResource::VerificationKey(method)) if method == **doc.authentication()
  ));

  Ok(())
}
