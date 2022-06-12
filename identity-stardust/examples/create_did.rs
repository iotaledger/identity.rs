use identity_core::convert::ToJson;
use iota_client::{
  bee_block::{
    output::{
      AliasId,
      AliasOutputBuilder,
      feature::{IssuerFeature, MetadataFeature, SenderFeature}, Feature, Output, OutputId, unlock_condition::{
        GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition, UnlockCondition,
      },
    },
    payload::{Payload, transaction::TransactionEssence},
  },
  Client,
  constants::SHIMMER_TESTNET_BECH32_HRP,
  node_api::indexer::query_parameters::QueryParameter,
  request_funds_from_faucet,
  secret::{mnemonic::MnemonicSecretManager, SecretManager},
};
use iota_client::bee_block::Block;
use iota_client::bee_block::output::ByteCostConfig;
use iota_client::crypto::keys::bip39;

use identity_stardust::StardustDocument;

// PROBLEMS SO FAR:
// 1) Alias Id is inferred from the block, so we have to use a placeholder DID.
// 2) Cannot get an Output Id back from an Alias Id (hash of Output Id), need to use Indexer API (TO_DO).

/// Demonstrate how to embed a DID Document in an Alias Output.
///
/// iota.rs alias example:
/// https://github.com/iotaledger/iota.rs/blob/f945ccf326829a418334942ae9cf53b8fab3dbe5/examples/outputs/alias.rs
///
/// iota.js mint-nft example:
/// https://github.com/iotaledger/iota.js/blob/79a71d3a2ad03be5bd6148689d083947f3b98476/packages/iota/examples/mint-nft/src/index.ts
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // let endpoint = "http://localhost:14265";
  let endpoint = "https://api.alphanet.iotaledger.net";
  let faucet_auto = format!("{endpoint}/api/plugins/faucet/v1/enqueue");
  let faucet_manual = "https://faucet.alphanet.iotaledger.net";

  // ===========================================================================
  // Step 1: Create/load a wallet.
  // ===========================================================================

  // let keypair = identity_core::crypto::KeyPair::new(KeyType::Ed25519).unwrap();
  // println!("PrivateKey: {}", keypair.private().to_string());
  // let mnemonic = bip39::wordlist::encode(keypair.private().as_ref(),&bip39::wordlist::ENGLISH).unwrap();
  let mnemonic = "veteran provide abstract express quick another fee dragon trend extend cotton tail dog truly angle napkin lunch dinosaur shrimp odor gain bag media mountain";
  println!("Mnemonic: {}", mnemonic);
  let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic)?);

  // Create a client instance.
  let client = Client::builder()
    .with_node(endpoint)?
    .with_node_sync_disabled()
    .finish()
    .await?;

  let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
  let address_bech32 = address.to_bech32(SHIMMER_TESTNET_BECH32_HRP);
  println!("Wallet address: {address_bech32}");

  println!("INTERACTION REQUIRED: request faucet funds to the above wallet from {faucet_manual}");
  // request_funds_from_faucet(&faucet_auto, &address_bech32).await?;
  // tokio::time::sleep(std::time::Duration::from_secs(15)).await;

  // ===========================================================================
  // Step 2: Create and publish a DID Document in an Alias Output.
  // ===========================================================================

  // Create an empty DID Document.
  // All new Stardust DID Documents have a placeholder DID,
  // "did:stardust:00000000000000000000000000000000".
  let document: StardustDocument = StardustDocument::new();
  println!("DID Document {:#}", document);

  // Create a new Alias Output with the DID Document as metadata.
  let byte_cost_config: ByteCostConfig = client.get_byte_cost_config().await?;
  let alias_output: Output = AliasOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, AliasId::null())?
    .with_state_index(0)
    .with_foundry_counter(0)
    .add_feature(Feature::Sender(SenderFeature::new(address)))
    .add_feature(Feature::Metadata(MetadataFeature::new(document.to_json_vec()?)?))
    .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
    .add_unlock_condition(UnlockCondition::StateControllerAddress(
      StateControllerAddressUnlockCondition::new(address),
    ))
    .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
      address,
    )))
    .finish_output()?;
  println!("Deposit amount: {}", alias_output.amount());

  let block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![alias_output])?
    .finish()
    .await?;

  println!(
    "Transaction with new alias output sent: {endpoint}/api/v2/blocks/{}",
    block.id()
  );
  let _ = client.retry_until_included(&block.id(), None, None).await?;


  // Infer DID from Alias Output block.
  let did = StardustDocument::did_from_block(&block)?;
  println!("DID: {did}");

  // ===========================================================================
  // Step 3: Resolve a DID Document.
  // ===========================================================================
  // iota.rs indexer example:
  // https://github.com/iotaledger/iota.rs/blob/f945ccf326829a418334942ae9cf53b8fab3dbe5/examples/indexer.rs

  // Extract Alias ID from DID.
  let alias_id = StardustDocument::did_to_alias_id(&did)?;
  println!("Alias ID: {alias_id}");

  // TODO: how to query the Indexer plugin, or get a bech32 address out of an Alias ID?
  // // Query Indexer INX Plugin for the Output Id of the Alias ID.
  // client.basic_output_ids(QueryParameter::Address())
  // println!("output ids {:?}", output_ids);
  //
  // let outputs = client.get_outputs(output_ids).await?;
  //
  // println!("outputs {:?}", outputs);


  // ===========================================================================
  // Step 4: Publish an update. (optional)
  // ===========================================================================
  // TODO: we could always publish twice on creation to populate the DID (could fail),
  //       or just infer the DID during resolution (safer).

  // TODO: get this from the resolved Alias Output + Document.
  let alias_output_id = get_alias_output_id(block.payload().unwrap());

  let outputs = vec![
    AliasOutputBuilder::new_with_amount(1_000_000, alias_id)?
      .with_state_index(1)
      .with_foundry_counter(0)
      .add_feature(Feature::Sender(SenderFeature::new(address)))
      .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
      .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(address),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        address,
      )))
      .finish_output()?,
  ];

  let block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_input(alias_output_id.into())?
    .with_outputs(outputs)?
    .finish()
    .await?;
  println!(
    "Transaction with alias id set sent: {endpoint}/api/v2/blocks/{}",
    block.id()
  );
  let _ = client.retry_until_included(&block.id(), None, None).await?;
  Ok(())
}

// helper function to get the output id for the first alias output
fn get_alias_output_id(payload: &Payload) -> OutputId {
  match payload {
    Payload::Transaction(tx_payload) => {
      let TransactionEssence::Regular(regular) = tx_payload.essence();
      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(_alias_output) = output {
          return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
        }
      }
      panic!("No alias output in transaction essence")
    }
    _ => panic!("No tx payload"),
  }
}
