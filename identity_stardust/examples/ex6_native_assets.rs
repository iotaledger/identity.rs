// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;

use iota_client::block::address::Address;
use iota_client::block::address::AliasAddress;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::unlock_condition::ImmutableAliasAddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::Feature;
use iota_client::block::output::FoundryId;
use iota_client::block::output::FoundryOutput;
use iota_client::block::output::FoundryOutputBuilder;
use iota_client::block::output::NativeToken;
use iota_client::block::output::NftId;
use iota_client::block::output::NftOutput;
use iota_client::block::output::NftOutputBuilder;
use iota_client::block::output::RentStructure;
use iota_client::block::output::SimpleTokenScheme;
use iota_client::block::output::TokenId;
use iota_client::block::output::TokenScheme;
use iota_client::block::output::UnlockCondition;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;
use primitive_types::U256;

mod ex0_create_did;

/// An example to demonstrate how an identity can issue and control native assets
/// such as Token Foundries and NFTs.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  pretty_env_logger::init();

  // Create a new DID in an Alias Output.
  let (client, _address, secret_manager, document): (Client, Address, SecretManager, StardustDocument) =
    ex0_create_did::run().await?;

  let alias_id: AliasId = document.id().into();
  let alias_address: AliasAddress = alias_id.into();

  let rent_structure: RentStructure = client.get_rent_structure().await?;

  // We want to update the foundry counter of the Alias Output, so we obtain an
  // updated version of the output. We pass in the same document,
  // because we don't want to modify it in this update.
  let alias_output: AliasOutput = client.update_did_output(document).await?;

  // We don't need to modify the document, so we
  let alias_output = AliasOutputBuilder::from(&alias_output)
    // Set the foundry counter to 1, because we will add a foundry to this Alias Output.
    .with_foundry_counter(1)
    .finish()?;

  // Create a token foundry with the alias set as the immutable owner.
  let foundry_output: FoundryOutput = create_foundry(rent_structure.clone(), alias_address)?;

  // Create an NFT with the alias set as the immutable issuer.
  let nft_output: NftOutput = create_nft(rent_structure, alias_address)?;

  // Publish all outputs.
  let block: Block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![alias_output.into(), nft_output.into(), foundry_output.into()])?
    .finish()
    .await?;

  let _ = client.retry_until_included(&block.id(), None, None).await?;

  Ok(())
}

/// Creates an example foundry that is owned by `foundry_owner`.
fn create_foundry(rent_structure: RentStructure, foundry_owner: AliasAddress) -> anyhow::Result<FoundryOutput> {
  let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(
    U256::from(70u8),
    U256::from(0u8),
    U256::from(100u8),
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
