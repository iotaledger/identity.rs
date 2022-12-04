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
use identity_iota::credential::DIDConfigurationResource;
use identity_iota::credential::DomainLinkageCredentialBuilder;
use identity_iota::credential::Issuer;
use identity_iota::credential::ValidatorDocument;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::ProofOptions;


use identity_iota::iota::IotaClientExt;

use identity_iota::iota::IotaDIDUrl;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaService;
use identity_iota::resolver::DomainLinkageVerifier;
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

  let origin_1: Url = Url::parse("https://foo.example.com")?;
  let origin_2: Url = Url::parse("https://bar.example.com")?;

  let mut domains: OrderedSet<Url> = OrderedSet::new();
  domains.append(origin_1.clone());
  domains.append(origin_2.clone());
  let service_url: IotaDIDUrl = did_document.id().to_owned().join("#domain-linkage")?;

  let linked_domain_service: IotaService =
    DIDConfigurationResource::create_linked_domain_service(service_url, domains)?;
  did_document.insert_service(linked_domain_service)?;

  let updated_did_document: IotaDocument = publish_document(client.clone(), secret_manager, did_document).await?;
  println!("DID document with linked domain service: {:#}", updated_did_document);

  ////////////////////////////////////////////////////////////
  // Create DID Configuration resource
  ////////////////////////////////////////////////////////////

  // Create DID Configuration resource for the first domain containing a signed Domain Linkage Credential.
  let mut domain_linkage_credential: Credential = DomainLinkageCredentialBuilder::new()
    .issuer(Issuer::Url(updated_did_document.id().to_url().into()))
    .origin(origin_1)
    .issuance_date(Timestamp::now_utc())
     // Expires after a year.
    .expiration_date(Timestamp::now_utc().checked_add(Duration::days(365)).unwrap())
    .build()?;

  updated_did_document.sign_data(
    &mut domain_linkage_credential,
    keypair.private(),
    "#key-1",
    ProofOptions::default(),
  )?;
  let configuration_resource: DIDConfigurationResource = DIDConfigurationResource::new(vec![domain_linkage_credential]);
  println!("Configuration Resource >>: {:#}", configuration_resource);

  // The DID Configuration resource can be made available at `https://foo.example.com/.well-known/did-configuration.json`.
  let configuration_resource_json: String = configuration_resource.to_json()?;

  // Todo: Add a way to extract domains from Linked Domain Service inside a DID Document.

  ////////////////////////////////////////////////////////////
  // Verify DID Configuration resource
  ////////////////////////////////////////////////////////////

  // Create a verifier, for that a resolver is needed to resolve the issuer and verify the credential's signature.
  let mut resolver: Resolver = Resolver::new();
  resolver.attach_iota_handler(client.clone());
  let verifier: DomainLinkageVerifier = DomainLinkageVerifier::new(resolver);

  // The DID Configuration resource can be fetched using
  // let configuration_resource: DIDConfigurationResource = DomainLinkageVerifier::fetch_configuration(Url::parse("https://foo.example.com/".to_owned())?).await?;
  // But since we're working locally, we will simply deserialize the JSON.
  let configuration_resource: DIDConfigurationResource =
    DIDConfigurationResource::from_json(&configuration_resource_json)?;

  // Verify the "fetched" DID Configuration resource.
  let verification_result = verifier.verify_configuration(&configuration_resource).await;

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
  let rent_structure: RentStructure = client.get_rent_structure()?;
  let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_minimum_storage_deposit(rent_structure)
    .finish(client.get_token_supply()?)?;

  // Publish the updated Alias Output.
  Ok(client.publish_did_output(&secret_manager, alias_output).await.unwrap())
}
