// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An example that revokes a key and shows how verification fails as a consequence.
//!
//! cargo run --example merkle_key

mod common;
mod create_did;

use identity::core::ToJson;
use identity::credential::Credential;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::KeyCollection;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::did::MethodScope;
use identity::iota::ClientMap;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::iota::IotaDID;
use identity::iota::IotaVerificationMethod;
use identity::iota::Receipt;
use identity::prelude::*;
use rand::rngs::OsRng;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (mut doc_iss, key_iss, receipt): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (doc_sub, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Generate a Merkle Key Collection Verification Method with 8 keys (Must be a power of 2)
  let keys: KeyCollection = KeyCollection::new_ed25519(8)?;
  let method_did: IotaDID = doc_iss.id().clone();
  let method = IotaVerificationMethod::create_merkle_key::<Sha256, _>(method_did, &keys, "merkle-key")?;

  // Add to the DID Document as a general-purpose verification method
  doc_iss.insert_method(MethodScope::VerificationMethod, method);
  doc_iss.set_previous_message_id(*receipt.message_id());
  doc_iss.sign(key_iss.secret())?;

  let receipt: Receipt = client.publish_document(&doc_iss).await?;

  println!("Publish Receipt > {:#?}", receipt);

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Select a random key from the collection
  let index: usize = OsRng.gen_range(0..keys.len());

  let public: &PublicKey = keys.public(index).unwrap();
  let secret: &SecretKey = keys.secret(index).unwrap();

  // Generate an inclusion proof for the selected key
  let proof: Proof<Sha256> = keys.merkle_proof(index).unwrap();

  // Sign the Credential with the issuers secret key
  doc_iss
    .signer(secret)
    .method("merkle-key")
    .merkle_key((public, &proof))
    .sign(&mut credential)?;

  println!("Credential JSON > {:#}", credential);

  let credential_json: String = credential.to_json()?;

  // Check the verifiable credential
  let validator: CredentialValidator<ClientMap> = CredentialValidator::new(&client);
  let validation: CredentialValidation = validator.check(&credential_json).await?;
  assert!(validation.verified);

  println!("Credential Validation > {:#?}", validation);

  // The Issuer would like to revoke the credential (and therefore revokes key at `index`)
  doc_iss
    .try_resolve_mut("merkle-key")
    .and_then(IotaVerificationMethod::try_from_mut)?
    .revoke_merkle_key(index)?;
  doc_iss.set_previous_message_id(*receipt.message_id());
  doc_iss.sign(key_iss.secret())?;

  let receipt: Receipt = client.publish_document(&doc_iss).await?;

  println!("Publish Receipt > {:#?}", receipt);

  // Check the verifiable credential
  let validation: CredentialValidation = validator.check(&credential_json).await?;
  assert!(!validation.verified);

  println!("Credential Validation > {:#?}", validation);

  Ok(())
}
