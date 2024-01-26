// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;

use identity_iota::core::FromJson;
use identity_iota::core::Object;

use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::status_list_2021::StatusList2021;
use identity_iota::credential::status_list_2021::StatusList2021Credential;
use identity_iota::credential::status_list_2021::StatusList2021CredentialBuilder;
use identity_iota::credential::status_list_2021::StatusList2021Entry;
use identity_iota::credential::status_list_2021::StatusPurpose;

use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;

use identity_iota::credential::FailFast;
use identity_iota::credential::Issuer;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::credential::JwtValidationError;
use identity_iota::credential::Status;
use identity_iota::credential::StatusCheck;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
use identity_iota::iota::IotaDocument;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use serde_json::json;

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
  let (_, issuer_document, fragment_issuer): (Address, IotaDocument, String) =
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

  // Create a new empty status list. No credentials have been revoked yet.
  let status_list: StatusList2021 = StatusList2021::default();

  // Create a status list credential so that the status list can be stored anywhere.
  // The issuer makes this credential available on `http://example.com/credential/status`.
  // For the purposes of this example, the credential will be used directly without fetching.
  let status_list_credential: StatusList2021Credential = StatusList2021CredentialBuilder::new(status_list)
    .purpose(StatusPurpose::Revocation)
    .subject_id(Url::parse("http://example.com/credential/status")?)
    .issuer(Issuer::Url(issuer_document.id().to_url().into()))
    .build()?;

  println!("Status list credential > {status_list_credential:#}");

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
  // The issuer also chooses a unique `StatusList2021` index to be able to revoke it later.
  let credential_index: usize = 420;
  let status: Status = StatusList2021Entry::new(
    status_list_credential.id().cloned().unwrap(),
    status_list_credential.purpose(),
    credential_index,
    None,
  )
  .into();

  // Build credential using subject above, status, and issuer.
  let mut credential: Credential = CredentialBuilder::default()
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

  // The validator has no way of retriving the status list to check for the
  // revocation of the credential. Let's skip that pass and perform the operation manually.
  let mut validation_options = JwtCredentialValidationOptions::default();
  validation_options.status = StatusCheck::SkipUnsupported;
  // Validate the credential's signature using the issuer's DID Document.
  validator.validate::<_, Object>(
    &credential_jwt,
    &issuer_document,
    &validation_options,
    FailFast::FirstError,
  )?;
  // Check manually for revocation
  JwtCredentialValidatorUtils::check_status_with_status_list_2021(
    &credential,
    &status_list_credential,
    StatusCheck::Strict,
  )?;
  println!("Credential is valid.");

  let status_list_credential_json = status_list_credential.to_json().unwrap();

  // ===========================================================================
  // Revocation of the Verifiable Credential.
  // ===========================================================================

  // At a later time, the issuer university found out that Alice cheated in her final exam.
  // The issuer will revoke Alice's credential.

  // The issuer retrieves the status list credential.
  let mut status_list_credential =
    serde_json::from_str::<StatusList2021Credential>(status_list_credential_json.as_str()).unwrap();

  // Set the value of the chosen index entry to true to revoke the credential
  status_list_credential.set_credential_status(&mut credential, credential_index, true)?;

  // validate the credential and check for revocation
  validator.validate::<_, Object>(
    &credential_jwt,
    &issuer_document,
    &validation_options,
    FailFast::FirstError,
  )?;
  let revocation_result = JwtCredentialValidatorUtils::check_status_with_status_list_2021(
    &credential,
    &status_list_credential,
    StatusCheck::Strict,
  );

  assert!(revocation_result.is_err_and(|e| matches!(e, JwtValidationError::Revoked)));
  println!("The credential has been successfully revoked.");

  Ok(())
}
