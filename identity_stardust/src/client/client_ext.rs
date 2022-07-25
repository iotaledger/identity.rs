// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::CoreDID;
use iota_client::api_types::responses::OutputResponse;
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
use iota_client::block::output::OutputId;
use iota_client::block::output::UnlockCondition;
use iota_client::block::payload::transaction::TransactionEssence;
use iota_client::block::payload::Payload;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;

use crate::error::Result;
use crate::Error;
use crate::StardustDocument;

#[async_trait::async_trait]
pub trait StardustClientExt: Sync {
  fn client(&self) -> &Client;

  // TODO: Return StardustDID eventually.
  // async fn publish(
  //   &self,
  //   address: Address,
  //   secret_manager: &SecretManager,
  //   document: StardustDocument,
  // ) -> Result<StardustDocument> {
  //   let byte_cost_config: ByteCostConfig = self.client().get_byte_cost_config().await.map_err(Error::ClientError)?;
  //   let alias_output: Output = AliasOutputBuilder::new_with_minimum_storage_deposit(byte_cost_config,
  // AliasId::null())?     .with_state_index(0)
  //     .with_foundry_counter(0)
  //     .with_state_metadata(document.clone().pack()?)
  //     .add_feature(Feature::Sender(SenderFeature::new(address)))
  //     .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
  //     .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
  //     .add_unlock_condition(UnlockCondition::StateControllerAddress(
  //       StateControllerAddressUnlockCondition::new(address),
  //     ))
  //     .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
  //       address,
  //     )))
  //     .finish_output()?;

  //   let block: Block = self
  //     .client()
  //     .block()
  //     .with_secret_manager(secret_manager)
  //     .with_outputs(vec![alias_output])
  //     .expect("TODO")
  //     .finish()
  //     .await
  //     .expect("TODO");

  //   let _ = self
  //     .client()
  //     .retry_until_included(&block.id(), None, None)
  //     .await
  //     .map_err(Error::ClientError)?;

  //   // TODO: Document panics.
  //   let alias_output_id_from_payload = |payload: &Payload| -> OutputId {
  //     match payload {
  //       Payload::Transaction(tx_payload) => {
  //         let TransactionEssence::Regular(regular) = tx_payload.essence();
  //         for (index, output) in regular.outputs().iter().enumerate() {
  //           if let Output::Alias(_alias_output) = output {
  //             return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
  //           }
  //         }
  //         panic!("No alias output in transaction essence")
  //       }
  //       _ => panic!("the payload should contain a transaction"),
  //     }
  //   };

  //   let alias_id: AliasId = AliasId::from(alias_output_id_from_payload(
  //     block.payload().expect("the block should contain a payload"),
  //   ));

  //   let did: CoreDID = StardustDocument::alias_id_to_did(&alias_id)?;

  //   Ok(StardustDocument(document.0.map(|_| did.clone(), |o| o)))
  // }

  // TODO: Take StardustDID eventually.
  async fn resolve(&self, alias_id: AliasId) -> Result<StardustDocument> {
    let output_id: OutputId = self
      .client()
      .alias_output_id(alias_id)
      .await
      .map_err(Error::ClientError)?;
    let response: OutputResponse = self.client().get_output(&output_id).await.map_err(Error::ClientError)?;
    let output: Output = Output::try_from(&response.output)
      .map_err(iota_client::Error::from)
      .map_err(Error::ClientError)?;

    let did = StardustDocument::alias_id_to_did(&alias_id)?;
    StardustDocument::deserialize_from_output(&did, &output)
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

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;
  use iota_client::block::address::Address;
  use iota_client::block::output::Output;
  use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
  use iota_client::crypto::keys::bip39;
  use iota_client::node_api::indexer::query_parameters::QueryParameter;
  use iota_client::secret::mnemonic::MnemonicSecretManager;
  use iota_client::secret::SecretManager;
  use iota_client::Client;

  use crate::StardustCoreDocument;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustDocumentMetadata;

  use super::StardustClientExt;

  // TODO: Change to private tangle in CI; detect CI via env var?.
  static ENDPOINT: &str = "https://api.alphanet.iotaledger.net";
  static FAUCET_URL: &str = "https://faucet.alphanet.iotaledger.net/api/enqueue";

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
    let address_bech32 = address.to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    loop {
      println!("Requesting funds");
      iota_client::request_funds_from_faucet(FAUCET_URL, &address_bech32)
        .await
        .unwrap();
      tokio::time::sleep(std::time::Duration::from_secs(15)).await;

      let balance = get_address_balance(client, &address_bech32).await;
      println!("balance for {address_bech32} is {balance}");
      if balance > 0 {
        break;
      }
    }

    (address, secret_manager)
  }

  fn valid_did() -> StardustDID {
    "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  // async fn publish_document(client: impl StardustClientExt) -> crate::error::Result<StardustDocument> {
  //   let document = generate_document(&valid_did());
  //   let (address, secret_manager) = get_address_with_funds(client.client()).await;

  //   client.publish(address, &secret_manager, document.clone()).await
  // }

  // #[tokio::test]
  // async fn test_publish_resolve() {
  //   let client: Client = client().await;
  //   let document = publish_document(&client).await.unwrap();
  //   let alias_id = StardustDocument::did_to_alias_id(document.0.id()).unwrap();
  //   let resolved = client.resolve(alias_id).await.unwrap();

  //   assert_eq!(document, resolved);
  // }
}
