// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_did::did::DID;
use iota_client::api_types::responses::OutputResponse;
use iota_client::block::output::AliasId;
use iota_client::block::output::ByteCostConfig;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::payload::transaction::TransactionEssence;
use iota_client::block::payload::Payload;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;

use crate::error::Result;
use crate::Error;
use crate::StardustDID;
use crate::StardustDocument;

#[async_trait::async_trait]
pub trait StardustClientExt: Sync {
  fn client(&self) -> &Client;

  // TODO: Return StardustDID eventually.
  async fn publish_outputs(&self, secret_manager: &SecretManager, alias_outputs: Vec<Output>) -> Result<Block> {
    StardustClient::from(self.client())
      .publish_outputs(secret_manager, alias_outputs)
      .await

    // let block: Block = self
    //   .client()
    //   .block()
    //   .with_secret_manager(secret_manager)
    //   .with_outputs(outputs)
    //   .expect("TODO")
    //   .finish()
    //   .await
    //   .expect("TODO");

    // let _ = self
    //   .client()
    //   .retry_until_included(&block.id(), None, None)
    //   .await
    //   .map_err(Error::ClientError)?;

    // // TODO: Document panics.
    // let alias_output_id_from_payload = |payload: &Payload| -> OutputId {
    //   match payload {
    //     Payload::Transaction(tx_payload) => {
    //       let TransactionEssence::Regular(regular) = tx_payload.essence();
    //       for (index, output) in regular.outputs().iter().enumerate() {
    //         if let Output::Alias(_alias_output) = output {
    //           return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
    //         }
    //       }
    //       panic!("No alias output in transaction essence")
    //     }
    //     _ => panic!("the payload should contain a transaction"),
    //   }
    // };

    // let alias_id: AliasId = AliasId::from(alias_output_id_from_payload(
    //   block.payload().expect("the block should contain a payload"),
    // ));

    // let did: CoreDID = StardustDocument::alias_id_to_did(&alias_id)?;

    // Ok(StardustDocument(document.0.map(|_| did.clone(), |o| o)))
  }

  // /// Consumes the alias output identified by the DID in `document` and creates a new one with `document`.
  // ///
  // /// The state controller address of the alias output must be `address`, which must be managed by
  // /// `secret_manager`.
  // /// If the new document is larger than the old one, the required storage deposit might increase.
  // async fn publish_update(
  //   &self,
  //   address: Address,
  //   secret_manager: &SecretManager,
  //   document: StardustDocument,
  // ) -> Result<()> {
  //   let byte_cost_config: ByteCostConfig = StardustClient::from(self.client()).get_byte_cost_config().await?;

  //   let alias_id: AliasId = StardustDocument::did_to_alias_id(document.0.id())?;
  //   let output_id: OutputId = self
  //     .client()
  //     .alias_output_id(alias_id)
  //     .await
  //     .map_err(Error::ClientError)?;

  //   let output_response: OutputResponse = self.client().get_output(&output_id).await.map_err(Error::ClientError)?;
  //   let output: Output = Output::try_from(&output_response.output).map_err(OutputError::ConversionError)?;

  //   // TODO: Unnecessary step?
  //   // let resolved: StardustDocument = StardustDocument::deserialize_from_output(&alias_id, &output)?;

  //   let alias_output: AliasOutput = if let Output::Alias(alias_output) = output {
  //     alias_output
  //   } else {
  //     return Err(Error::OutputError(OutputError::NotAnAliasOutput));
  //   };

  //   let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
  //     // Update storage deposit if size changes.
  //     .with_minimum_storage_deposit(byte_cost_config)
  //     // State controller updates increment the state index.
  //     .with_state_index(alias_output.state_index() + 1);

  //   if alias_output.alias_id().is_null() {
  //     alias_output_builder = alias_output_builder.with_alias_id(alias_id);
  //   }

  //   let updated_alias_output: Output = alias_output_builder.finish_output().map_err(OutputError::BuildError)?;

  //   let _ = StardustClient::from(self.client())
  //     .publish_output(secret_manager, updated_alias_output)
  //     .await?;

  //   Ok(())
  // }

  async fn resolve(&self, did: &StardustDID) -> Result<StardustDocument> {
    // TODO: Fix me.
    let alias_id = AliasId::from_str(&format!("0x{}", did.method_id()))?;

    let output_id: OutputId = self
      .client()
      .alias_output_id(alias_id)
      .await
      .map_err(Error::ClientError)
      .unwrap();
    let response: OutputResponse = self
      .client()
      .get_output(&output_id)
      .await
      .map_err(Error::ClientError)
      .unwrap();
    let output: Output = Output::try_from(&response.output)
      .map_err(iota_client::Error::from)
      .map_err(Error::ClientError)
      .unwrap();

    let did = StardustDocument::alias_id_to_did(&alias_id)?;
    StardustDocument::deserialize_from_output(&did, &output)
  }

