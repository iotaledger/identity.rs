// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to revoke a verifiable credential.
//! It demonstrates two methods for revocation. The first uses a revocation bitmap of type `RevocationBitmap2022`,
//! while the second method simply removes the verification method (public key) that signed the credential
//! from the DID Document of the issuer.
//!
//! Note: make sure `API_ENDPOINT` and `FAUCET_ENDPOINT` are set to the correct network endpoints.
//!
//! cargo run --example 7_revoke_vc

use anyhow::anyhow;
use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::Url;
use identity_iota::credential::CompoundCredentialValidationError;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::DecodedJwtCredential;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::credential::JwtValidationError;
use identity_iota::credential::RevocationBitmap;
use identity_iota::credential::RevocationBitmapStatus;
use identity_iota::credential::Status;
use identity_iota::credential::Subject;
use identity_iota::did::DIDUrl;
use identity_iota::did::DID;
use identity_iota::document::Service;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::prelude::IotaDID;
use identity_iota::resolver::Resolver;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::AliasOutputBuilder;
use iota_sdk::types::block::output::RentStructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ===========================================================================
  // Create a Verifiable Credential.
  // ===========================================================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  let mut secret_manager_issuer: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_1".to_owned()))
      .build(random_stronghold_path())?,
  );

  // Create an identity for the issuer with one verification method `key-1`.
  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, mut issuer_document, fragment_issuer): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_issuer, &storage_issuer).await?;

  // Create an identity for the holder, in this case also the subject.
  let mut secret_manager_alice: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_2".to_owned()))
      .build(random_stronghold_path())?,
  );
  let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, alice_document, _): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_alice, &storage_alice).await?;

  // Create a new empty revocation bitmap. No credential is revoked yet.
  let revocation_bitmap: RevocationBitmap = RevocationBitmap::new();

  // Add the revocation bitmap to the DID document of the issuer as a service.
  let service_id: DIDUrl = issuer_document.id().to_url().join("#my-revocation-service")?;
  let service: Service = revocation_bitmap.to_service(service_id)?;

  assert!(issuer_document.insert_service(service).is_ok());

  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did_output(issuer_document.clone()).await?;

  // Because the size of the DID document increased, we have to increase the allocated storage deposit.
  // This increases the deposit amount to the new minimum.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the updated Alias Output.
  issuer_document = client.publish_did_output(&secret_manager_issuer, alias_output).await?;

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": alice_document.id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Create an unsigned `UniversityDegree` credential for Alice.
  // The issuer also chooses a unique `RevocationBitmap` index to be able to revoke it later.
  let service_url = issuer_document.id().to_url().join("#my-revocation-service")?;
  let credential_index: u32 = 5;
  let status: Status = RevocationBitmapStatus::new(service_url, credential_index).into();

  // Build credential using subject above, status, and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .status(status)
    .subject(subject)
    .build()?;

  println!("Credential JSON > {credential:#}");

  let credential_jwt: Jwt = issuer_document
    .create_credential_jwt(
      &credential,
      &storage_issuer,
      &fragment_issuer,
      &JwsSignatureOptions::default(),
      None,
    )
    .await?;

  let validator: JwtCredentialValidator<EdDSAJwsVerifier> =
    JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default());
  // Validate the credential's signature using the issuer's DID Document.
  validator.validate::<_, Object>(
    &credential_jwt,
    &issuer_document,
    &JwtCredentialValidationOptions::default(),
    FailFast::FirstError,
  )?;

  // ===========================================================================
  // Revocation of the Verifiable Credential.
  // ===========================================================================

  // Update the RevocationBitmap service in the issuer's DID Document.
  // This revokes the credential's unique index.
  issuer_document.revoke_credentials("my-revocation-service", &[credential_index])?;

  // Publish the changes.
  let alias_output: AliasOutput = client.update_did_output(issuer_document.clone()).await?;
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;
  issuer_document = client.publish_did_output(&secret_manager_issuer, alias_output).await?;

  let validation_result: std::result::Result<DecodedJwtCredential, CompoundCredentialValidationError> = validator
    .validate(
      &credential_jwt,
      &issuer_document,
      &JwtCredentialValidationOptions::default(),
      FailFast::FirstError,
    );

  // We expect validation to no longer succeed because the credential was revoked.
  assert!(matches!(
    validation_result.unwrap_err().validation_errors[0],
    JwtValidationError::Revoked
  ));

  // ===========================================================================
  // Alternative revocation of the Verifiable Credential.
  // ===========================================================================

  // By removing the verification method, that signed the credential, from the issuer's DID document,
  // we effectively revoke the credential, as it will no longer be possible to validate the signature.
  let original_method: DIDUrl = issuer_document
    .resolve_method(&fragment_issuer, None)
    .ok_or_else(|| anyhow!("expected method to exist"))?
    .id()
    .clone();
  issuer_document
    .remove_method(&original_method)
    .ok_or_else(|| anyhow!("expected method to exist"))?;

  // Publish the changes.
  let alias_output: AliasOutput = client.update_did_output(issuer_document.clone()).await?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output).finish()?;
  client.publish_did_output(&secret_manager_issuer, alias_output).await?;

  // We expect the verifiable credential to be revoked.
  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client);
  let resolved_issuer_did: IotaDID = JwtCredentialValidatorUtils::extract_issuer_from_jwt(&credential_jwt)?;
  let resolved_issuer_doc: IotaDocument = resolver.resolve(&resolved_issuer_did).await?;

  let validation_result = validator.validate::<_, Object>(
    &credential_jwt,
    &resolved_issuer_doc,
    &JwtCredentialValidationOptions::default(),
    FailFast::FirstError,
  );

  println!("VC validation result: {validation_result:?}");
  assert!(validation_result.is_err());

  println!("Credential successfully revoked!");

  Ok(())
}
