use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;
use identity_iota::core::OrderedSet;
use identity_iota::core::Url;
use identity_iota::credential::CompoundJwtPresentationValidationError;
use identity_iota::credential::DecodedJwtPresentation;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtPresentationValidationOptions;
use identity_iota::credential::JwtPresentationValidator;
use identity_iota::credential::JwtPresentationValidatorUtils;
use identity_iota::credential::LinkedVerifiablePresentationService;
use identity_iota::did::CoreDID;
use identity_iota::did::DIDUrl;
use identity_iota::did::DID;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::resolver::Resolver;
use identity_iota::storage::JwkMemStore;
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
  let (_, mut did_document, _): (Address, IotaDocument, String) =
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
  let linked_verifiable_presentation_service: LinkedVerifiablePresentationService =
    LinkedVerifiablePresentationService::new(service_url, verifiable_presentation_urls, Object::new())?;
  did_document.insert_service(linked_verifiable_presentation_service.into())?;
  let updated_did_document: IotaDocument = publish_document(client.clone(), secret_manager, did_document).await?;

  println!("DID document with linked verifiable presentation service: {updated_did_document:#}");

  // =====================================================
  // Verification
  // =====================================================

  // Init a resolver for resolving DID Documents.
  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client.clone());

  // Resolve the DID Document of the DID that issued the credential.
  let did_document: IotaDocument = resolver.resolve(&did).await?;

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
  // But since the URLs are not actually online in this example, we will simply create an example JWT.
  let presentation_jwt: Jwt = example_vp();

  // Resolve the holder's document.
  let holder_did: CoreDID = JwtPresentationValidatorUtils::extract_holder(&presentation_jwt)?;
  let holder: IotaDocument = resolver.resolve(&holder_did).await?;

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

  // TODO: Validate the credentials in the presentation ...

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

/// A static VP, without nonce and expiry (created using basic example `6_create_vp.rs`).
fn example_vp() -> Jwt {
  Jwt::from("eyJraWQiOiJkaWQ6aW90YTpybXM6MHg2Y2I5MWUyMGMxMzhhMTQ1MTUzMDY4ZTEwODNhMGEyYTUwYjU2ZDI1MGI3YjUzYzYwYmEzOTI4NGJkMWRjNzQxI1k5TUppd1k4U0s0NjlNcW0weXBZYzRYSUl5TnVpMzZIejdreVdkUEkyejQiLCJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpc3MiOiJkaWQ6aW90YTpybXM6MHg2Y2I5MWUyMGMxMzhhMTQ1MTUzMDY4ZTEwODNhMGEyYTUwYjU2ZDI1MGI3YjUzYzYwYmEzOTI4NGJkMWRjNzQxIiwibmJmIjoxNzI0Njg0NTEzLCJ2cCI6eyJAY29udGV4dCI6Imh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwidHlwZSI6IlZlcmlmaWFibGVQcmVzZW50YXRpb24iLCJ2ZXJpZmlhYmxlQ3JlZGVudGlhbCI6WyJleUpyYVdRaU9pSmthV1E2YVc5MFlUcHliWE02TUhnek5ERTVNek5qWW1ReU1USm1NalkzT1dOaE5tSTFZbU00WXpGbE5EazROREV4WlRWaFpHUXdZMk0zWkRnNU1qZzVZV1ppTmpaaVpEZGtNMk5tTWpObEkyRlZkMTgyZGw5bGRUQlJlVFJQWWtOTFIwUlJNWFJEUW00elJGcGxVVzUxYmtkNlRWWlJlRWxrYjJNaUxDSjBlWEFpT2lKS1YxUWlMQ0poYkdjaU9pSkZaRVJUUVNKOS5leUpwYzNNaU9pSmthV1E2YVc5MFlUcHliWE02TUhnek5ERTVNek5qWW1ReU1USm1NalkzT1dOaE5tSTFZbU00WXpGbE5EazROREV4WlRWaFpHUXdZMk0zWkRnNU1qZzVZV1ppTmpaaVpEZGtNMk5tTWpObElpd2libUptSWpveE56STBOamcwTlRFekxDSnFkR2tpT2lKb2RIUndjem92TDJWNFlXMXdiR1V1WldSMUwyTnlaV1JsYm5ScFlXeHpMek0zTXpJaUxDSnpkV0lpT2lKa2FXUTZhVzkwWVRweWJYTTZNSGcyWTJJNU1XVXlNR014TXpoaE1UUTFNVFV6TURZNFpURXdPRE5oTUdFeVlUVXdZalUyWkRJMU1HSTNZalV6WXpZd1ltRXpPVEk0TkdKa01XUmpOelF4SWl3aWRtTWlPbnNpUUdOdmJuUmxlSFFpT2lKb2RIUndjem92TDNkM2R5NTNNeTV2Y21jdk1qQXhPQzlqY21Wa1pXNTBhV0ZzY3k5Mk1TSXNJblI1Y0dVaU9sc2lWbVZ5YVdacFlXSnNaVU55WldSbGJuUnBZV3dpTENKVmJtbDJaWEp6YVhSNVJHVm5jbVZsUTNKbFpHVnVkR2xoYkNKZExDSmpjbVZrWlc1MGFXRnNVM1ZpYW1WamRDSTZleUpqWlhKMGFXWnBZMkYwWlNJNmV5SjBlWEJsSWpvaVFXTmpjbVZrYVhSaGRHbHZiaUlzSW14bGRtVnNJam96ZlgxOWZRLm5ldEpyMkZEaWlPYmRFVWVaaTkwcW90dG9BcFlYLVhacmxMQ0ZwOTA2RHZCMlJUbEw2WDVWb3JhYy1reFpNUThwMkpIUEZMbUk5ZzM5c3NuSG1MWkNnIl19fQ.vcpp_imMMv6inSOy9L-IsvF_WPfEYsuTpcPfEAHQfrBJ_O_zhZxZ0pzcbbvwJqh-wcmMgas0DuR_0NGcZK8CAw".to_string())
}
