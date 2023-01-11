// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::Duration;
use identity_iota::core::FromJson;
use identity_iota::core::OrderedSet;
use identity_iota::core::Timestamp;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::DomainLinkageConfiguration;
use identity_iota::credential::DomainLinkageCredentialBuilder;
use identity_iota::credential::DomainLinkageUtils;

use identity_iota::credential::Issuer;

use identity_iota::crypto::KeyPair;
use identity_iota::crypto::ProofOptions;

use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;

use identity_iota::did::Document;
use identity_iota::did::Service;
use identity_iota::did::DID;
use identity_iota::iota::IotaDIDUrl;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaService;
use identity_iota::resolver::Resolver;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::RentStructure;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

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

  // Create a DID for an entity.
  let (_, mut did_document, keypair): (Address, IotaDocument, KeyPair) =
    create_did(&client, &mut secret_manager).await?;
  let did: IotaDID = did_document.id().clone();

  ////////////////////////////////////////////////////////////
  // Create Linked Domain service
  ////////////////////////////////////////////////////////////

  // The DID should be linked to the following domains.
  let domain_1: Url = Url::parse("https://foo.example.com")?;
  let domain_2: Url = Url::parse("https://bar.example.com")?;

  let mut domains: OrderedSet<Url> = OrderedSet::new();
  domains.append(domain_1.clone());
  domains.append(domain_2.clone());

  // Create a Linked Domain Service to enable the discovery of the linked domains and add it to the DID Document.
  let service_url: IotaDIDUrl = did.clone().join("#domain-linkage")?;
  let domain_linkage_service: IotaService = DomainLinkageUtils::create_linked_domain_service(service_url, domains)?;
  did_document.insert_service(domain_linkage_service)?;
  let updated_did_document: IotaDocument = publish_document(client.clone(), secret_manager, did_document).await?;

  println!("DID document with linked domain service: {:#}", updated_did_document);

  ////////////////////////////////////////////////////////////
  // Create DID Configuration resource
  ////////////////////////////////////////////////////////////

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
  println!("Configuration Resource >>: {:#}", configuration_resource);

  // The DID Configuration resource can be made available on `https://foo.example.com/.well-known/did-configuration.json`.
  let configuration_resource_json: String = configuration_resource.to_json()?;

  // Now the DID Document links to the Domains through the service, and first domain links to the DID
  // through the DID Configuration resource. A bidirectional linkage is established.

  ////////////////////////////////////////////////////////////
  // Resolve and verify the DID Configuration resource
  ////////////////////////////////////////////////////////////

  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client.clone());

  // Resolve the DID Document.
  let resolved_did_document: IotaDocument = resolver.resolve(&did).await?;
  let service_url: IotaDIDUrl = did.clone().join("#domain-linkage")?;
  let linked_domain_service: &Service<IotaDID> = resolved_did_document.resolve_service(service_url).unwrap();

  // Verify the structure of the Linked Domain service and extract the domains from it.
  let domains: Vec<Url> = DomainLinkageUtils::extract_domains(linked_domain_service)?;
  assert_eq!(domains, vec![domain_1.clone(), domain_2.clone()]);

  // Fetch the configuration resource of the first domain.
  // let configuration_resource = DomainLinkageConfiguration::fetch_configuration(domains[0].clone()).await?;
  // But since we're working locally, we will simply deserialize the JSON.
  let configuration_resource: DomainLinkageConfiguration =
    DomainLinkageConfiguration::from_json(&configuration_resource_json)?;

  // Verify the "fetched" DID Configuration resource.
  let verification_result = resolver
    .verify_domain_linkage_configuration(
      &configuration_resource,
      domain_1.clone(),
      &CredentialValidationOptions::default(),
    )
    .await;

  // Verification succeeds.
  assert!(verification_result.is_ok());
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
