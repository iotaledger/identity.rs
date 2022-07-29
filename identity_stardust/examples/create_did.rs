// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodScope;
use iota_client::block::address::Address;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::feature::MetadataFeature;
use iota_client::block::output::feature::SenderFeature;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use iota_client::block::output::unlock_condition::UnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::BasicOutputBuilder;
use iota_client::block::output::ByteCostConfig;
use iota_client::block::output::Feature;
use iota_client::block::output::Output;
use iota_client::node_api::indexer::query_parameters::QueryParameter;
use iota_client::secret::mnemonic::MnemonicSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustVerificationMethod;

// PROBLEMS SO FAR:
// 1) Alias Id is inferred from the block, so we have to use a placeholder DID for creation.
// 2) Cannot get an Output Id back from an Alias Id (hash of Output Id), need to use Indexer API.
// 3) The Output response from the Indexer is an Output, not a Block, so cannot infer Alias ID from it (fine since we
// use the ID to retrieve the Output in the first place).    The OutputDto conversion is annoying too.
// 4) The pieces needed to publish an update are fragmented (Output ID for input, amount, document), bit annoying to
// reconstruct.    Use a holder struct like Holder { AliasOutput, StardustDocument } with convenience functions?
// 5) Inferred fields such as the controller and governor need to reflect in the (JSON) Document but excluded from the
// StardustDocument serialization when published.    Handle with a separate `pack` function like before?

static FAUCET_URL: &str = "http://localhost:8091";
static PRIVATE_TESTNET_BECH32_HRP: &str = "tst";