  // TODO: Fix panics.
  fn alias_output_id_from_payload(payload: &Payload) -> OutputId {
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
      _ => panic!("the payload should contain a transaction"),
    }
  }

  fn alias_ids_from_payload(payload: &Payload) -> Result<Vec<AliasId>> {
    match payload {
      Payload::Transaction(tx_payload) => {
        let TransactionEssence::Regular(regular) = tx_payload.essence();
        let mut alias_ids = Vec::new();
        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Alias(_) = output {
            alias_ids.push(AliasId::from(OutputId::new(
              tx_payload.id(),
              index.try_into().expect("the output count should not exceed u16"),
            )?));
          }
        }
        Ok(alias_ids)
      }
      _ => Ok(Vec::new()),
    }
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

struct StardustClient<'client> {
  client: &'client Client,
}

impl<'client> From<&'client Client> for StardustClient<'client> {
  fn from(client: &'client Client) -> Self {
    StardustClient { client }
  }
}

impl<'client> StardustClient<'client> {
  async fn get_byte_cost_config(&self) -> Result<ByteCostConfig> {
    self.client.get_byte_cost_config().await.map_err(Error::ClientError)
  }

  async fn publish_outputs(&self, secret_manager: &SecretManager, alias_outputs: Vec<Output>) -> Result<Block> {
    let block: Block = self
      .client
      .block()
      .with_secret_manager(secret_manager)
      .with_outputs(alias_outputs)
      .map_err(Error::ClientError)?
      .finish()
      .await
      .map_err(Error::ClientError)?;

    // TODO: Should we return the block returned from this?
    let _ = self
      .client
      .retry_until_included(&block.id(), None, None)
      .await
      .map_err(Error::ClientError)?;

    Ok(block)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;
  use iota_client::block::address::Address;
  use iota_client::block::output::feature::IssuerFeature;
  use iota_client::block::output::feature::MetadataFeature;
  use iota_client::block::output::feature::SenderFeature;
  use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
  use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
  use iota_client::block::output::AliasId;
  use iota_client::block::output::AliasOutputBuilder;
  use iota_client::block::output::ByteCostConfig;
  use iota_client::block::output::Feature;
  use iota_client::block::output::Output;
  use iota_client::block::output::UnlockCondition;
  use iota_client::block::Block;
  use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
  use iota_client::crypto::keys::bip39;
  use iota_client::node_api::indexer::query_parameters::QueryParameter;
  use iota_client::secret::mnemonic::MnemonicSecretManager;
  use iota_client::secret::SecretManager;
  use iota_client::Client;

  use crate::Error;
  use crate::StardustCoreDocument;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustDocumentMetadata;

  use super::StardustClientExt;

  // TODO: Change to private tangle in CI; detect CI via env var?.
  static ENDPOINT: &str = "https://api.testnet.shimmer.network/";
  static FAUCET_URL: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

  fn generate_method(controller: &CoreDID, fragment: &str) -> VerificationMethod {
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
    let byte_cost_config: ByteCostConfig = client.get_byte_cost_config().await.map_err(Error::ClientError).unwrap();
    AliasOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config, AliasId::null())
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
    "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  async fn publish_document(client: impl StardustClientExt) -> crate::error::Result<StardustDocument> {
    let document = generate_document(&valid_did());
    let (address, secret_manager) = get_address_with_funds(client.client()).await;
    let output = alias_output(client.client(), address, &document).await;

    let block: Block = client.publish_outputs(&secret_manager, vec![output]).await?;

    let alias_ids = <Client as StardustClientExt>::alias_ids_from_payload(
      block.payload().expect("the block we published should have a payload"),
    )?;

    let alias_id = alias_ids.get(0).expect("there should be at least one alias output");
    let new_did = StardustDocument::alias_id_to_did(&alias_id)?;

    // TODO: Add StardustDocument::map instead?
    Ok(StardustDocument {
      document: document.document.map(|_| new_did.clone(), |o| o),
      metadata: document.metadata,
    })
  }

  #[tokio::test]
  async fn test_publish_resolve() {
    let client: Client = client();
    let document = publish_document(&client).await.unwrap();
    let resolved = client.resolve(document.id()).await.unwrap();

    assert_eq!(document, resolved);
  }
}
