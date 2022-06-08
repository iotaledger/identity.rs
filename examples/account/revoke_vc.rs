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
//! cargo run --example account_revoke_vc

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::IdentitySetup;
use identity::account::MethodContent;
use identity::account::Result;
use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::RevocationBitmapStatus;
use identity::credential::Subject;
use identity::crypto::ProofOptions;
use identity::did::RevocationBitmap;
use identity::did::DID;
use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::iota::ValidationError;
use identity::iota_core::IotaDIDUrl;

#[tokio::main]
async fn main() -> Result<()> {
  // ===========================================================================
  // Create a Verifiable Credential.
  // ===========================================================================

  // Create an account builder with in-memory storage for simplicity.
  // See `create_did` example to configure Stronghold storage.
  let mut builder: AccountBuilder = Account::builder();

  // Create an identity for the issuer.
  let mut issuer: Account = builder.create_identity(IdentitySetup::default()).await?;

  // Add a dedicated verification method to the issuer, with which to sign credentials.
  issuer
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("key-1")
    .apply()
    .await?;

  // Create a new empty revocation bitmap. No credential is revoked yet.
  let revocation_bitmap: RevocationBitmap = RevocationBitmap::new();

  // Add the RevocationBitmap as a service endpoint to allow verfiers to check the credential status.
  issuer
    .update_identity()
    .create_service()
    .fragment("my-revocation-service")
    .type_(RevocationBitmap::TYPE)
    .endpoint(revocation_bitmap.to_endpoint()?)
    .apply()
    .await?;

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": "did:iota:B8DucnzULJ9E8cmaReYoePU2b7UKE9WKxyEVov8tQA7H",
    "name": "Alice",
    "degree": "Bachelor of Science and Arts",
    "GPA": "4.0",
  }))?;

  // Create a credential status pointing verifiers to the endpoint that states if the credential has been revoked.
  // The issuer determines the index of the credential in the revocation list, we chose one arbitrarily.
  let service_url = IotaDIDUrl::join(issuer.did().clone().try_into().unwrap(), "#my-revocation-service")?;
  let credential_index: u32 = 5;
  let status: RevocationBitmapStatus = RevocationBitmapStatus::new(service_url, credential_index);

  // Build credential using subject above, status, and issuer.
  let mut credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer.did().as_str())?)
    .type_("UniversityDegreeCredential")
    .status(status.try_into().unwrap())
    .subject(subject)
    .build()?;

  // Sign the Credential with the issuer's verification method.
  issuer.sign("#key-1", &mut credential, ProofOptions::default()).await?;

  let validation_result = CredentialValidator::validate(
    &credential,
    &issuer.document(),
    &CredentialValidationOptions::default(),
    identity::iota::FailFast::FirstError,
  );

  // The credential wasn't revoked, so we expect the validation to succeed.
  assert!(validation_result.is_ok());

  // ===========================================================================
  // Revocation of the Verifiable Credential.
  // ===========================================================================

  // Update the RevocationBitmap service in the issuer's DID Document.
  // This revokes the credential's unique index.
  issuer
    .revoke_credentials("my-revocation-service", &[credential_index])
    .await?;

  let validation_result = CredentialValidator::validate(
    &credential,
    &issuer.document(),
    &CredentialValidationOptions::default(),
    identity::iota::FailFast::FirstError,
  );

  // We expect validation to no longer succeed because the credential was revoked.
  match validation_result.unwrap_err().validation_errors[0] {
    ValidationError::RevokedCredential(index) => assert_eq!(index, credential_index),
    _ => panic!("expected revoked credential error"),
  }

  // ===========================================================================
  // Alternative revocation of the Verifiable Credential.
  // ===========================================================================

  // By removing the verification method, that signed the credential, from the issuer's DID document,
  // we effectively revoke the credential, as it will no longer be possible to validate the signature.
  issuer
    .update_identity()
    .delete_method()
    .fragment("key-1")
    .apply()
    .await?;

  // We expect the verifiable credential to be revoked.
  let resolver: Resolver = Resolver::new().await?;
  let resolved_issuer_doc: ResolvedIotaDocument = resolver.resolve_credential_issuer(&credential).await?;
  let validation_result = CredentialValidator::validate(
    &credential,
    &resolved_issuer_doc,
    &CredentialValidationOptions::default(),
    identity::iota::FailFast::FirstError,
  );

  println!("VC validation result: {:?}", validation_result);
  assert!(validation_result.is_err());

  println!("Credential successfully revoked!");

  Ok(())
}
