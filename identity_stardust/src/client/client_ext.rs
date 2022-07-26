// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::ops::Deref;
use std::str::FromStr;

use iota_client::api_types::responses::OutputResponse;
use iota_client::block::output::AliasId;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::payload::transaction::TransactionEssence;
use iota_client::block::payload::Payload;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;

use crate::error::OutputError;
use crate::error::Result;
use crate::Error;
use crate::NetworkName;
use crate::StardustDID;
use crate::StardustDocument;

#[async_trait::async_trait]
pub trait StardustClientExt: Sync {
  fn client(&self) -> &Client;

  async fn publish_outputs(&self, secret_manager: &SecretManager, alias_outputs: Vec<Output>) -> Result<Block> {
    let block: Block = self
      .client()
      .block()
      .with_secret_manager(secret_manager)
      .with_outputs(alias_outputs)
      .map_err(Error::ClientError)?
      .finish()
      .await
      .map_err(Error::ClientError)?;

    // TODO: Should we return the block returned from this?
    let _ = self
      .client()
      .retry_until_included(&block.id(), None, None)
      .await
      .map_err(Error::ClientError)?;

    Ok(block)
  }

  async fn resolve(&self, did: &StardustDID) -> Result<StardustDocument> {
    let alias_id: AliasId = AliasId::from_str(did.tag())?;

    let output_id: OutputId = self
      .client()
      .alias_output_id(alias_id)
      .await
      .map_err(Error::ClientError)?;
    let response: OutputResponse = self.client().get_output(&output_id).await.map_err(Error::ClientError)?;
    let output: Output =
      Output::try_from(&response.output).map_err(|err| Error::OutputError(OutputError::ConversionError(err)))?;

    let network_hrp: String = get_network_hrp(self.client()).await?;

    let did: StardustDID = StardustDID::new(alias_id.deref(), &NetworkName::try_from(Cow::from(network_hrp))?);

    StardustDocument::unpack_from_output(&did, &output)
  }

  async fn documents_from_block(&self, block: &Block) -> Result<Vec<StardustDocument>> {
    let network_hrp: String = get_network_hrp(self.client()).await?;
    let mut documents = Vec::new();

    if let Some(Payload::Transaction(tx_payload)) = block.payload() {
      let TransactionEssence::Regular(regular) = tx_payload.essence();

      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(alias_output) = output {
          let alias_id = if alias_output.alias_id().is_null() {
            AliasId::from(OutputId::new(
              tx_payload.id(),
              index.try_into().expect("the output count should not exceed u16"),
            )?)
          } else {
            alias_output.alias_id().to_owned()
          };

          let did: StardustDID = StardustDID::new(
            alias_id.deref(),
            &NetworkName::try_from(Cow::from(network_hrp.clone()))?,
          );
          documents.push(StardustDocument::unpack(&did, alias_output.state_metadata())?);
        }
      }
    }

    Ok(documents)
  }

  fn alias_ids_from_payload(payload: &Payload) -> Result<Vec<AliasId>> {
    let mut alias_ids = Vec::new();
    if let Payload::Transaction(tx_payload) = payload {
      let TransactionEssence::Regular(regular) = tx_payload.essence();
      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(_) = output {
          alias_ids.push(AliasId::from(OutputId::new(
            tx_payload.id(),
            index.try_into().expect("the output count should not exceed u16"),
          )?));
        }
      }
    }
    Ok(alias_ids)
  }
}

impl StardustClientExt for Client {
  fn client(&self) -> &Client {
    self
  }
}

impl StardustClientExt for &Client {
  fn client(&self) -> &Client {
    self
  }
}

