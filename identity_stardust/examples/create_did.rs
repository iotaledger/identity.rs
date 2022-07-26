// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_did::did::DID;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use identity_stardust::StardustClientExt;
use identity_stardust::StardustCoreDocument;
use identity_stardust::StardustDocumentMetadata;
use iota_client::block::address::Address;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::feature::MetadataFeature;
use iota_client::block::output::feature::SenderFeature;
use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use iota_client::block::output::unlock_condition::UnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::Feature;
use iota_client::block::output::Output;
use iota_client::block::output::RentStructure;
use iota_client::block::Block;
use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
use iota_client::crypto::keys::bip39;
use iota_client::node_api::indexer::query_parameters::QueryParameter;
use iota_client::secret::mnemonic::MnemonicSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StardustVerificationMethod;

static ENDPOINT: &str = "https://api.testnet.shimmer.network/";
static FAUCET_URL: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

/// Demonstrate how to embed a DID Document in an Alias Output.
pub async fn run() -> anyhow::Result<(Client, Address, SecretManager, StardustDocument)> {
  // Create a new DID Document.
  let did: StardustDID = StardustDID::placeholder(
    &NetworkName::try_from(SHIMMER_TESTNET_BECH32_HRP).expect("HRP should be a valid network name"),
  );

  let method: StardustVerificationMethod = VerificationMethod::builder(Default::default())
    .id(did.to_url().join("#key-1")?)
    .controller(did.clone())
    .type_(MethodType::Ed25519VerificationKey2018)
    .data(MethodData::new_multibase(b"#key-1"))
    .build()?;

  let mut metadata: StardustDocumentMetadata = StardustDocumentMetadata::new();
  metadata.created = Some(Timestamp::now_utc());
  metadata.updated = Some(Timestamp::now_utc());

  let document: StardustCoreDocument = StardustCoreDocument::builder(Object::default())
    .id(did.clone())
    .controller(did.clone())
    .verification_method(method)
    .build()?;

  let document: StardustDocument = StardustDocument::from((document, metadata));

  // Create a client and an address with funds from the testnet faucet.
  let client: Client = Client::builder()
    .with_node(ENDPOINT)?
    .with_node_sync_disabled()
    .finish()?;

  let (address, secret_manager): (Address, SecretManager) = get_address_with_funds(&client).await?;

  // Create an alias output and include the DID document in its metadata.
  let rent_structure: RentStructure = client.get_rent_structure().await?;
  let output: Output = AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())?
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

  // Publish the output and get the published document.
  let block: Block = client.publish_outputs(&secret_manager, vec![output]).await?;
  let documents: Vec<StardustDocument> = client.documents_from_block(&block).await?;

  println!("Published DID document: {:#?}", documents[0]);

  Ok((
    client,
    address,
    secret_manager,
    documents
      .into_iter()
      .next()
      .expect("documents should contain exactly one document"),
  ))
}

async fn get_address_with_funds(client: &Client) -> anyhow::Result<(Address, SecretManager)> {
  let keypair = identity_core::crypto::KeyPair::new(identity_core::crypto::KeyType::Ed25519)?;
  let mnemonic =
    iota_client::crypto::keys::bip39::wordlist::encode(keypair.private().as_ref(), &bip39::wordlist::ENGLISH)
      .map_err(|err| anyhow::anyhow!(format!("{err:?}")))?;

  let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic)?);

  let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

  request_faucet_funds(client, address).await?;

  Ok((address, secret_manager))
}

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

async fn get_address_balance(client: &Client, address: &str) -> anyhow::Result<u64> {
  let output_ids = client
    .basic_output_ids(vec![
      QueryParameter::Address(address.to_owned()),
      QueryParameter::HasExpirationCondition(false),
      QueryParameter::HasTimelockCondition(false),
      QueryParameter::HasStorageReturnCondition(false),
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
