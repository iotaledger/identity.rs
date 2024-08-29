// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples_kinesis::create_kinesis_did_document;
use examples_kinesis::get_client_and_create_account;

use examples_kinesis::get_memstorage;
use identity_iota::iota::IotaDocument;
use identity_iota::prelude::Resolver;

/// Demonstrates how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // create new client to interact with chain and get funded account with keys
  let storage = get_memstorage()?;
  let identity_client = get_client_and_create_account(&storage).await?;
  // create new DID document and publish it
  let (document, _) = create_kinesis_did_document(&identity_client, &storage).await?;

  let did = document.id().clone();

  // We can resolve a `IotaDID` to bytes via client.
  // Resolve the associated Alias Output and extract the DID document from it.
  let client_document: IotaDocument = identity_client.resolve_did(&did).await?;
  println!("Client resolved DID Document: {client_document:#}");

  // We can also create a `Resolver` that has additional convenience methods,
  // for example to resolve presentation issuers or to verify presentations.
  let mut resolver = Resolver::<IotaDocument>::new();

  // We need to register a handler that can resolve IOTA DIDs.
  // This convenience method only requires us to provide a client.
  resolver.attach_kinesis_iota_handler((*identity_client).clone());

  let resolver_document: IotaDocument = resolver.resolve(&did).await.unwrap();

  // Client and Resolver resolve to the same document.
  assert_eq!(client_document, resolver_document);

  Ok(())
}
