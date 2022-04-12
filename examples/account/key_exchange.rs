// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.
//!
//! cargo run --example key_exchange

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::EncryptionKey;
use identity::account::IdentitySetup;
use identity::account::MethodContent;
use identity::account::Result;
use identity::account_storage::EncryptedData;
use identity::account_storage::Stronghold;
use identity::did::MethodScope;
use identity::iota::Client;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleResolve;
use identity::iota_core::IotaVerificationMethod;

mod create_did;

pub async fn run() -> Result<()> {
  // Alice and Bob want to communicate securely by encrypting their messages so only they
  // can read them. They both publish DID Documents with X25519 public keys and use them
  // to derive a shared secret key for encryption.

  // Sets the location and password for the Stronghold
  //
  // Stronghold is an encrypted file that manages private keys.
  // It implements best practices for security and is the recommended way of handling private keys.
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".to_owned();
  let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;

  // Alice creates and publishes their DID Document (see create_did and manipulate_did examples).
  let mut builder: AccountBuilder = Account::builder().storage(stronghold);
  let mut alice_account: Account = builder.create_identity(IdentitySetup::default()).await?;
  alice_account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateX25519)
    .fragment("kex-0")
    .scope(MethodScope::key_agreement())
    .apply()
    .await?;

  // Bob creates and publishes their DID Document (see create_did and manipulate_did examples).
  let mut bob_account: Account = builder.create_identity(IdentitySetup::default()).await?;
  bob_account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateX25519)
    .fragment("kex-0")
    .scope(MethodScope::key_agreement())
    .apply()
    .await?;

  // Alice and Bob tell each other their DIDs. They each resolve the DID Document of the other
  // to obtain their X25519 public key. Note that in practice, they would run this code completely
  // separately.
  let client: Client = Client::new().await?;

  // Alice: resolves Bob's DID Document and extracts their public key.
  let bob_document: ResolvedIotaDocument = client.resolve(bob_account.did()).await?;
  let bob_method: &IotaVerificationMethod = bob_document
    .document
    .resolve_method("kex-0", Some(MethodScope::key_agreement()))
    .unwrap();
  let bob_public_key: Vec<u8> = bob_method.data().try_decode()?;

  // Bob: resolves Alice's DID Document and extracts their public key.
  let alice_document: ResolvedIotaDocument = client.resolve(alice_account.did()).await?;
  let alice_method: &IotaVerificationMethod = alice_document
    .document
    .resolve_method("kex-0", Some(MethodScope::key_agreement()))
    .unwrap();
  let alice_public_key: Vec<u8> = alice_method.data().try_decode()?;

  // Alice encrypts the data using Diffie-Hellman key exchange
  let message: &[u8] = "This msg will be encrypted and decrypted".as_bytes();
  let encrypted_data: EncryptedData = alice_account
    .encrypt_data("kex-0", &EncryptionKey::X25519(bob_public_key.clone().into()), message)
    .await?;
  // Bob must be able to decrypt the message using the shared secret
  let decrypted_msg: Vec<u8> = bob_account
    .decrypt_data("kex-0", &EncryptionKey::X25519(alice_public_key.into()), encrypted_data)
    .await?;
  assert_eq!(message, &decrypted_msg);

  // Both shared secret keys computed separately by Alice and Bob will match
  // and can then be used to establish encrypted communications.
  println!("Diffie-Hellman key exchange successful!");
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let _ = run().await?;
  Ok(())
}
