// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to revoke a verifiable credential.
//!
//! The Verifiable Credential is revoked by actually removing a verification method (public key)
//! from the DID Document of the Issuer.
//! As such, the Verifiable Credential can no longer be validated.
//! This would invalidate every Verifiable Credential signed with the same public key, therefore the
//! issuer would have to sign every VC with a different key.
//!
//! cargo run --example did_history

use identity::core::Timestamp;
use identity::credential::Credential;
use identity::crypto::ProofOptions;
use identity::did::MethodScope;
use identity::did::DID;

use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::ExplorerUrl;
use identity::iota::Receipt;

use identity::iota::Resolver;
use identity::iota::Result;
use identity::iota_core::IotaVerificationMethod;
use identity::prelude::*;

mod common;
mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: Client = Client::new().await?;

  // Create a signed VC
  let (issuer, signed_vc) = create_vc_helper(&client).await?;

  // Remove the public key that signed the VC from the issuer's DID document
  // - effectively revoking the VC as it will no longer be able to verified.
  let (mut issuer_doc, issuer_key, issuer_receipt) = issuer;
  issuer_doc.remove_method(&issuer_doc.id().to_url().join("#newKey")?)?;
  issuer_doc.metadata.previous_message_id = *issuer_receipt.message_id();
  issuer_doc.metadata.updated = Some(Timestamp::now_utc());
  issuer_doc.sign_self(issuer_key.private(), issuer_doc.default_signing_method()?.id().clone())?;
  // This is an integration chain update, so we publish the full document.
  let update_receipt = client.publish_document(&issuer_doc).await?;

  // Log the resulting Identity update
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "Issuer Update Transaction > {}",
    explorer.message_url(update_receipt.message_id())?
  );
  println!(
    "Explore the Issuer DID Document > {}",
    explorer.resolver_url(issuer_doc.id())?
  );

  // Check the verifiable credential
  let resolver: Resolver = Resolver::new().await?;
  let resolved_issuer_doc = resolver.resolve_credential_issuer(&signed_vc).await?;
  let validation_result: Result<()> = CredentialValidator::validate(
    &signed_vc,
    &resolved_issuer_doc,
    &CredentialValidationOptions::default(),
    identity::iota::FailFast::FirstError,
  )
  .map_err(Into::into);

  println!("VC validation result: {:?}", validation_result);
  assert!(validation_result.is_err());
  println!("Credential successfully revoked!");

  Ok(())
}

/// Convenience function for creating a verifiable `Credential`, signed with a VerificationMethod
/// with tag #newKey.
///
/// See "create_vc" example for explanation.
async fn create_vc_helper(
  client: &Client,
) -> Result<(
  (IotaDocument, KeyPair, Receipt), // issuer
  Credential,                       // signed verifiable credential
)> {
  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (issuer_doc, issuer_key, issuer_receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (subject_doc, ..) = create_did::run().await?;

  // Add a new VerificationMethod to the issuer with tag #newKey
  // NOTE: this allows us to revoke it without removing the default signing key.
  let (issuer_doc, issuer_new_key, issuer_updated_receipt) =
    common::add_new_key(client, &issuer_doc, &issuer_key, &issuer_receipt).await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&issuer_doc, &subject_doc)?;

  // Sign the Credential with the issuer's #newKey private key, so we can later revoke it
  issuer_doc.sign_data(
    &mut credential,
    issuer_new_key.private(),
    issuer_doc.default_signing_method()?.id(),
    ProofOptions::default(),
  )?;

  let issuer = (issuer_doc, issuer_key, issuer_updated_receipt);
  Ok((issuer, credential))
}

/// Convenience function for adding a new `VerificationMethod` with tag #newKey to a DID document
/// and performing an integration chain update, publishing it to the Tangle.
///
/// See "manipulate_did" for further explanation.
pub async fn add_new_key(
  client: &Client,
  doc: &IotaDocument,
  key: &KeyPair,
  receipt: &Receipt,
) -> Result<(IotaDocument, KeyPair, Receipt)> {
  let mut updated_doc = doc.clone();

  // Add #newKey to the document
  let new_key: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(updated_doc.id().clone(), new_key.type_(), new_key.public(), "newKey")?;
  assert!(updated_doc
    .insert_method(method, MethodScope::VerificationMethod)
    .is_ok());

  // Prepare the update
  updated_doc.metadata.previous_message_id = *receipt.message_id();
  updated_doc.metadata.updated = Some(Timestamp::now_utc());
  updated_doc.sign_self(key.private(), updated_doc.default_signing_method()?.id().clone())?;

  // Publish the update to the Tangle
  let update_receipt: Receipt = client.publish_document(&updated_doc).await?;
  Ok((updated_doc, new_key, update_receipt))
}
