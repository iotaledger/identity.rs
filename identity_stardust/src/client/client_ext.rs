// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::ops::Deref;
use std::str::FromStr;

use iota_client::api_types::responses::OutputResponse;
use iota_client::block::address::Address;
use iota_client::block::output::feature::IssuerFeature;
use iota_client::block::output::feature::SenderFeature;
use iota_client::block::output::unlock_condition::AddressUnlockCondition;
use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::BasicOutputBuilder;
use iota_client::block::output::Feature;
use iota_client::block::output::Output;
use iota_client::block::output::OutputId;
use iota_client::block::output::RentStructure;
use iota_client::block::output::UnlockCondition;
use iota_client::block::payload::transaction::TransactionEssence;
use iota_client::block::payload::Payload;
use iota_client::block::Block;
use iota_client::secret::SecretManager;
use iota_client::Client;

use crate::error::Result;
use crate::Error;
use crate::NetworkName;
use crate::StardustDID;
use crate::StardustDocument;

/// An extension trait for a [`Client`] that provides helper functions publication and resolution of
/// DID documents in Alias Outputs.
#[async_trait::async_trait]
pub trait StardustClientExt: Sync {
  /// Returns a reference to a [`Client`].
  fn client(&self) -> &Client;

  /// Create a new DID in an Alias Output from the given `document`.
  ///
  /// The alias output's state and governance will be controlled by `address`.
  /// The returned Alias Output can be further customized before publication, if desired.
  /// If the `rent_structure` is not provided, it will be fetched from the node.
  ///
  /// This method does not modify the on-ledger state.
  async fn new_did(
    &self,
    address: Address,
    document: StardustDocument,
    rent_structure: Option<RentStructure>,
  ) -> Result<AliasOutput> {
    let rent_structure: RentStructure = if let Some(inner) = rent_structure {
      inner
    } else {
      self.client().get_rent_structure().await.map_err(Error::ClientError)?
    };

    AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
      .map_err(Error::AliasOutputBuildError)?
      .with_state_index(0)
      .with_foundry_counter(0)
      .with_state_metadata(document.pack()?)
      .add_feature(Feature::Sender(SenderFeature::new(address)))
      .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(address),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        address,
      )))
      .finish()
      .map_err(Error::AliasOutputBuildError)
  }

  /// Resolves the alias output associated to the DID in `document` and updates it with that document.
  ///
  /// Returns the updated alias output for further customization and publication. The storage deposit
  /// on the output is unchanged. If the size of the document increased, the amount has to be increased.
  ///
  /// This method does not modify the on-ledger state.
  async fn update_did(&self, document: StardustDocument) -> Result<AliasOutput> {
    let alias_id: AliasId = AliasId::from_str(document.id().tag())?;
    let output_id: OutputId = self
      .client()
      .alias_output_id(alias_id)
      .await
      .map_err(Error::ClientError)?;

    let output_response: OutputResponse = self.client().get_output(&output_id).await.map_err(Error::ClientError)?;
    let output: Output = Output::try_from(&output_response.output).map_err(Error::OutputConversionError)?;

    let alias_output: AliasOutput = if let Output::Alias(alias_output) = output {
      alias_output
    } else {
      return Err(Error::NotAnAliasOutput);
    };

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(document.pack()?);

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    alias_output_builder.finish().map_err(Error::AliasOutputBuildError)
  }

  /// Publish the given `alias_outputs` with the provided `secret_manager`
  /// and returns the block they were published in.
  ///
  /// This method modifies the on-ledger state.
  async fn publish_did(&self, secret_manager: &SecretManager, alias_output: AliasOutput) -> Result<StardustDocument> {
    let block: Block = self
      .client()
      .block()
      .with_secret_manager(secret_manager)
      .with_outputs(vec![alias_output.into()])
      .map_err(Error::ClientError)?
      .finish()
      .await
      .map_err(Error::ClientError)?;

    let _ = self
      .client()
      .retry_until_included(&block.id(), None, None)
      .await
      .map_err(Error::ClientError)?;

    Ok(
      documents_from_block(self.client(), &block)
        .await?
        .into_iter()
        .next()
        .expect("there should be exactly one document"),
    )
  }

  /// Consume the Alias Output containing the given `did`, sending its tokens to a new Basic Output
  /// unlockable by `address`.
  /// WARNING: This destroys the DID Document and the Alias Output and renders the DID permanently unrecoverable.
  // TODO: Return one of (), Block, Output or OutputId?
  async fn delete_did(&self, secret_manager: &SecretManager, address: Address, did: &StardustDID) -> Result<()> {
    let client = self.client();

    let alias_id: AliasId = AliasId::from_str(did.tag())?;
    let output_id: OutputId = client.alias_output_id(alias_id).await?;
    let output_response: OutputResponse = client.get_output(&output_id).await?;
    let output: Output = Output::try_from(&output_response.output).map_err(Error::OutputConversionError)?;

    let basic_output = BasicOutputBuilder::new_with_amount(output.amount())?
      .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
      .finish_output()?;

    client
      .block()
      .with_secret_manager(secret_manager)
      .with_input(output_id.into())?
      .with_outputs(vec![basic_output])?
      .finish()
      .await?;

    Ok(())
  }

  /// Resolve a [`StardustDID`] to a [`StardustDocument`].
  async fn resolve(&self, did: &StardustDID) -> Result<StardustDocument> {
    // TODO: Replace usage of HRP with network_id -> network_name mapping?
    let network_hrp: String = get_network_hrp(self.client()).await?;

    if did.network_str() != network_hrp.as_str() {
      return Err(Error::NetworkMismatch {
        expected: did.network_str().to_owned(),
        actual: network_hrp,
      });
    }

    let alias_id: AliasId = AliasId::from_str(did.tag())?;

    let output_id: OutputId = self
      .client()
      .alias_output_id(alias_id)
      .await
      .map_err(Error::ClientError)?;
    let response: OutputResponse = self.client().get_output(&output_id).await.map_err(Error::ClientError)?;
    let output: Output = Output::try_from(&response.output).map_err(Error::OutputConversionError)?;

    StardustDocument::unpack_from_output(did, &output)
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

/// Get the BECH32 HRP from the client's network.
async fn get_network_hrp(client: &Client) -> Result<String> {
  client
    .get_network_info()
    .await
    .map_err(Error::ClientError)?
    .bech32_hrp
    .ok_or(Error::InvalidNetworkName)
}

/// Returns all DID documents of the Alias Outputs contained in the payload's transaction, if any.
async fn documents_from_block(client: &Client, block: &Block) -> Result<Vec<StardustDocument>> {
  let network_hrp: String = get_network_hrp(client).await?;
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

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_did::did::DID;
  use identity_did::document::Document;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodScope;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;
  use iota_client::block::address::Address;
  use iota_client::block::output::AliasOutput;
  use iota_client::block::output::AliasOutputBuilder;
  use iota_client::block::output::Output;
  use iota_client::block::output::RentStructure;
  use iota_client::constants::SHIMMER_TESTNET_BECH32_HRP;
  use iota_client::crypto::keys::bip39;
  use iota_client::node_api::indexer::query_parameters::QueryParameter;
  use iota_client::secret::mnemonic::MnemonicSecretManager;
  use iota_client::secret::SecretManager;
  use iota_client::Client;

  use crate::client::client_ext::get_network_hrp;
  use crate::Error;
  use crate::StardustCoreDocument;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustDocumentMetadata;
  use crate::StardustVerificationMethod;

  use super::StardustClientExt;

  // TODO: Change to private tangle in CI; detect CI via env var?.
  // TODO: Change to shimmer testnet after bee-api-types and nodes agree again.
  // Somehow linked to this: https://github.com/iotaledger/tips/pull/57#discussion_r921955443.
  static ENDPOINT: &str = "https://api.alphanet.iotaledger.net/";
  static FAUCET_URL: &str = "https://faucet.alphanet.iotaledger.net/api/enqueue";
  // static ENDPOINT: &str = "https://api.testnet.shimmer.network/";
  // static FAUCET_URL: &str = "https://faucet.testnet.shimmer.network/api/enqueue";

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

  async fn request_faucet_funds(client: &Client, address: Address) {
    let address_bech32 = address.to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    iota_client::request_funds_from_faucet(FAUCET_URL, &address_bech32)
      .await
      .unwrap();

    tokio::time::timeout(std::time::Duration::from_secs(45), async {
      loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let balance = get_address_balance(client, &address_bech32).await;
        if balance > 0 {
          break;
        }
      }
      Ok::<(), anyhow::Error>(())
    })
    .await
    .unwrap()
    .unwrap();
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

  fn valid_did() -> StardustDID {
    "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  #[tokio::test]
  async fn test_client_publish_and_resolve() {
    let client: Client = client();
    let (address, secret_manager) = get_address_with_funds(&client).await;
    let document = generate_document(&valid_did());

    let output = client.new_did(address, document, None).await.unwrap();

    let document = client.publish_did(&secret_manager, output).await.unwrap();

    let resolved = client.resolve(document.id()).await.unwrap();

    assert_eq!(document, resolved);
  }

  #[tokio::test]
  async fn test_client_publish_update() {
    let client: Client = client();
    let (address, secret_manager) = get_address_with_funds(&client).await;
    let initial_document = generate_document(&valid_did());
    let rent_structure: RentStructure = client.get_rent_structure().await.map_err(Error::ClientError).unwrap();

    let output = client
      .new_did(address, initial_document, Some(rent_structure.clone()))
      .await
      .unwrap();

    let mut document = client.publish_did(&secret_manager, output).await.unwrap();

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

    let alias_output: AliasOutput = client.update_did(document).await.unwrap();

    let rent_structure = client.get_rent_structure().await.unwrap();

    let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
      .with_minimum_storage_deposit(rent_structure)
      .finish()
      .unwrap();

    let document: StardustDocument = client.publish_did(&secret_manager, alias_output).await.unwrap();

    let resolved = client.resolve(document.id()).await.unwrap();

    assert_eq!(document, resolved);
  }

  #[tokio::test]
  async fn test_client_delete() {
    let client: Client = client();
    let network_hrp: String = get_network_hrp(&client).await.unwrap();

    let (address, secret_manager) = get_address_with_funds(&client).await;
    let address_bech32: String = address.to_bech32(network_hrp);
    let initial_balance: u64 = get_address_balance(&client, &address_bech32).await;

    let initial_document = generate_document(&valid_did());

    let output = client.new_did(address, initial_document, None).await.unwrap();

    let document: StardustDocument = client.publish_did(&secret_manager, output).await.unwrap();

    client
      .delete_did(&secret_manager, address, document.id())
      .await
      .unwrap();

    client.consolidate_funds(&secret_manager, 0, 0..1).await.unwrap();

    // It takes time for the deletion to propagate.
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let error = client.resolve(document.id()).await.unwrap_err();

    assert!(matches!(error, Error::ClientError(iota_client::Error::NotFound)));

    let balance: u64 = get_address_balance(&client, &address_bech32).await;

    assert_eq!(initial_balance, balance);
  }
}
