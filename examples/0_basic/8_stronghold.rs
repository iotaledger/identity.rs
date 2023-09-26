// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::get_address_with_funds;
use examples::random_stronghold_path;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::credential::Jws;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::resolver::Resolver;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::Storage;
use identity_iota::verification::jws::DecodedJws;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use identity_stronghold::StrongholdStorage;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;

/// Demonstrates how to use stronghold for secure storage.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // The API endpoint of an IOTA node, e.g. Hornet.
  let api_endpoint: &str = "http://127.0.0.1:14265";

  // The faucet endpoint allows requesting funds for testing purposes.
  let faucet_endpoint: &str = "http://127.0.0.1:8091/api/enqueue";

  // Stronghold snapshot path.
  let path = random_stronghold_path();

  // Stronghold password.
  let password = Password::from("secure_password".to_owned());

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(api_endpoint, None)?
    .finish()
    .await?;

  let stronghold = StrongholdSecretManager::builder()
    .password(password.clone())
    .build(path.clone())?;

  // Create a `StrongholdStorage`.
  // `StrongholdStorage` creates internally a `SecretManager` that can be
  // referenced to avoid creating multiple instances around the same stronghold snapshot.
  let stronghold_storage = StrongholdStorage::new(stronghold);

  // Create a DID document.
  let address: Address =
    get_address_with_funds(&client, stronghold_storage.as_secret_manager(), faucet_endpoint).await?;
  let network_name: NetworkName = client.network_name().await?;
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Create storage for key-ids and JWKs.
  //
  // In this example, the same stronghold file that is used to store
  // key-ids as well as the JWKs.
  let storage = Storage::new(stronghold_storage.clone(), stronghold_storage.clone());

  // Generates a verification method. This will store the key-id as well as the private key
  // in the stronghold file.
  let fragment = document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client
    .publish_did_output(stronghold_storage.as_secret_manager(), alias_output)
    .await?;

  // Resolve the published DID Document.
  let mut resolver = Resolver::<IotaDocument>::new();
  resolver.attach_iota_handler(client.clone());
  let resolved_document: IotaDocument = resolver.resolve(document.id()).await.unwrap();

  drop(stronghold_storage);

  // Create the storage again to demonstrate that data are read from the stronghold file.
  let stronghold = StrongholdSecretManager::builder()
    .password(password.clone())
    .build(path.clone())?;
  let stronghold_storage = StrongholdStorage::new(stronghold);
  let storage = Storage::new(stronghold_storage.clone(), stronghold_storage.clone());

  // Sign data with the created verification method.
  let data = b"test_data";
  let jws: Jws = resolved_document
    .create_jws(&storage, &fragment, data, &JwsSignatureOptions::default())
    .await?;

  // Verify Signature.
  let decoded_jws: DecodedJws = resolved_document.verify_jws(
    &jws,
    None,
    &EdDSAJwsVerifier::default(),
    &JwsVerificationOptions::default(),
  )?;

  assert_eq!(String::from_utf8_lossy(decoded_jws.claims.as_ref()), "test_data");

  Ok(())
}
