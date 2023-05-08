// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::Duration;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::OrderedSet;
use identity_iota::core::Timestamp;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::DomainLinkageConfiguration;
use identity_iota::credential::DomainLinkageCredentialBuilder;
use identity_iota::credential::DomainLinkageValidationError;
use identity_iota::credential::DomainLinkageValidator;
use identity_iota::credential::Issuer;
use identity_iota::credential::LinkedDomainService;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DIDUrl;
use identity_iota::did::DID;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::resolver::Resolver;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::AliasOutputBuilder;
use iota_sdk::types::block::output::RentStructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Create a DID for the entity that will issue the Domain Linkage Credential.
  let (_, mut did_document, keypair): (Address, IotaDocument, KeyPair) =
    create_did(&client, &mut secret_manager).await?;
  let did: IotaDID = did_document.id().clone();

  // =====================================================
  // Create Linked Domain service
  // =====================================================

  // The DID should be linked to the following domains.
  let domain_1: Url = Url::parse("https://foo.example.com")?;
  let domain_2: Url = Url::parse("https://bar.example.com")?;

  let mut domains: OrderedSet<Url> = OrderedSet::new();
  domains.append(domain_1.clone());
  domains.append(domain_2.clone());

  // Create a Linked Domain Service to enable the discovery of the linked domains through the DID Document.
  // This is optional since it is not a hard requirement by the specs.
  let service_url: DIDUrl = did.clone().join("#domain-linkage")?;
  let linked_domain_service: LinkedDomainService =
    LinkedDomainService::new(service_url, domains, Object::new()).unwrap();
  did_document.insert_service(linked_domain_service.into())?;
  let updated_did_document: IotaDocument = publish_document(client.clone(), secret_manager, did_document).await?;

  println!("DID document with linked domain service: {updated_did_document:#}");

  // =====================================================
  // Create DID Configuration resource
  // =====================================================

  // Now the DID Document contains a service that includes the domains.
  // To allow a bidirectional linkage, the domains must link to the DID. This is
  // done by creating a `DID Configuration Resource` that includes a `Domain Linkage Credential`
  // and can be made available on the domain.

  // Create the Domain Linkage Credential.
  let mut domain_linkage_credential: Credential = DomainLinkageCredentialBuilder::new()
    .issuer(Issuer::Url(updated_did_document.id().to_url().into()))
    .origin(domain_1.clone())
    .issuance_date(Timestamp::now_utc())
    // Expires after a year.
    .expiration_date(Timestamp::now_utc().checked_add(Duration::days(365)).unwrap())
    .build()?;

  // Sign the credential.
  updated_did_document.sign_data(
    &mut domain_linkage_credential,
    keypair.private(),
    "#key-1",
    ProofOptions::default(),
  )?;

  // Create the DID Configuration Resource which wraps the Domain Linkage credential.
  let configuration_resource: DomainLinkageConfiguration =
    DomainLinkageConfiguration::new(vec![domain_linkage_credential]);
  println!("Configuration Resource >>: {configuration_resource:#}");

  // The DID Configuration resource can be made available on `https://foo.example.com/.well-known/did-configuration.json`.
  let configuration_resource_json: String = configuration_resource.to_json()?;

  // Now the DID Document links to the Domains through the service, and the Foo domain links to the DID
  // through the DID Configuration resource. A bidirectional linkage is established.
  // Note however that bidirectionality is not a hard requirement. It is valid to have a Domain Linkage
  // credential point to a DID, without the DID having a service that points back.

  // =====================================================
  // Verification can start from two different places.
  // The first case answers the question "What DID is this domain linked to?"
  // while the second answers "What domain is this DID linked to?".
  // =====================================================

  // Init a resolver for resolving DID Documents.
  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client.clone());

  // =====================================================
  // → Case 1: starting from domain
  // =====================================================
  let domain_foo: Url = domain_1.clone();

  // Fetch the DID Configuration resource
  // let configuration_resource: DomainLinkageConfiguration =
  // DomainLinkageConfiguration::fetch_configuration(domain_foo).await.unwrap();

  // But since the DID Configuration
  // resource isn't available online in this example, we will simply deserialize the JSON.
  let configuration_resource: DomainLinkageConfiguration =
    DomainLinkageConfiguration::from_json(&configuration_resource_json)?;

  // Retrieve the issuers of the Domain Linkage Credentials which correspond to the possibly linked DIDs.
  let linked_dids: Vec<&Url> = configuration_resource.issuers().collect();
  assert_eq!(linked_dids.len(), 1);

  // Resolve the DID Document of the DID that issued the credential.
  let issuer_did_document: IotaDocument = resolver.resolve(&did).await.unwrap();

  // Validate the linkage between the Domain Linkage Credential in the configuration and the provided issuer DID.
  let validation_result: Result<(), DomainLinkageValidationError> = DomainLinkageValidator::validate_linkage(
    &issuer_did_document,
    &configuration_resource,
    &domain_foo,
    &CredentialValidationOptions::default(),
  );
  assert!(validation_result.is_ok());

  // =====================================================
  // → Case 2: starting from a DID
  // =====================================================
  let did_document: IotaDocument = resolver.resolve(&did).await.unwrap();

  // Get the Linked Domain Services from the DID Document.
  let linked_domain_services: Vec<LinkedDomainService> = did_document
    .service()
    .iter()
    .cloned()
    .filter_map(|service| LinkedDomainService::try_from(service).ok())
    .collect();
  assert_eq!(linked_domain_services.len(), 1);

  // Get the domains included in the Linked Domain Service.
  let domains: &[Url] = linked_domain_services.get(0).unwrap().domains();

  let domain_foo: Url = domains.get(0).unwrap().clone();
  assert_eq!(domain_foo, domain_1);

  // Fetch the DID Configuration resource
  // let configuration_resource: DomainLinkageConfiguration =
  // DomainLinkageConfiguration::fetch_configuration(domain_foo).await.unwrap();

  // But since the DID Configuration
  // resource isn't available online in this example, we will simply deserialize the JSON.
  let configuration_resource: DomainLinkageConfiguration =
    DomainLinkageConfiguration::from_json(&configuration_resource_json)?;

  // Validate the linkage.
  let validation_result: Result<(), DomainLinkageValidationError> = DomainLinkageValidator::validate_linkage(
    &did_document,
    &configuration_resource,
    &domain_foo,
    &CredentialValidationOptions::default(),
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
    .finish(client.get_token_supply().await?)?;

  // Publish the updated Alias Output.
  Ok(client.publish_did_output(&secret_manager, alias_output).await.unwrap())
}
