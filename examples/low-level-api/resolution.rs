// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! See also https://www.w3.org/TR/did-core/#did-resolution and https://www.w3.org/TR/did-core/#did-url-dereferencing
//!
//! cargo run --example resolution

use identity::did::resolution;
use identity::did::resolution::Dereference;
use identity::did::resolution::InputMetadata;
use identity::did::resolution::Resolution;
use identity::did::resolution::Resource;
use identity::did::resolution::SecondaryResource;
use identity::did::CoreDID;
use identity::did::DID;
use identity::iota::Receipt;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDIDUrl;
use identity::prelude::*;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (document, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // ===========================================================================
  // DID Resolution
  // ===========================================================================

  let doc_did: &IotaDID = document.id();
  let did_url: &str = doc_did.as_str();

  // Retrieve the published DID Document from the Tangle.
  let input: InputMetadata = Default::default();
  let output: Resolution = resolution::resolve(did_url, input, &client).await?;

  println!("Resolution > {:#?}", output);

  // The resolved Document should be the same as what we published.
  assert_eq!(
    output.document.unwrap(),
    document
      .core_document()
      .clone()
      .map(CoreDID::from, |properties| properties)
  );

  // ===========================================================================
  // DID Dereferencing
  // ===========================================================================

  let resource_url: IotaDIDUrl = doc_did.to_url().join("#sign-0")?;

  // Retrieve a subset of the published DID Document properties.
  let input: InputMetadata = Default::default();
  let output: Dereference = resolution::dereference(resource_url.to_string(), input, &client).await?;

  println!("Dereference > {:#?}", output);

  // The resolved resource should be the DID Document's default signing method.
  match output.content.unwrap() {
    Resource::Secondary(SecondaryResource::VerificationKey(method)) => {
      assert_eq!(method, document.default_signing_method()?.clone().map(CoreDID::from));
    }
    resource => {
      panic!("Invalid Resource Dereference > {:#?}", resource);
    }
  }

  Ok(())
}
