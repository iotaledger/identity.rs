// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodScope;
use identity_stardust::NetworkName;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use identity_stardust::StardustVerificationMethod;

use iota_client::block::address::Address;
use iota_client::block::address::AliasAddress;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::RentStructure;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod ex0_create_did;

/// An example to demonstrate how one identity can control or "own" another identity.
///
/// For this example, we consider the case where a parent company's DID owns the DID of a subsidiary.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  pretty_env_logger::init();

  // Create a new DID for the company.
  let (client, _address, secret_manager, company_document): (Client, Address, SecretManager, StardustDocument) =
    ex0_create_did::run().await?;

  // Obtain the current byte costs and the network name.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let network_name: NetworkName = client.network_name().await?;

  // Create an exemplary DID document for the subsidiary.
  let subsidiary_document: StardustDocument = create_example_did_document(&network_name)?;

  // Create a DID for the subsidiary that is controlled by the parent company's DID.
  // That means the company's Alias Output will have to be unlocked in a transaction that
  // updates the subsidary's Alias Output.
  let subsidiary_alias: AliasOutput = client
    .new_did_output(
      Address::Alias(AliasAddress::new(company_document.id().into())),
      subsidiary_document,
      Some(rent_structure.clone()),
    )
    .await?;

  let alias_output2 = AliasOutputBuilder::from(&subsidiary_alias)
    // Optionally, we can set the company of the new DID as the issuer of the subsidiary DID.
    // This allows to verify trust relationships between DIDs, as
    // a resolver can verify that the subsidiary was issued by the parent company.
    .add_immutable_feature(IssuerFeature::new(AliasAddress::new(company_document.id().into()).into()).into())
    // Adding the issuer feature means we have to re-calculate the required storage deposit.
    .with_minimum_storage_deposit(rent_structure.clone())
    .finish()?;

  // Publish the subsidiary's DID.
  let mut subsidiary_document: StardustDocument = client.publish_did_output(&secret_manager, alias_output2).await?;

  // Add a verification method to the subsidiary.
  // This only serves as an example for updating the subsidiary DID.
  add_verification_method(&mut subsidiary_document)?;

  // Update the subsidiary Alias Output with the updated document
  // and increase the storage deposit.
  let subsidiary_alias: AliasOutput = client.update_did_output(subsidiary_document).await?;
  let subsidiary_alias = AliasOutputBuilder::from(&subsidiary_alias)
    .with_minimum_storage_deposit(rent_structure.clone())
    .finish()?;

  // Publish the updated subsidiary's DID.
  // This works because `secret_manager` can unlock the company's Alias Output,
  // which is required in order to update the subsidiary's Alias Output.
  let subsidiary_document: StardustDocument = client.publish_did_output(&secret_manager, subsidiary_alias).await?;

  // Resolve both DID documents for demonstration purposes.
  let company_document = client.resolve_did(company_document.id()).await?;
  let subsidiary_document = client.resolve_did(subsidiary_document.id()).await?;

  println!("Company: {company_document:#?}");
  println!("Subsidiary: {subsidiary_document:#?}");

  Ok(())
}

fn create_example_did_document(network_name: &NetworkName) -> anyhow::Result<StardustDocument> {
  // Create a new document for the given network with a placeholder DID.
  let mut document: StardustDocument = StardustDocument::new(network_name);

  // Create a new key pair that we'll use to create a verification method.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;

  // Create a new verification method based on the previously created key pair.
  let method: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;

  // Insert the method into the document.
  document.insert_method(method, MethodScope::VerificationMethod)?;

  Ok(document)
}

// TODO: Document.
fn add_verification_method(document: &mut StardustDocument) -> anyhow::Result<()> {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;

  let method: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-2")?;

  document.insert_method(method, MethodScope::VerificationMethod)?;

  Ok(())
}
