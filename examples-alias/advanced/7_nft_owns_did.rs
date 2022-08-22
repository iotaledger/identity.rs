// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use bee_block::address::NftAddress;
// use bee_block::output::AliasOutput;
use identity_stardust::NetworkName;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;

use identity_stardust::block::address::NftAddress;
use identity_stardust::block::output::AliasOutput;
use identity_stardust::StardustIdentityClientExt;
use iota_client::api_types::responses::OutputResponse;
use iota_client::block::address::Address;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::NftId;
use iota_client::block::output::NftOutput;
use iota_client::block::output::NftOutputBuilder;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::output::RentStructure;
use iota_client::block::output::UnlockCondition;
use iota_client::block::payload::transaction::TransactionEssence;
use iota_client::block::payload::Payload;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;
use utils::get_address_with_funds;
use utils::NETWORK_ENDPOINT;

/// Demonstrate how an identity can issue and own NFTs,
/// and how observers can verify the issuer of the NFT.
///
/// For this example, we consider the case where a car's NFT owns
/// the DID of the car.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ==========================================
  // Create the car's DID and the DPP.
  // ==========================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(NETWORK_ENDPOINT, None)?.finish()?;

  // Get an address and a secret manager with funds for testing.
  let (address, secret_manager): (Address, SecretManager) = get_address_with_funds(&client).await?;

  // Get the current byte cost.
  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // Create the car NFT with an Ed25519 address as the unlock condition.
  let car_nft: NftOutput = NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure.clone(), NftId::null())?
    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    .finish()?;

  // Publish all outputs.
  let block: Block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![car_nft.into()])?
    .finish()
    .await?;
  let _ = client.retry_until_included(&block.id(), None, None).await?;

  let car_nft_id: NftId = NftId::from(get_nft_output_id(
    block
      .payload()
      .ok_or_else(|| anyhow::anyhow!("expected the block to contain a payload"))?,
  )?);

  // TODO: Use network_name in all examples.
  let network: NetworkName = client.network_name().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  // TODO: Add methods.
  let document: StardustDocument = StardustDocument::new(&network);

  // Create a new DID for the car that is owned by the car NFT.
  let car_did_output: AliasOutput = client
    .new_did_output(Address::Nft(car_nft_id.into()), document, Some(rent_structure))
    .await?;

  let car_document: StardustDocument = client.publish_did_output(&secret_manager, car_did_output).await?;

  // ==========================================
  // Resolve the car's DID and the owning NFT.
  // ==========================================

  let output: AliasOutput = client.resolve_did_output(car_document.id()).await?;

  let unlock = output
    .unlock_conditions()
    .iter()
    .next()
    .ok_or_else(|| anyhow::anyhow!("expected at least one unlock condition"))?;

  let car_nft_address: NftAddress =
    if let UnlockCondition::StateControllerAddress(state_controller_unlock_condition) = unlock {
      if let Address::Nft(nft_address) = state_controller_unlock_condition.address() {
        *nft_address
      } else {
        anyhow::bail!("expected an NFT address as the unlock condition");
      }
    } else {
      anyhow::bail!("expected an Address as the unlock condition");
    };

  let car_nft_id: &NftId = car_nft_address.nft_id();

  let output_id: OutputId = client.nft_output_id(*car_nft_id).await?;
  let output_response: OutputResponse = client.get_output(&output_id).await?;
  let output: Output = Output::try_from(&output_response.output)?;

  let car_nft: NftOutput = if let Output::Nft(nft_output) = output {
    nft_output
  } else {
    anyhow::bail!("expected an NFT output");
  };

  println!("The car's DID is: {car_document:#?}");
  println!("The car's NFT, which owns its DID, is: {car_nft:#?}");

  Ok(())
}

// Helper function to get the output id for the first NFT output in a Block.
fn get_nft_output_id(payload: &Payload) -> anyhow::Result<OutputId> {
  match payload {
    Payload::Transaction(tx_payload) => {
      let TransactionEssence::Regular(regular) = tx_payload.essence();
      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Nft(_nft_output) = output {
          return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
        }
      }
      anyhow::bail!("no NFT output in transaction essence")
    }
    _ => anyhow::bail!("No transaction payload"),
  }
}
