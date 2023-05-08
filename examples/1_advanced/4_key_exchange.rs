// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use examples::get_address_with_funds;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use examples::FAUCET_ENDPOINT;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::crypto::X25519;
use identity_iota::iota::block::address::Address;
use identity_iota::iota::block::output::AliasOutput;
use identity_iota::iota::block::output::RentStructure;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::verification::MethodScope;
use identity_iota::verification::VerificationMethod;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;

/// Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.
///
/// Alice and Bob want to communicate securely by encrypting their messages so only they
/// can read them. They both publish DID Documents with X25519 public keys and use them
/// to derive a shared secret key for encryption.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ==============================
  // Create DIDs for Alice and Bob.
  // ==============================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Get an address and with funds for testing.
  let address: Address = get_address_with_funds(&client, &mut secret_manager, FAUCET_ENDPOINT)
    .await
    .context("failed to get address with funds")?;

  let network: NetworkName = client.network_name().await?;
  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // Alice creates and publishes their DID Document.
  let (alice_did, alice_x25519): (IotaDID, KeyPair) = {
    // Create a DID Document.
    let mut alice_document: IotaDocument = IotaDocument::new(&network);

    // Insert a new X25519 KeyAgreement verification method.
    let x25519: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: VerificationMethod =
      VerificationMethod::new(alice_document.id().clone(), KeyType::X25519, x25519.public(), "kex-0")?;
    alice_document.insert_method(method, MethodScope::key_agreement())?;

    // Publish the DID.
    let alice_output: AliasOutput = client
      .new_did_output(address, alice_document, Some(rent_structure.clone()))
      .await?;
    let alice_document: IotaDocument = client.publish_did_output(&secret_manager, alice_output).await?;

    (alice_document.id().clone(), x25519)
  };

  // Bob creates and publishes their DID Document.
  let (bob_did, bob_x25519): (IotaDID, KeyPair) = {
    // Create a DID Document.
    let mut bob_document: IotaDocument = IotaDocument::new(&network);

    // Insert a new X25519 KeyAgreement verification method.
    let x25519: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: VerificationMethod =
      VerificationMethod::new(bob_document.id().clone(), KeyType::X25519, x25519.public(), "kex-0")?;
    bob_document.insert_method(method, MethodScope::key_agreement())?;

    // Publish the DID.
    let bob_output: AliasOutput = client
      .new_did_output(address, bob_document, Some(rent_structure))
      .await?;
    let bob_document: IotaDocument = client.publish_did_output(&secret_manager, bob_output).await?;

    (bob_document.id().clone(), x25519)
  };

  // ======================================================================
  // Alice and Bob tell each other their DIDs. They each resolve the
  // DID Document of the other to obtain their X25519 public key.
  // Note that in practice, they would run this code completely separately.
  // ======================================================================

  let alice_shared_secret_key: [u8; 32] = {
    // Alice: resolves Bob's DID Document and extracts their public key.
    let bob_document: IotaDocument = client.resolve_did(&bob_did).await?;
    let bob_method: &VerificationMethod = bob_document
      .core_document()
      .resolve_method("kex-0", Some(MethodScope::key_agreement()))
      .unwrap();
    let bob_public_key: Vec<u8> = bob_method.data().try_decode()?;

    // Compute the shared secret.
    X25519::key_exchange(alice_x25519.private(), &bob_public_key)?
  };

  let bob_shared_secret_key: [u8; 32] = {
    // Bob: resolves Alice's DID Document and extracts their public key.
    let alice_document: IotaDocument = client.resolve_did(&alice_did).await?;
    let alice_method: &VerificationMethod = alice_document
      .core_document()
      .resolve_method("kex-0", Some(MethodScope::key_agreement()))
      .unwrap();
    let alice_public_key: Vec<u8> = alice_method.data().try_decode()?;

    // Compute the shared secret.
    X25519::key_exchange(bob_x25519.private(), &alice_public_key)?
  };

  // Both shared secret keys computed separately by Alice and Bob will match
  // and can then be used to establish encrypted communications.
  assert_eq!(alice_shared_secret_key, bob_shared_secret_key);

  println!("Diffie-Hellman key exchange successful!");

  Ok(())
}
