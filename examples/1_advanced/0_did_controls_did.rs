// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::MethodScope;
use identity_iota::iota::block::output::AliasId;
use identity_iota::iota::block::output::UnlockCondition;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::iota::NetworkName;
use iota_client::block::address::Address;
use iota_client::block::address::AliasAddress;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::RentStructure;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Demonstrates how an identity can control another identity.
///
/// For this example, we consider the case where a parent company's DID controls the DID of a subsidiary.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ========================================================
  // Create the company's and subsidiary's Alias Output DIDs.
  // ========================================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Create a new DID for the company.
  let (_, company_document, _): (Address, IotaDocument, KeyPair) = create_did(&client, &mut secret_manager).await?;
  let company_did = company_document.id().clone();

  // Get the current byte costs and network name.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let network_name: NetworkName = client.network_name().await?;

  // Construct a new DID document for the subsidiary.
  let subsidiary_document: IotaDocument = IotaDocument::new(&network_name);

  // Create a DID for the subsidiary that is controlled by the parent company's DID.
  // This means the subsidiary's Alias Output can only be updated or destroyed by
  // the state controller or governor of the company's Alias Output respectively.
  let subsidiary_alias: AliasOutput = client
    .new_did_output(
      Address::Alias(AliasAddress::new(AliasId::from(&company_did))),
      subsidiary_document,
      Some(rent_structure.clone()),
    )
    .await?;

  let subsidiary_alias: AliasOutput = AliasOutputBuilder::from(&subsidiary_alias)
    // Optionally, we can mark the company as the issuer of the subsidiary DID.
    // This allows to verify trust relationships between DIDs, as a resolver can
    // verify that the subsidiary DID was created by the parent company.
    .add_immutable_feature(IssuerFeature::new(AliasAddress::new(AliasId::from(&company_did)).into()).into())
    // Adding the issuer feature means we have to recalculate the required storage deposit.
    .with_minimum_storage_deposit(rent_structure.clone())
    .finish(client.get_token_supply().await?)?;

  // Publish the subsidiary's DID.
  let mut subsidiary_document: IotaDocument = client.publish_did_output(&secret_manager, subsidiary_alias).await?;

  // =====================================
  // Update the subsidiary's Alias Output.
  // =====================================

  // Add a verification method to the subsidiary.
  // This only serves as an example for updating the subsidiary DID.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod = IotaVerificationMethod::new(
    subsidiary_document.id().clone(),
    keypair.type_(),
    keypair.public(),
    "#key-2",
  )?;
  subsidiary_document.insert_method(method, MethodScope::VerificationMethod)?;

  // Update the subsidiary's Alias Output with the updated document
  // and increase the storage deposit.
  let subsidiary_alias: AliasOutput = client.update_did_output(subsidiary_document).await?;
  let subsidiary_alias: AliasOutput = AliasOutputBuilder::from(&subsidiary_alias)
    .with_minimum_storage_deposit(rent_structure.clone())
    .finish(client.get_token_supply().await?)?;

  // Publish the updated subsidiary's DID.
  //
  // This works because `secret_manager` can unlock the company's Alias Output,
  // which is required in order to update the subsidiary's Alias Output.
  let subsidiary_document: IotaDocument = client.publish_did_output(&secret_manager, subsidiary_alias).await?;

  // ===================================================================
  // Determine the controlling company's DID given the subsidiary's DID.
  // ===================================================================

  // Resolve the subsidiary's Alias Output.
  let subsidiary_output: AliasOutput = client.resolve_did_output(subsidiary_document.id()).await?;

  // Extract the company's Alias Id from the state controller unlock condition.
  //
  // If instead we wanted to determine the original creator of the DID,
  // we could inspect the issuer feature. This feature needs to be set when creating the DID.
  let company_alias_id: AliasId = if let Some(UnlockCondition::StateControllerAddress(address)) =
    subsidiary_output.unlock_conditions().iter().next()
  {
    if let Address::Alias(alias) = *address.address() {
      *alias.alias_id()
    } else {
      anyhow::bail!("expected an alias address as the state controller");
    }
  } else {
    anyhow::bail!("expected two unlock conditions");
  };

  // Reconstruct the company's DID from the Alias Id and the network.
  let company_did = IotaDID::new(company_alias_id.deref(), &network_name);

  // Resolve the company's DID document.
  let company_document: IotaDocument = client.resolve_did(&company_did).await?;

  println!("Company: {company_document:#}");
  println!("Subsidiary: {subsidiary_document:#}");

  Ok(())
}
