// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! See also https://www.w3.org/TR/did-core/#did-resolution and https://www.w3.org/TR/did-core/#did-url-dereferencing
//!
//! cargo run --example resolution

use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::iota_core::IotaDID;
use identity::prelude::*;

mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a signed DID Document and KeyPair (see create_did.rs).
  let (document, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // ===========================================================================
  // Resolve DID Document
  // ===========================================================================

  let doc_did: &IotaDID = document.id();

  // Retrieve the published DID Document from the Tangle.
  let resolver: Resolver = Resolver::new().await?;
  let resolved_did_document: ResolvedIotaDocument = resolver.resolve(doc_did).await?;

  println!("Resolved DID Document > {:#?}", resolved_did_document);

  // The resolved Document should be the same as what we published.
  assert_eq!(resolved_did_document.document, document);

  Ok(())
}
