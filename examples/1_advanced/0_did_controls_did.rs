// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::iota::block::output::AliasId;
use identity_iota::iota::block::output::UnlockCondition;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::address::AliasAddress;
use iota_sdk::types::block::output::feature::IssuerFeature;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::AliasOutputBuilder;
use iota_sdk::types::block::output::RentStructure;

/// Demonstrates how an identity can control another identity.
///
/// For this example, we consider the case where a parent company's DID controls the DID of a subsidiary.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ========================================================
  // Create the company's and subsidiary's Alias Output DIDs.
  // ========================================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password".to_owned()))
      .build(random_stronghold_path())?,
  );

  // Create a new DID for the company.
  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, company_document, _): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager, &storage_issuer).await?;
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
      Some(rent_structure),
    )
    .await?;

  let subsidiary_alias: AliasOutput = AliasOutputBuilder::from(&subsidiary_alias)
    // Optionally, we can mark the company as the issuer of the subsidiary DID.
    // This allows to verify trust relationships between DIDs, as a resolver can
    // verify that the subsidiary DID was created by the parent company.
    .add_immutable_feature(IssuerFeature::new(AliasAddress::new(AliasId::from(&company_did))))
    // Adding the issuer feature means we have to recalculate the required storage deposit.
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

  // Publish the subsidiary's DID.
  let mut subsidiary_document: IotaDocument = client.publish_did_output(&secret_manager, subsidiary_alias).await?;

  // =====================================
  // Update the subsidiary's Alias Output.
  // =====================================

  // Add a verification method to the subsidiary.
  // This only serves as an example for updating the subsidiary DID.

  let storage_subsidary: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  subsidiary_document
    .generate_method(
      &storage_subsidary,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  // Update the subsidiary's Alias Output with the updated document
  // and increase the storage deposit.
  let subsidiary_alias: AliasOutput = client.update_did_output(subsidiary_document).await?;
  let subsidiary_alias: AliasOutput = AliasOutputBuilder::from(&subsidiary_alias)
    .with_minimum_storage_deposit(rent_structure)
    .finish()?;

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
