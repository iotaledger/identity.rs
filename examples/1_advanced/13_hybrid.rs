// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use examples::get_address_with_funds;
use examples::random_stronghold_path;
use examples::MemStorage;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Duration;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::Timestamp;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::DecodedJwtCredential;
use identity_iota::credential::DecodedJwtPresentation;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidatorHybrid;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::credential::JwtPresentationOptions;
use identity_iota::credential::JwtPresentationValidationOptions;
use identity_iota::credential::JwtPresentationValidatorHybrid;
use identity_iota::credential::JwtPresentationValidatorUtils;
use identity_iota::credential::Presentation;
use identity_iota::credential::PresentationBuilder;
use identity_iota::credential::Subject;
use identity_iota::credential::SubjectHolderRelationship;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::resolver::Resolver;
use identity_iota::storage::JwkDocumentExtHybrid;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::jwk::CompositeAlgId;
use identity_iota::verification::MethodScope;
use identity_pqc_verifier::PQCJwsVerifier;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;
use serde_json::json;

// // The API endpoint of an IOTA node, e.g. Hornet.
// const API_ENDPOINT: &str = "http://localhost";
// // The faucet endpoint allows requesting funds for testing purposes.
// const FAUCET_ENDPOINT: &str = "http://localhost/faucet/api/enqueue";

const API_ENDPOINT: &str = "https://api.testnet.shimmer.network";
const FAUCET_ENDPOINT: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

