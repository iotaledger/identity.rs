// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::MemStorage;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::KinesisIotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use identity_storage::JwkStorage;
use identity_storage::KeyType;
use identity_storage::StorageSigner;
use identity_sui_name_tbd::client::convert_to_address;
use identity_sui_name_tbd::client::get_sender_public_key;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::utils::request_funds;

use sui_sdk::SuiClientBuilder;

const TEST_GAS_BUDGET: u64 = 50_000_000;

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
///
/// In this example we connect to a locally running private network, but it can be adapted
/// to run on any IOTA node by setting the network and faucet endpoints.
///
/// See the following instructions on running your own private network
/// https://github.com/iotaledger/hornet/tree/develop/private_tangle
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // The API endpoint of an IOTA node
  let api_endpoint: &str = "http://127.0.0.1:9000";

  // Insert a new Ed25519 verification method in the DID document.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let sui_client = SuiClientBuilder::default()
    .build(api_endpoint)
    .await
    .map_err(|err| anyhow::anyhow!(format!("failed to connect to network; {}", err)))?;

  // generate new key
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key_jwk = generate.jwk.to_public().expect("public components should be derivable");
  let public_key_bytes = get_sender_public_key(&public_key_jwk)?;
  let sender_address = convert_to_address(&public_key_bytes)?;
  request_funds(&sender_address).await?;

  let identity_client: IdentityClient = IdentityClient::builder()
    .sui_client(sui_client)
    .sender_public_key(&public_key_bytes)
    .build()?;

  let signer = StorageSigner::new(&storage, generate.key_id, public_key_jwk);

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut unpublished: IotaDocument = IotaDocument::new(&NetworkName::try_from(identity_client.network_name())?);

  unpublished
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  let document = identity_client
    .publish_did_document(unpublished, TEST_GAS_BUDGET, &signer)
    .await?;
  println!("Published DID document: {document:#}");

  let resolved = identity_client.resolve_did(document.id()).await?;
  println!("Resolved DID document: {resolved:#}");

  Ok(())
}
