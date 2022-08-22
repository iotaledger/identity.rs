// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;

use identity_stardust::block::output::Output;
use identity_stardust::block::output::OutputId;
use identity_stardust::StardustIdentityClientExt;
use iota_client::api_types::responses::OutputResponse;
use iota_client::block::address::Address;
use iota_client::block::address::AliasAddress;
use iota_client::block::output::unlock_condition::ImmutableAliasAddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::FoundryId;
use iota_client::block::output::FoundryOutput;
use iota_client::block::output::FoundryOutputBuilder;
use iota_client::block::output::NativeToken;
use iota_client::block::output::RentStructure;
use iota_client::block::output::SimpleTokenScheme;
use iota_client::block::output::TokenId;
use iota_client::block::output::TokenScheme;
use iota_client::block::output::UnlockCondition;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;
use primitive_types::U256;
use utils::create_did;

/// An example to demonstrate how an identity can issue and control native assets
/// such as Token Foundries and NFTs.
///
/// For this example, we consider the case where an authority issues
/// carbon credits that can be used to pay for carbon emissions or traded on a marketplace.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // =======================================
  // Create the authority's DID and the foundry.
  // =======================================

  // Create a new DID for the authority.
  let (client, _address, secret_manager, authority_did): (Client, Address, SecretManager, StardustDID) =
    create_did().await?;

  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // We want to update the foundry counter of the authority's Alias Output, so we create an
  // updated version of the output. We pass in the previous document,
  // because we don't want to modify it in this update.
  let authority_document: StardustDocument = client.resolve_did(&authority_did).await?;
  let authority_alias_output: AliasOutput = client.update_did_output(authority_document).await?;

  // We will add one foundry to this Alias Output.
  let authority_alias_output = AliasOutputBuilder::from(&authority_alias_output)
    .with_foundry_counter(1)
    .finish()?;

  // Create a token foundry that represents carbon credits.
  // The authority is set as the immutable owner.
  let carbon_credits_foundry: FoundryOutput =
    create_foundry(rent_structure.clone(), AliasAddress::new(AliasId::from(&authority_did)))?;
  let carbon_credits_foundry_id: FoundryId = carbon_credits_foundry.id();

  // Publish all outputs.
  let block: Block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![authority_alias_output.into(), carbon_credits_foundry.into()])?
    .finish()
    .await?;
  let _ = client.retry_until_included(&block.id(), None, None).await?;

  // =======================================
  // Resolve Foundry and its issuer DID.
  // =======================================

  let foundry_output_id: OutputId = client.foundry_output_id(carbon_credits_foundry_id).await?;
  let carbon_credits_foundry: OutputResponse = client.get_output(&foundry_output_id).await?;
  let carbon_credits_foundry: Output = Output::try_from(&carbon_credits_foundry.output)?;

  let carbon_credits_foundry: FoundryOutput = if let Output::Foundry(foundry_output) = carbon_credits_foundry {
    foundry_output
  } else {
    anyhow::bail!("expected Foundry output")
  };

  let authority_alias_id: &AliasId = carbon_credits_foundry.alias_address().alias_id();

  let network: NetworkName = NetworkName::try_from(client.get_bech32_hrp().await?)?;
  let authority_did: StardustDID = StardustDID::new(authority_alias_id.deref(), &network);

  let _authority_document: StardustDocument = client.resolve_did(&authority_did).await?;

  Ok(())
}

/// Creates the carbon credits foundry that is owned by `foundry_owner`
/// and with half of its maximum supply of tokens minted.
fn create_foundry(rent_structure: RentStructure, foundry_owner: AliasAddress) -> anyhow::Result<FoundryOutput> {
  let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(
    U256::from(500_000u32),
    U256::from(0u8),
    U256::from(1_000_000u32),
  )?);
  let foundry_id = FoundryId::build(&foundry_owner, 1, token_scheme.kind());
  let token_id = TokenId::from(foundry_id);

  FoundryOutputBuilder::new_with_minimum_storage_deposit(rent_structure, 1, token_scheme)?
    .add_native_token(NativeToken::new(token_id, U256::from(70u8))?)
    .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
      ImmutableAliasAddressUnlockCondition::new(foundry_owner),
    ))
    .finish()
    .map_err(Into::into)
}
