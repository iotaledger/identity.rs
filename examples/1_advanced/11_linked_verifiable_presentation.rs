// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::OrderedSet;
use identity_iota::core::Url;
use identity_iota::credential::CompoundJwtPresentationValidationError;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::DecodedJwtPresentation;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtPresentationOptions;
use identity_iota::credential::JwtPresentationValidationOptions;
use identity_iota::credential::JwtPresentationValidator;
use identity_iota::credential::JwtPresentationValidatorUtils;
use identity_iota::credential::LinkedVerifiablePresentationService;
use identity_iota::credential::PresentationBuilder;
use identity_iota::credential::Subject;
use identity_iota::did::CoreDID;
use identity_iota::did::DIDUrl;
use identity_iota::did::DID;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
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
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;
  let stronghold_path = random_stronghold_path();

  println!("Using stronghold path: {stronghold_path:?}");
  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password".to_owned()))
      .build(stronghold_path)?,
  );

  // Create a DID for the entity that will be the holder of the Verifiable Presentation.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, mut did_document, fragment): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager, &storage).await?;
  let did: IotaDID = did_document.id().clone();

  // =====================================================
  // Create Linked Verifiable Presentation service
  // =====================================================

  // The DID should link to the following VPs.
  let verifiable_presentation_url_1: Url = Url::parse("https://foo.example.com/verifiable-presentation.jwt")?;
  let verifiable_presentation_url_2: Url = Url::parse("https://bar.example.com/verifiable-presentation.jsonld")?;

  let mut verifiable_presentation_urls: OrderedSet<Url> = OrderedSet::new();
  verifiable_presentation_urls.append(verifiable_presentation_url_1.clone());
  verifiable_presentation_urls.append(verifiable_presentation_url_2.clone());

  // Create a Linked Verifiable Presentation Service to enable the discovery of the linked VPs through the DID Document.
  // This is optional since it is not a hard requirement by the specs.
  let service_url: DIDUrl = did.clone().join("#linked-vp")?;
  let linked_verifiable_presentation_service =
    LinkedVerifiablePresentationService::new(service_url, verifiable_presentation_urls, Object::new())?;
  did_document.insert_service(linked_verifiable_presentation_service.into())?;
  let updated_did_document: IotaDocument = publish_document(client.clone(), secret_manager, did_document).await?;

  println!("DID document with linked verifiable presentation service: {updated_did_document:#}");

  // =====================================================
  // Verification
  // =====================================================

  // Resolve the DID Document of the DID that issued the credential.
  let did_document: IotaDocument = client.resolve(&did).await?;

  // Get the Linked Verifiable Presentation Services from the DID Document.
  let linked_verifiable_presentation_services: Vec<LinkedVerifiablePresentationService> = did_document
    .service()
    .iter()
    .cloned()
    .filter_map(|service| LinkedVerifiablePresentationService::try_from(service).ok())
    .collect();
  assert_eq!(linked_verifiable_presentation_services.len(), 1);

  // Get the VPs included in the service.
  let _verifiable_presentation_urls: &[Url] = linked_verifiable_presentation_services
    .first()
    .ok_or_else(|| anyhow::anyhow!("expected verifiable presentation urls"))?
    .verifiable_presentation_urls();

  // Fetch the verifiable presentation from the URL (for example using `reqwest`).
  // But since the URLs do not point to actual online resource, we will simply create an example JWT.
  let presentation_jwt: Jwt = make_vp_jwt(&did_document, &storage, &fragment).await?;

  // Resolve the holder's document.
  let holder_did: CoreDID = JwtPresentationValidatorUtils::extract_holder(&presentation_jwt)?;
  let holder: IotaDocument = client.resolve(&holder_did).await?;

  // Validate linked presentation. Note that this doesn't validate the included credentials.
  let presentation_verifier_options: JwsVerificationOptions = JwsVerificationOptions::default();
  let presentation_validation_options =
    JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);
  let validation_result: Result<DecodedJwtPresentation<Jwt>, CompoundJwtPresentationValidationError> =
    JwtPresentationValidator::with_signature_verifier(EdDSAJwsVerifier::default()).validate(
      &presentation_jwt,
      &holder,
      &presentation_validation_options,
    );

  assert!(validation_result.is_ok());

  Ok(())
}

async fn publish_document(
  client: Client,
  secret_manager: SecretManager,
  document: IotaDocument,
) -> anyhow::Result<IotaDocument> {
  // Resolve the latest output and update it with the given document.
  let alias_output: AliasOutput = client.update_did_output(document.clone()).await?;

  // Because the size of the DID document increased, we have to increase the allocated storage deposit.
  // This increases the deposit amount to the new minimum.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the updated Alias Output.
  Ok(client.publish_did_output(&secret_manager, alias_output).await?)
}

async fn make_vp_jwt(did_doc: &IotaDocument, storage: &MemStorage, fragment: &str) -> anyhow::Result<Jwt> {
  // first we create a credential encoding it as jwt
  let credential = CredentialBuilder::new(Object::default())
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(did_doc.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(Subject::from_json_value(serde_json::json!({
      "id": did_doc.id().as_str(),
      "name": "Alice",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts",
      },
      "GPA": "4.0",
    }))?)
    .build()?;
  let credential = did_doc
    .create_credential_jwt(&credential, storage, fragment, &JwsSignatureOptions::default(), None)
    .await?;
  // then we create a presentation including the just created JWT encoded credential.
  let presentation = PresentationBuilder::new(Url::parse(did_doc.id().as_str())?, Object::default())
    .credential(credential)
    .build()?;
  // we encode the presentation as JWT
  did_doc
    .create_presentation_jwt(
      &presentation,
      storage,
      fragment,
      &JwsSignatureOptions::default(),
      &JwtPresentationOptions::default(),
    )
    .await
    .context("jwt presentation failed")
}
