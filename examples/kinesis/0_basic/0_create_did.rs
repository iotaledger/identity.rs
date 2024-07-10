// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples_kinesis::create_kinesis_did_document;
use examples_kinesis::get_client_and_create_account;

use identity_iota::iota::KinesisIotaIdentityClientExt;
use identity_storage::StorageSigner;

/// Demonstrates how to create a DID Document and publish it on chain.
///
/// In this example we connect to a locally running private network, but it can be adapted
/// to run on any IOTA node by setting the network and faucet endpoints.
///
/// See the following instructions on running your own private network
/// https://github.com/iotaledger/hornet/tree/develop/private_tangle
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // create new client to interact with chain and get funded account with keys
  let (identity_client, storage, key_id, public_key_jwk) = get_client_and_create_account().await?;
  // create new signer that will be used to sign tx with
  let signer = StorageSigner::new(&storage, key_id, public_key_jwk);

  // create new DID document and publish it
  let document = create_kinesis_did_document(&identity_client, &storage, &signer).await?;
  println!("Published DID document: {document:#}");

  // check if we can resolve it via client
  let resolved = identity_client.resolve_did(document.id()).await?;
  println!("Resolved DID document: {resolved:#}");

  Ok(())
}
