// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
//!
//! cargo run --example account_encryption

use std::path::PathBuf;

use identity_account::account::Account;
use identity_account::account::AccountBuilder;
use identity_account::types::IdentitySetup;
use identity_account::types::MethodContent;
use identity_account::Result;
use identity_account_storage::stronghold::Stronghold;
use identity_account_storage::types::AgreementInfo;
use identity_account_storage::types::CekAlgorithm;
use identity_account_storage::types::EncryptedData;
use identity_account_storage::types::EncryptionAlgorithm;
use identity_iota::did::MethodScope;
use identity_iota_client_legacy::document::ResolvedIotaDocument;
use identity_iota_client_legacy::tangle::Resolver;
use identity_iota_core_legacy::document::IotaVerificationMethod;

#[tokio::main]
async fn main() -> Result<()> {
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
  let resolver: Resolver = Resolver::new().await?;

  // Alice: resolves Bob's DID Document and extracts their public key.
  let bob_document: ResolvedIotaDocument = resolver.resolve(bob_account.did()).await?;
  let bob_method: &IotaVerificationMethod = bob_document
    .document
    .resolve_method("kex-0", Some(MethodScope::key_agreement()))
    .unwrap();
  let bob_public_key: Vec<u8> = bob_method.data().try_decode()?;

  // Alice encrypts the data using Diffie-Hellman key exchange
  let agreement: AgreementInfo = AgreementInfo::new(b"Alice".to_vec(), b"Bob".to_vec(), Vec::new(), Vec::new());
  let encryption_algorithm: EncryptionAlgorithm = EncryptionAlgorithm::AES256GCM;
  let cek_algorithm: CekAlgorithm = CekAlgorithm::ECDH_ES(agreement);

  let message: &[u8] = b"This msg will be encrypted and decrypted";
  let encrypted_data: EncryptedData = alice_account
    .encrypt_data(
      message,
      b"associated_data",
      &encryption_algorithm,
      &cek_algorithm,
      bob_public_key.into(),
    )
    .await?;

  // Bob must be able to decrypt the message using the shared secret.
  let decrypted_msg: Vec<u8> = bob_account
    .decrypt_data(encrypted_data, &encryption_algorithm, &cek_algorithm, "kex-0")
    .await?;
  assert_eq!(message, &decrypted_msg);

  // Both shared secret keys computed separately by Alice and Bob matched
  // and were used to exchange an encrypted message.
  println!("Diffie-Hellman key exchange successful!");
  Ok(())
}