async fn create_did(
  client: &Client,
  secret_manager: &SecretManager,
  storage: &MemStorage,
  alg_id: CompositeAlgId,
) -> anyhow::Result<(Address, IotaDocument, String)> {
  // Get an address with funds for testing.
  let address: Address = get_address_with_funds(&client, &secret_manager, FAUCET_ENDPOINT).await?;

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // New Verification Method containing a PQC key
  let fragment = document
    .generate_method_hybrid(&storage, alg_id, None, MethodScope::VerificationMethod)
    .await?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {document:#}");

  Ok((address, document, fragment))
}

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
///
/// In this example we connect to a locally running private network, but it can be adapted
/// to run on any IOTA node by setting the network and faucet endpoints.
///
/// See the following instructions on running your own private network
/// https://github.com/iotaledger/hornet/tree/develop/private_tangle
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  let mut secret_manager_issuer = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_1".to_owned()))
      .build(random_stronghold_path())?,
  );

  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let (_, issuer_document, fragment_issuer): (Address, IotaDocument, String) = create_did(
    &client,
    &mut secret_manager_issuer,
    &storage_issuer,
    CompositeAlgId::IdMldsa65Ed25519,
  )
  .await?;

  let mut secret_manager_holder = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_2".to_owned()))
      .build(random_stronghold_path())?,
  );

  let storage_holder: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let (_, holder_document, fragment_holder): (Address, IotaDocument, String) = create_did(
    &client,
    &mut secret_manager_holder,
    &storage_holder,
    CompositeAlgId::IdMldsa65Ed25519,
  )
  .await?;

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": holder_document.id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  let credential_jwt: Jwt = issuer_document
    .create_credential_jwt_hybrid(
      &credential,
      &storage_issuer,
      &fragment_issuer,
      &JwsSignatureOptions::default(),
      None,
    )
    .await?;

  println!("Credential JWT: {}", credential_jwt.as_str());

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  let decoded_credential: DecodedJwtCredential<Object> =
    JwtCredentialValidatorHybrid::with_signature_verifiers(EdDSAJwsVerifier::default(), PQCJwsVerifier::default())
      .validate::<_, Object>(
        &credential_jwt,
        &issuer_document,
        &JwtCredentialValidationOptions::default(),
        FailFast::FirstError,
      )
      .unwrap();

  println!("VC successfully validated");

  println!("Credential JSON > {:#}", decoded_credential.credential);

  // ===========================================================================
  // Step 4: Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
  // ===========================================================================

  // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  // The verifier and holder also agree that the signature should have an expiry date
  // 10 minutes from now.
  let expires: Timestamp = Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap();

  // ===========================================================================
  // Step 5: Holder creates and signs a verifiable presentation from the issued credential.
  // ===========================================================================

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let presentation: Presentation<Jwt> =
    PresentationBuilder::new(holder_document.id().to_url().into(), Default::default())
      .credential(credential_jwt)
      .build()?;

  // Create a JWT verifiable presentation using the holder's verification method
  // and include the requested challenge and expiry timestamp.
  let presentation_jwt: Jwt = holder_document
    .create_presentation_jwt_hybrid(
      &presentation,
      &storage_holder,
      &fragment_holder,
      &JwsSignatureOptions::default().nonce(challenge.to_owned()),
      &JwtPresentationOptions::default().expiration_date(expires),
    )
    .await?;

  // ===========================================================================
  // Step 6: Holder sends a verifiable presentation to the verifier.
  // ===========================================================================
  println!(
    "Sending presentation (as JWT) to the verifier: {}",
    presentation_jwt.as_str()
  );

  // ===========================================================================
  // Step 7: Verifier receives the Verifiable Presentation and verifies it.
  // ===========================================================================

  // The verifier wants the following requirements to be satisfied:
  // - JWT verification of the presentation (including checking the requested challenge to mitigate replay attacks)
  // - JWT verification of the credentials.
  // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
  // - The issuance date must not be in the future.

  let presentation_verifier_options: JwsVerificationOptions =
    JwsVerificationOptions::default().nonce(challenge.to_owned());

  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client);

  // Resolve the holder's document.
  let holder_did: CoreDID = JwtPresentationValidatorUtils::extract_holder(&presentation_jwt)?;
  let holder: IotaDocument = resolver.resolve(&holder_did).await?;

  // Validate presentation. Note that this doesn't validate the included credentials.
  let presentation_validation_options =
    JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);
  let presentation: DecodedJwtPresentation<Jwt> =
    JwtPresentationValidatorHybrid::with_signature_verifiers(EdDSAJwsVerifier::default(), PQCJwsVerifier::default())
      .validate(&presentation_jwt, &holder, &presentation_validation_options)?;

  // Concurrently resolve the issuers' documents.
  let jwt_credentials: &Vec<Jwt> = &presentation.presentation.verifiable_credential;
  let issuers: Vec<CoreDID> = jwt_credentials
    .iter()
    .map(JwtCredentialValidatorUtils::extract_issuer_from_jwt)
    .collect::<Result<Vec<CoreDID>, _>>()?;
  let issuers_documents: HashMap<CoreDID, IotaDocument> = resolver.resolve_multiple(&issuers).await?;

  // Validate the credentials in the presentation.
  let credential_validator =
    JwtCredentialValidatorHybrid::with_signature_verifiers(EdDSAJwsVerifier::default(), PQCJwsVerifier::default());
  let validation_options: JwtCredentialValidationOptions = JwtCredentialValidationOptions::default()
    .subject_holder_relationship(holder_did.to_url().into(), SubjectHolderRelationship::AlwaysSubject);

  for (index, jwt_vc) in jwt_credentials.iter().enumerate() {
    // SAFETY: Indexing should be fine since we extracted the DID from each credential and resolved it.
    let issuer_document: &IotaDocument = &issuers_documents[&issuers[index]];

    let _decoded_credential: DecodedJwtCredential<Object> = credential_validator
      .validate::<_, Object>(jwt_vc, issuer_document, &validation_options, FailFast::FirstError)
      .unwrap();
  }

  // Since no errors were thrown by `verify_presentation` we know that the validation was successful.
  println!("VP successfully validated: {:#?}", presentation.presentation);

  // Note that we did not declare a latest allowed issuance date for credentials. This is because we only want to check
  // that the credentials do not have an issuance date in the future which is a default check.

  Ok(())
}