/// Demonstrate how to embed a DID Document in an Alias Output.
///
/// iota.rs alias example:
/// https://github.com/iotaledger/iota.rs/blob/f945ccf326829a418334942ae9cf53b8fab3dbe5/examples/outputs/alias.rs
///
/// iota.js mint-nft example:
/// https://github.com/iotaledger/iota.js/blob/79a71d3a2ad03be5bd6148689d083947f3b98476/packages/iota/examples/mint-nft/src/index.ts
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let endpoint = "http://localhost:14265";
  // let endpoint = "https://api.alphanet.iotaledger.net";
  let faucet_manual = "https://faucet.alphanet.iotaledger.net";
  let network_name = NetworkName::try_from("alpha")?;

  // ===========================================================================
  // Step 1: Create or load your wallet.
  // ===========================================================================

  // let keypair = identity_core::crypto::KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();
  // println!("PrivateKey: {}", keypair.private().to_string());
  // let mnemonic =
  // iota_client::crypto::keys::bip39::wordlist::encode(keypair.private().as_ref(),&bip39::wordlist::ENGLISH).unwrap();

  // NOTE: this is just a randomly generated mnemonic, REMOVE THIS, never actually commit your seed or mnemonic.
  let mnemonic = "veteran provide abstract express quick another fee dragon trend extend cotton tail dog truly angle napkin lunch dinosaur shrimp odor gain bag media mountain";
  println!("Mnemonic: {}", mnemonic);
  let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?);

  // Create a client instance.
  let client = Client::builder()
    .with_node(endpoint)?
    .with_node_sync_disabled()
    .finish()?;

  // Get the Bech32 human-readable part (HRP) of the protocol.
  // E.g. "rms" for the Shimmer testnet.
  let node_info = client.get_info().await?;
  let network_hrp = node_info.node_info.protocol.bech32_hrp;

  let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
  let address_bech32 = address.to_bech32(PRIVATE_TESTNET_BECH32_HRP);
  println!("Wallet address: {address_bech32}");

  request_faucet_funds(&client, address).await;

  // ===========================================================================
  // Step 2: Create and publish a DID Document in an Alias Output.
  // ===========================================================================

  // Create an empty DID Document.
  //
  // All new Stardust DID Documents initially use a placeholder DID for the network,
  // "did:stardust:rms:0x0000000000000000000000000000000000000000000000000000000000000000".
  let document: StardustDocument = StardustDocument::new(&network_name);

  println!("DID Document {:#}", document);

  // Create a new Alias Output with the DID Document as state metadata.
  let byte_cost_config: ByteCostConfig = client.get_byte_cost_config().await?;
  let alias_output: Output = AliasOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, AliasId::null())?
    .with_state_index(0)
    .with_foundry_counter(0)
    .with_state_metadata(document.pack()?)
    .add_feature(Feature::Sender(SenderFeature::new(address)))
    .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
    .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
    .add_unlock_condition(UnlockCondition::StateControllerAddress(
      StateControllerAddressUnlockCondition::new(address),
    ))
    .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
      address,
    )))
    .finish_output()?;
  println!("Deposit amount: {}", alias_output.amount());

  // Publish to the Tangle ledger.
  let block1 = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![alias_output])?
    .finish()
    .await?;
  println!(
    "Transaction with new alias output sent: {endpoint}/api/v2/blocks/{}",
    block1.id()
  );
  let _ = client.retry_until_included(&block1.id(), None, None).await?;

  // Infer DID from the Alias Output block.
  let did: StardustDID = StardustDID::from_block(&block1, &network_name)?;
  println!("DID: {did}");

  // ===========================================================================
  // Step 3: Resolve a DID Document.
  // ===========================================================================
  // iota.rs indexer example:
  // https://github.com/iotaledger/iota.rs/blob/f945ccf326829a418334942ae9cf53b8fab3dbe5/examples/indexer.rs

  // Extract Alias ID from the DID tag.
  let alias_id: AliasId = AliasId::from_str(did.tag())?;
  println!("Alias ID: {alias_id}");

  // Query Indexer INX Plugin for the Output of the Alias ID.
  let alias_output_id = client.alias_output_id(alias_id).await?;
  println!("Output ID: {alias_output_id}");
  let response = client.get_output(&alias_output_id).await?;
  let output = Output::try_from(&response.output)?;
  println!("Output: {output:?}");

  // The resolved DID Document replaces the placeholder DID with the correct one.
  let resolved_document = StardustDocument::unpack_from_output(&did, &output)?;
  println!("Resolved Document: {resolved_document:#}");

  let alias_output = match output {
    Output::Alias(output) => Ok(output),
    _ => Err(anyhow::anyhow!("not an alias output")),
  }?;

  // ===========================================================================
  // Step 4: Publish an updated Alias Output. (optional)
  // ===========================================================================

  // Add a new Ed25519 verification method to the DID Document for authentication. (optional)
  let mut updated_document = resolved_document.clone();
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method = StardustVerificationMethod::new(
    updated_document.id().clone(),
    keypair.type_(),
    keypair.public(),
    "#key-0",
  )?;
  updated_document.insert_method(method, MethodScope::authentication())?;

  // Update the Alias Output to contain an explicit Alias ID, and the updated DID Document.
  let byte_cost_config: ByteCostConfig = client.get_byte_cost_config().await?;
  let updated_alias_output = AliasOutputBuilder::from(&alias_output)
    // Set the deposit to the new minimum covering the increased size of the DID Document.
    .with_minimum_storage_deposit(byte_cost_config)
    // Set the explicit Alias ID.
    .with_alias_id(alias_id)
    // Set the updated DID Document.
    .with_state_metadata(updated_document.pack()?)
    // State controller updates must increment the state index.
    .with_state_index(alias_output.state_index() + 1)
    .finish_output()?;

  println!("Updated output: {updated_alias_output:?}");

  let block2 = client
    .block()
    .with_secret_manager(&secret_manager)
    // Omit inputs so it automatically selects a Basic Output to cover the increased amount.
    // .with_input(alias_output_id.into())?
    .with_outputs(vec![updated_alias_output])?
    .finish()
    .await?;

  println!(
    "Transaction with updated Alias Output sent: {endpoint}/api/v2/blocks/{}",
    block2.id()
  );
  let _ = client.retry_until_included(&block2.id(), None, None).await?;

  // ===========================================================================
  // Step 5: Destroy Alias Output. (optional)
  // ===========================================================================

  // Query Indexer INX Plugin for the latest Output of the Alias ID.
  let alias_output_id = client.alias_output_id(alias_id).await?;
  let response = client.get_output(&alias_output_id).await?;
  let alias_output = Output::try_from(&response.output)?;

  // Consume the Alias Output containing the DID Document, sending its tokens to a new Basic Output.
  // WARNING: this destroys the DID Document and renders it permanently unrecoverable.
  let basic_output = BasicOutputBuilder::new_with_amount(alias_output.amount())?
    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    .finish_output()?;
  let block3 = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_input(alias_output_id.into())?
    .with_outputs(vec![basic_output])?
    .finish()
    .await?;

  println!(
    "Transaction destroying Alias Output sent: {endpoint}/api/v2/blocks/{}",
    block3.id()
  );
  let _ = client.retry_until_included(&block3.id(), None, None).await?;

  // Consolidate amounts in separate Basic Outputs into a single one.
  client.consolidate_funds(&secret_manager, 0, 0..1).await?;

  Ok(())
}

/// Request funds from a faucet for the given address.
///
/// Returns when the funds were granted to the address.
async fn request_faucet_funds(client: &Client, address: Address) {
  let address_bech32 = address.to_bech32(PRIVATE_TESTNET_BECH32_HRP);

  let faucet_funds_request_url = format!("{FAUCET_URL}/api/enqueue");

  iota_client::request_funds_from_faucet(faucet_funds_request_url.as_str(), &address_bech32)
    .await
    .unwrap();

  loop {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let balance = get_address_balance(client, &address_bech32).await;
    println!("{address_bech32} balance is {balance}");
    if balance > 0 {
      break;
    }
  }
}

async fn get_address_balance(client: &Client, address: &str) -> u64 {
  let output_ids = client
    .basic_output_ids(vec![
      QueryParameter::Address(address.to_owned()),
      QueryParameter::HasExpirationCondition(false),
      QueryParameter::HasTimelockCondition(false),
      QueryParameter::HasStorageReturnCondition(false),
    ])
    .await
    .unwrap();

  let outputs_responses = client.get_outputs(output_ids).await.unwrap();

  let mut total_amount = 0;
  for output_response in outputs_responses {
    let output = Output::try_from(&output_response.output).unwrap();
    total_amount += output.amount();
  }

  total_amount
}
