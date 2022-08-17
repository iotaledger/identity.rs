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
use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::Output;
use iota_client::block::Block;
use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
use iota_client::node_api::indexer::query_parameters::QueryParameter;
use iota_client::secret::mnemonic::MnemonicSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

static ENDPOINT: &str = "https://api.testnet.shimmer.network/";
static FAUCET_URL: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

/// An example to demonstrate how one identity can control (and therefore "own") another identity.
pub async fn run() -> anyhow::Result<()> {
  pretty_env_logger::init();

  let mnemonic = "tiny random pull river old pause rail genuine obscure balcony slogan black dentist almost december century clutch couple cotton example host tonight cactus hobby";

  let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?);

  let client: Client = Client::builder().with_primary_node(ENDPOINT, None)?.finish()?;

  let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

  request_faucet_funds(&client, address).await?;

  let rent_structure = client.get_rent_structure().await?;

  let network_name: NetworkName = client.network_name().await?;

  let mut document: StardustDocument = StardustDocument::new(&network_name);

  let mut document2: StardustDocument = StardustDocument::new(&network_name);

  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let keypair2: KeyPair = KeyPair::new(KeyType::Ed25519)?;

  let method: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;
  let method2: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair2.public(), "#key-1")?;

  document.insert_method(method, MethodScope::VerificationMethod)?;

  document2.insert_method(method2, MethodScope::VerificationMethod)?;

  let alias_output: AliasOutput = client
    .new_did_output(address, document, Some(rent_structure.clone()))
    .await?;

  let document: StardustDocument = client.publish_did_output(&secret_manager, alias_output.clone()).await?;

  let alias_output2: AliasOutput = client
    .new_did_output(address, document2, Some(rent_structure.clone()))
    .await?;

  let tag_bytes: [u8; 32] = prefix_hex::decode(document.id().tag())?;
  let alias_id: AliasId = AliasId::new(tag_bytes);

  let alias_output2 = AliasOutputBuilder::from(&alias_output2)
    .with_minimum_storage_deposit(rent_structure.clone())
    .replace_unlock_condition(StateControllerAddressUnlockCondition::new(Address::Alias(alias_id.into())).into())?
    .replace_unlock_condition(GovernorAddressUnlockCondition::new(Address::Alias(alias_id.into())).into())?
    .finish()?;

  let document2: StardustDocument = client.publish_did_output(&secret_manager, alias_output2).await?;

  let tag_bytes: [u8; 32] = prefix_hex::decode(document2.id().tag())?;
  let alias_id2: AliasId = AliasId::new(tag_bytes);

  println!("published {}", alias_id);
  println!("published {}", alias_id2);

  let did = document.id().to_owned();
  let did2 = document2.id().to_owned();

  let mut document2: StardustDocument = client.resolve_did(&did2).await?;

  // Update
  let keypair = KeyPair::new(KeyType::Ed25519)?;
  let method =
    StardustVerificationMethod::new(document2.id().to_owned(), KeyType::Ed25519, keypair.public(), "#key-2")?;
  document2.insert_method(method, MethodScope::authentication())?;

  let alias_output2: AliasOutput = client.update_did_output(document2).await?;

  let alias_output2 = AliasOutputBuilder::from(&alias_output2)
    .with_minimum_storage_deposit(rent_structure.clone())
    .finish()?;

  let block: Block = client
    .block()
    .with_secret_manager(&secret_manager)
    .with_outputs(vec![alias_output2.into()])?
    .finish()
    .await?;

  println!("{block:#?}");

  let _ = client.retry_until_included(&block.id(), None, None).await?;

  let doc = client.resolve_did(&did).await?;
  let doc2 = client.resolve_did(&did2).await?;

  println!("doc: {doc:#?}");
  println!("doc2: {doc2:#?}");

  Ok(())
}

/// Requests funds from the testnet faucet for the given `address`.
async fn request_faucet_funds(client: &Client, address: Address) -> anyhow::Result<()> {
  let address_bech32 = address.to_bech32(SHIMMER_TESTNET_BECH32_HRP);

  iota_client::request_funds_from_faucet(FAUCET_URL, &address_bech32).await?;

  tokio::time::timeout(std::time::Duration::from_secs(30), async {
    loop {
      tokio::time::sleep(std::time::Duration::from_secs(5)).await;

      let balance = get_address_balance(client, &address_bech32).await?;
      if balance > 0 {
        break;
      }
    }
    Ok::<(), anyhow::Error>(())
  })
  .await??;

  Ok(())
}

/// Returns the balance of the given bech32-encoded `address`.
async fn get_address_balance(client: &Client, address: &str) -> anyhow::Result<u64> {
  let output_ids = client
    .basic_output_ids(vec![
      QueryParameter::Address(address.to_owned()),
      QueryParameter::HasExpiration(false),
      QueryParameter::HasTimelock(false),
      QueryParameter::HasStorageDepositReturn(false),
    ])
    .await?;

  let outputs_responses = client.get_outputs(output_ids).await?;

  let mut total_amount = 0;
  for output_response in outputs_responses {
    let output = Output::try_from(&output_response.output)?;
    total_amount += output.amount();
  }

  Ok(total_amount)
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  run().await.map(|_| ())
}
