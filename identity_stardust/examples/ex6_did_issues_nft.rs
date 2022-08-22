// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;

use identity_stardust::StardustIdentityClientExt;
use iota_client::api_types::responses::OutputResponse;
use iota_client::block::address::Address;
use iota_client::block::address::AliasAddress;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::Feature;
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

mod ex0_create_did;

/// Demonstrate how an identity can issue and own NFTs,
/// and how observers can verify the issuer of the NFT.
///
/// For this example, we consider the case where a manufacturer issues
/// a digital product passport (DPP) as an NFT.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ==========================================
  // Create the manufacturer's DID and the DPP.
  // ==========================================

  // Create a new DID for the manufacturer.
  let (client, _address, secret_manager, manufacturer_did): (Client, Address, SecretManager, StardustDID) =
    ex0_create_did::run().await?;

  // Get the current byte cost.
  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // Create the digital product passport NFT with the manufacturer set as the immutable issuer.
  let product_passport_nft: NftOutput =
    create_nft(rent_structure, AliasAddress::new(AliasId::from(&manufacturer_did)))?;

  // Publish all outputs.
  let block: Block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![product_passport_nft.into()])?
    .finish()
    .await?;
  let _ = client.retry_until_included(&block.id(), None, None).await?;

  // ==========================================
  // Resolve the DPP and its issuer.
  // ==========================================

  let nft_id: NftId = NftId::from(get_nft_output_id(
    block
      .payload()
      .ok_or_else(|| anyhow::anyhow!("expected block to contain a payload"))?,
  )?);

  let nft_output_id: OutputId = client.nft_output_id(nft_id).await?;
  let output_response: OutputResponse = client.get_output(&nft_output_id).await?;
  let output: Output = Output::try_from(&output_response.output)?;

  let nft_output: NftOutput = if let Output::Nft(nft_output) = output {
    nft_output
  } else {
    anyhow::bail!("expected NFT output")
  };

  let issuer_address: Address = if let Some(Feature::Issuer(issuer)) = nft_output.immutable_features().iter().next() {
    *issuer.address()
  } else {
    anyhow::bail!("expected an issuer feature")
  };

  let alias_id: AliasId = if let Address::Alias(alias_address) = issuer_address {
    *alias_address.alias_id()
  } else {
    anyhow::bail!("expected an Alias Address")
  };

  let network: NetworkName = NetworkName::try_from(client.get_bech32_hrp().await?)?;
  let did: StardustDID = StardustDID::new(&*alias_id, &network);

  // Resolve the issuer of the DPP.
  let issuer_document: StardustDocument = client.resolve_did(&did).await?;

  println!("The issuer of the DPP is: {issuer_document:#?}");

  Ok(())
}

/// Creates an example NFT that is issued and owned by `nft_issuer`.
fn create_nft(rent_structure: RentStructure, nft_issuer: AliasAddress) -> anyhow::Result<NftOutput> {
  NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, NftId::null())?
    // The NFT will initially be owned by the issuer.
    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(Address::Alias(
      nft_issuer,
    ))))
    // Set the `nft_issuer` as the immutable issuer of the NFT.
    .add_immutable_feature(Feature::Issuer(IssuerFeature::new(Address::Alias(nft_issuer))))
    .finish()
    .map_err(Into::into)
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
