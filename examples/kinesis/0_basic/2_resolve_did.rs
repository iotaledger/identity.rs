// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use examples_kinesis::create_kinesis_did_document;
use examples_kinesis::get_client_and_create_account;

use identity_iota::iota::IotaDocument;
use identity_iota::iota::KinesisIotaIdentityClientExt;
use identity_iota::prelude::Resolver;
use identity_storage::StorageSigner;
use iota_sdk::types::block::output::AliasId;
use sui_sdk::types::base_types::ObjectID;

/// Demonstrates how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // create new client to interact with chain and get funded account with keys
  let (identity_client, storage, key_id, public_key_jwk) = get_client_and_create_account().await?;
  // create new signer that will be used to sign tx with
  let signer = StorageSigner::new(&storage, key_id, public_key_jwk);
  // create new DID document and publish it
  let document = create_kinesis_did_document(&identity_client, &storage, &signer).await?;

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
  resolver.attach_kinesis_iota_handler(identity_client.clone());

  let resolver_document: IotaDocument = resolver.resolve(&did).await.unwrap();

  // Client and Resolver resolve to the same document in this case.
  assert_eq!(client_document, resolver_document);

  // We can also resolve the raw bytes directly.
  let object_id = ObjectID::from_str(&AliasId::from(&did).to_string())?;
  let raw_did_document: Vec<u8> = identity_client.get_raw_did_document(object_id).await?;

  println!("did document as bytes: {raw_did_document:?}");

  Ok(())
}