async fn get_network_hrp(client: &Client) -> Result<String> {
  client
    .get_network_info()
    .await
    .map_err(Error::ClientError)?
    .bech32_hrp
    .ok_or(Error::InvalidNetworkName)
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_did::did::DID;
  use identity_did::document::Document;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodScope;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;
  use iota_client::api_types::responses::OutputResponse;
  use iota_client::block::address::Address;
  use iota_client::block::output::feature::IssuerFeature;
  use iota_client::block::output::feature::MetadataFeature;
  use iota_client::block::output::feature::SenderFeature;
  use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
  use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
  use iota_client::block::output::AliasId;
  use iota_client::block::output::AliasOutput;
  use iota_client::block::output::AliasOutputBuilder;
  use iota_client::block::output::Feature;
  use iota_client::block::output::Output;
  use iota_client::block::output::OutputId;
  use iota_client::block::output::RentStructure;
  use iota_client::block::output::UnlockCondition;
  use iota_client::block::payload::transaction::TransactionEssence;
  use iota_client::block::payload::Payload;
  use iota_client::block::Block;
  use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
  use iota_client::crypto::keys::bip39;
  use iota_client::node_api::indexer::query_parameters::QueryParameter;
  use iota_client::secret::mnemonic::MnemonicSecretManager;
  use iota_client::secret::SecretManager;
  use iota_client::Client;

  use crate::error::OutputError;
  use crate::Error;
  use crate::StardustCoreDocument;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustDocumentMetadata;
  use crate::StardustVerificationMethod;

  use super::StardustClientExt;

  // TODO: Change to private tangle in CI; detect CI via env var?.
  // static ENDPOINT: &str = "https://api.alphanet.iotaledger.net/";
  // static FAUCET_URL: &str = "https://faucet.alphanet.iotaledger.net/api/enqueue";
  static ENDPOINT: &str = "https://api.testnet.shimmer.network/";
  static FAUCET_URL: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

  fn generate_method(controller: &StardustDID, fragment: &str) -> StardustVerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn generate_document(id: &StardustDID) -> StardustDocument {
    let mut metadata: StardustDocumentMetadata = StardustDocumentMetadata::new();
    metadata.created = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());
    metadata.updated = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());

    let document: StardustCoreDocument = StardustCoreDocument::builder(Object::default())
      .id(id.clone())
      .controller(id.clone())
      .verification_method(generate_method(id, "#key-1"))
      .verification_method(generate_method(id, "#key-2"))
      .verification_method(generate_method(id, "#key-3"))
      .authentication(generate_method(id, "#auth-key"))
      .authentication(id.to_url().join("#key-3").unwrap())
      .build()
      .unwrap();

    StardustDocument::from((document, metadata))
  }

  fn client() -> Client {
    Client::builder()
      .with_node(ENDPOINT)
      .unwrap()
      .with_node_sync_disabled()
      .finish()
      .unwrap()
  }

  async fn get_address_with_funds(client: &Client) -> (Address, SecretManager) {
    let keypair = identity_core::crypto::KeyPair::new(identity_core::crypto::KeyType::Ed25519).unwrap();
    let mnemonic =
      iota_client::crypto::keys::bip39::wordlist::encode(keypair.private().as_ref(), &bip39::wordlist::ENGLISH)
        .unwrap();

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic).unwrap());

    let address = client
      .get_addresses(&secret_manager)
      .with_range(0..1)
      .get_raw()
      .await
      .unwrap()[0];

    request_faucet_funds(client, address).await;

    (address, secret_manager)
  }

  /// Request funds from a faucet for the given address.
  ///
  /// Returns when the funds were granted to the address.
  async fn request_faucet_funds(client: &Client, address: Address) {
    let address_bech32 = address.to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    iota_client::request_funds_from_faucet(FAUCET_URL, &address_bech32)
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

  async fn alias_output(client: &Client, address: Address, document: &StardustDocument) -> Output {
    let rent_structure: RentStructure = client.get_rent_structure().await.map_err(Error::ClientError).unwrap();
    AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
      .map_err(Error::AliasOutputBuildError)
      .unwrap()
      .with_state_index(0)
      .with_foundry_counter(0)
      .with_state_metadata(document.clone().pack().unwrap())
      .add_feature(Feature::Sender(SenderFeature::new(address)))
      .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3]).unwrap()))
      .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(address),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        address,
      )))
      .finish_output()
      .unwrap()
  }

  fn valid_did() -> StardustDID {
    "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  async fn publish_document(
    client: impl StardustClientExt,
    document: StardustDocument,
    address: Address,
    secret_manager: &SecretManager,
  ) -> crate::error::Result<(Block, StardustDocument)> {
    let output = alias_output(client.client(), address, &document).await;

    let block: Block = client.publish_outputs(secret_manager, vec![output]).await?;

    let documents: Vec<StardustDocument> = client.documents_from_block(&block).await?;

    Ok((
      block,
      documents
        .into_iter()
        .next()
        .expect("there should be at least one document"),
    ))
  }

  #[tokio::test]
  async fn test_publish_resolve() {
    let client: Client = client();
    let (address, secret_manager) = get_address_with_funds(&client).await;
    let document = generate_document(&valid_did());
    let (_, document) = publish_document(&client, document, address, &secret_manager)
      .await
      .unwrap();
    let resolved = client.resolve(document.id()).await.unwrap();

    assert_eq!(document, resolved);
  }

  async fn publish_update(
    client: &Client,
    document: StardustDocument,
    secret_manager: &SecretManager,
  ) -> crate::error::Result<Block> {
    let rent_structure: RentStructure = client.get_rent_structure().await.map_err(Error::ClientError)?;

    let alias_id: AliasId = AliasId::from_str(document.id().tag())?;
    let output_id: OutputId = client.alias_output_id(alias_id).await.map_err(Error::ClientError)?;

    let output_response: OutputResponse = client.get_output(&output_id).await.map_err(Error::ClientError)?;
    let output: Output = Output::try_from(&output_response.output).map_err(OutputError::ConversionError)?;

    let alias_output: AliasOutput = if let Output::Alias(alias_output) = output {
      alias_output
    } else {
      return Err(Error::OutputError(OutputError::NotAnAliasOutput));
    };

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      // Update storage deposit if size changes.
      .with_minimum_storage_deposit(rent_structure)
      // State controller updates increment the state index.
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(document.pack()?);

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    let updated_alias_output: Output = alias_output_builder.finish_output().map_err(OutputError::BuildError)?;

    let block = client
      .publish_outputs(secret_manager, vec![updated_alias_output])
      .await?;

    Ok(block)
  }

  fn output_ids_from_block(block: Block) -> Vec<OutputId> {
    if let Payload::Transaction(tx) = block.payload().unwrap() {
      let TransactionEssence::Regular(regular) = tx.essence();
      let mut output_ids = Vec::new();
      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(_) = output {
          output_ids.push(OutputId::new(tx.id(), index.try_into().unwrap()).unwrap());
        }
      }
      output_ids
    } else {
      panic!("not a tx payload")
    }
  }

  async fn assert_spent_status(client: &Client, output_id: OutputId, is_spent: bool) {
    let metadata = client.get_output_metadata(&output_id).await.unwrap();
    assert_eq!(
      metadata.is_spent, is_spent,
      "expected {output_id} to have is_spent: {is_spent}"
    );
  }

  #[tokio::test]
  async fn test_publish_update() {
    let client: Client = client();
    let (address, secret_manager) = get_address_with_funds(&client).await;
    let document = generate_document(&valid_did());
    let (original_block, mut document) = publish_document(&client, document, address, &secret_manager)
      .await
      .unwrap();

    let method_url = document
      .resolve_method("#key-1", Some(MethodScope::VerificationMethod))
      .unwrap()
      .id()
      .to_owned();

    document
      .attach_method_relationship(
        &method_url,
        identity_did::verification::MethodRelationship::Authentication,
      )
      .unwrap();

    let update_block = publish_update(&client, document.clone(), &secret_manager)
      .await
      .unwrap();

    let resolved = client.resolve(document.id()).await.unwrap();

    assert_eq!(document, resolved);

    let original_block_ids = output_ids_from_block(original_block);
    let update_block_ids = output_ids_from_block(update_block);

    assert_spent_status(&client, original_block_ids[0], true).await;
    assert_spent_status(&client, update_block_ids[0], false).await;
  }
}
