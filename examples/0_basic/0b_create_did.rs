// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use examples::get_address_with_funds;
use examples::random_stronghold_path;
use examples::MemStorage;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::jwk::Jwk;
use identity_iota::verification::jwk::JwkType;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use identity_storage::JwkStorage;
use identity_storage::KeyId;
use identity_storage::KeyType;
use identity_storage::StorageSigner;
use identity_sui_name_tbd::client::convert_to_address;
use identity_sui_name_tbd::client::get_sender_public_key;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::utils::request_funds;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::SuiClientBuilder;

const TEST_DOC: &[u8] = &[
  68, 73, 68, 1, 0, 131, 1, 123, 34, 100, 111, 99, 34, 58, 123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58,
  48, 34, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91,
  123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58, 48, 35, 79, 115, 55, 95, 66, 100, 74, 120, 113, 86, 119,
  101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100, 108, 74, 73, 99, 48,
  45, 100, 50, 49, 73, 34, 44, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58, 48,
  58, 48, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 74, 115, 111, 110, 87, 101, 98, 75, 101, 121, 34, 44, 34, 112,
  117, 98, 108, 105, 99, 75, 101, 121, 74, 119, 107, 34, 58, 123, 34, 107, 116, 121, 34, 58, 34, 79, 75, 80, 34, 44,
  34, 97, 108, 103, 34, 58, 34, 69, 100, 68, 83, 65, 34, 44, 34, 107, 105, 100, 34, 58, 34, 79, 115, 55, 95, 66, 100,
  74, 120, 113, 86, 119, 101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100,
  108, 74, 73, 99, 48, 45, 100, 50, 49, 73, 34, 44, 34, 99, 114, 118, 34, 58, 34, 69, 100, 50, 53, 53, 49, 57, 34, 44,
  34, 120, 34, 58, 34, 75, 119, 99, 54, 89, 105, 121, 121, 65, 71, 79, 103, 95, 80, 116, 118, 50, 95, 49, 67, 80, 71,
  52, 98, 86, 87, 54, 102, 89, 76, 80, 83, 108, 115, 57, 112, 122, 122, 99, 78, 67, 67, 77, 34, 125, 125, 93, 125, 44,
  34, 109, 101, 116, 97, 34, 58, 123, 34, 99, 114, 101, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50, 52, 45, 48, 53, 45,
  50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 44, 34, 117, 112, 100, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50,
  52, 45, 48, 53, 45, 50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 125, 125,
];
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
  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = NetworkName::try_from("iota")?;

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
    .identity_iota_package_id(ObjectID::from_str(
      "0x4993b68aa0656850d7bd1559f439e48510533a60dea7a3b8844bc21da080d593",
    )?)
    .sui_client(sui_client)
    .sender_public_key(&public_key_bytes)
    .build()?;

  let signer = StorageSigner::new(&storage, generate.key_id, public_key_jwk);

  // // Create a new client to interact with the IOTA ledger.
  // let client: Client = Client::builder()
  //   .with_primary_node(api_endpoint, None)?
  //   .finish()
  //   .await?;

  // // Create a new secret manager backed by a Stronghold.
  // let secret_manager: SecretManager = SecretManager::Stronghold(
  //   StrongholdSecretManager::builder()
  //     .password(Password::from("secure_password".to_owned()))
  //     .build(random_stronghold_path())?,
  // );

  // // Get an address with funds for testing.
  // let address: Address = get_address_with_funds(&client, &secret_manager, faucet_endpoint).await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  // // Construct an Alias Output containing the DID document, with the wallet address
  // // set as both the state controller and governor.
  // let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // // Publish the Alias Output and get the published DID document.
  // let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  // println!("Published DID document: {document:#}");

  let document: ObjectID = identity_client.publish_did(TEST_DOC, TEST_GAS_BUDGET, &signer).await?;
  println!("Published DID document: {document:#}");

  Ok(())
}
