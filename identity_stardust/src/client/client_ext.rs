// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::ops::Deref;

use identity_did::did::DIDError;
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

/// An extension trait for a [`Client`] that provides helper functions for publication
/// and resolution of DID documents in Alias Outputs.
///
/// This trait is only meant to be used rather than implemented.
#[async_trait::async_trait]
pub trait StardustClientExt: Sync {
  /// Returns a reference to a [`Client`].
  fn client(&self) -> &Client;

  /// Create a DID with a new Alias Output containing the given `document`.
  ///
  /// The `address` will be set as the state controller and governor unlock conditions.
  /// The minimum required token deposit amount will be set according to the given
  /// `rent_structure`, which will be fetched from the node if not provided.
  /// The returned Alias Output can be further customized before publication, if desired.
  ///
  /// NOTE: this does *not* publish the Alias Output. See [`publish_did_output`](StardustClientExt::publish_did_output).
  ///
  /// # Errors
  ///
  /// - Returns an [`Error::DIDUpdateError`] when retrieving the `RentStructure` fails.
  /// - Returns an [`Error::AliasOutputBuildError`] when building the Alias Output fails.
  async fn new_did(
    &self,
    address: Address,
    document: StardustDocument,
    rent_structure: Option<RentStructure>,
  ) -> Result<AliasOutput> {
    let rent_structure: RentStructure = if let Some(inner) = rent_structure {
      inner
    } else {
      self
        .client()
        .get_rent_structure()
        .await
        .map_err(Error::DIDUpdateError)?
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

  /// Returns the updated Alias Output for further customization and publication. The storage deposit
  /// on the output is unchanged. If the size of the document increased, the amount must be increased manually.
  ///
  /// NOTE: this does *not* publish the updated Alias Output. See
  /// [`publish_did_output`](StardustClientExt::publish_did_output).
  ///
  /// # Errors
  ///
  /// Returns `Err` when failing to resolve the DID contained in `document`.
  async fn update_did(&self, document: StardustDocument) -> Result<AliasOutput> {
    let (alias_id, _, alias_output) = resolve_alias_output(self.client(), document.id()).await?;

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(document.pack()?);

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    alias_output_builder.finish().map_err(Error::AliasOutputBuildError)
  }

  /// Resolves the Alias Output associated to the `did`, removes the DID document,
  /// and publishes the output. This effectively deactivates the DID.
  /// Deactivating does not destroy the output. Hence, a deactivated DID can be
  /// re-activated by updating the contained document.
  ///
  /// The storage deposit on the output is left unchanged.
  ///
  /// # Errors
  ///
  /// Returns `Err` when failing to resolve the `did`.
  async fn deactivate_did_output(&self, secret_manager: &SecretManager, did: &StardustDID) -> Result<()> {
    let (alias_id, _, alias_output) = resolve_alias_output(self.client(), did).await?;

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(Vec::new());

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    let alias_output: AliasOutput = alias_output_builder.finish().map_err(Error::AliasOutputBuildError)?;

    let _ = publish_output(self.client(), secret_manager, alias_output).await?;

    Ok(())
  }

  /// Publish the given `alias_output` with the provided `secret_manager`
  /// and returns the block they were published in.
  ///
  /// Needs to be called by the state controller of the Alias Output.
  ///
  /// This method modifies the on-ledger state.
  async fn publish_did_output(
    &self,
    secret_manager: &SecretManager,
    alias_output: AliasOutput,
  ) -> Result<StardustDocument> {
    let block: Block = publish_output(self.client(), secret_manager, alias_output).await?;

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
  ///
  /// Note that only the governor of an Alias Output is allowed to destroy it.
  ///
  /// # WARNING
  ///
  /// This destroys the DID Document and the Alias Output and renders the DID permanently unrecoverable.
  async fn delete_did_output(&self, secret_manager: &SecretManager, address: Address, did: &StardustDID) -> Result<()> {
    let client: &Client = self.client();

    let (_, output_id, alias_output) = resolve_alias_output(client, did).await?;

    let basic_output = BasicOutputBuilder::new_with_amount(alias_output.amount())
      .map_err(Error::BasicOutputBuildError)?
      .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
      .finish_output()
      .map_err(Error::BasicOutputBuildError)?;

    client
      .block()
      .with_secret_manager(secret_manager)
      .with_input(output_id.into())
      .map_err(Error::DIDUpdateError)?
      .with_outputs(vec![basic_output])
      .map_err(Error::DIDUpdateError)?
      .finish()
      .await
      .map_err(Error::DIDUpdateError)?;

    Ok(())
  }

  /// Resolve a [`StardustDID`] to a [`StardustDocument`].
  ///
  /// # Errors
  ///
  /// - Returns a [`NetworkMismatch`](Error::NetworkMismatch) error if the DID's and the client's network do not match.
  /// - Returns a [`NotFound`](iota_client::Error::NotFound) error if the associated Alias Output wasn't found.
  async fn resolve_did(&self, did: &StardustDID) -> Result<StardustDocument> {
    let network_hrp: String = get_network_hrp(self.client()).await?;

    if did.network_str() != network_hrp.as_str() {
      return Err(Error::NetworkMismatch {
        expected: did.network_str().to_owned(),
        actual: network_hrp,
      });
    }

    let (_, _, alias_output) = resolve_alias_output(self.client(), did).await?;

    if alias_output.state_metadata().is_empty() {
      let mut empty_document: StardustDocument = StardustDocument::new_with_id(did.to_owned());
      empty_document.metadata.deactivated = Some(true);

      Ok(empty_document)
    } else {
      let document: &[u8] = alias_output.state_metadata();
      StardustDocument::unpack(did, document)
    }
  }

  /// Resolve a [`StardustDID`] to an [`AliasOutput`].
  ///
  /// # Errors
  ///
  /// - Returns a [`NetworkMismatch`](Error::NetworkMismatch) error if the DID's and the client's network do not match.
  /// - Returns a [`NotFound`](iota_client::Error::NotFound) error if the associated Alias Output wasn't found.
  async fn resolve_did_output(&self, did: &StardustDID) -> Result<AliasOutput> {
    let network_hrp: String = get_network_hrp(self.client()).await?;

    if did.network_str() != network_hrp.as_str() {
      return Err(Error::NetworkMismatch {
        expected: did.network_str().to_owned(),
        actual: network_hrp,
      });
    }

    resolve_alias_output(self.client(), did)
      .await
      .map(|(_, _, alias_output)| alias_output)
  }

  /// Returns the network name of the connected node, which is the
  /// BECH32 human-readable part (HRP) of the network.
  ///
  /// For the IOTA main network this is `iota` and for the Shimmer network it is `smr`.
  async fn network_name(&self) -> Result<NetworkName> {
    get_network_hrp(self.client()).await.and_then(NetworkName::try_from)
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

/// Publishes an `alias_output`.
/// Returns the block that the output was included in.
async fn publish_output(client: &Client, secret_manager: &SecretManager, alias_output: AliasOutput) -> Result<Block> {
  let block: Block = client
    .block()
    .with_secret_manager(secret_manager)
    .with_outputs(vec![alias_output.into()])
    .map_err(Error::DIDUpdateError)?
    .finish()
    .await
    .map_err(Error::DIDUpdateError)?;

  let _ = client
    .retry_until_included(&block.id(), None, None)
    .await
    .map_err(Error::DIDUpdateError)?;

  Ok(block)
}

/// Get the BECH32 HRP from the client's network.
async fn get_network_hrp(client: &Client) -> Result<String> {
  client
    .get_network_info()
    .await
    .map_err(Error::DIDResolutionError)?
    .bech32_hrp
    .ok_or_else(|| Error::InvalidNetworkName("".to_owned()))
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
          AliasId::from(
            OutputId::new(
              tx_payload.id(),
              index
                .try_into()
                .map_err(|_| Error::OutputIdConversionError(format!("the output index {index} must fit into a u16")))?,
            )
            .map_err(|err| Error::OutputIdConversionError(err.to_string()))?,
          )
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

/// Resolve a did into an Alias Output and the associated identifiers.
async fn resolve_alias_output(client: &Client, did: &StardustDID) -> Result<(AliasId, OutputId, AliasOutput)> {
  let tag_bytes: [u8; StardustDID::TAG_BYTES_LEN] =
    prefix_hex::decode(did.tag()).map_err(|_| DIDError::InvalidMethodId)?;
  let alias_id: AliasId = AliasId::new(tag_bytes);
  let output_id: OutputId = client
    .alias_output_id(alias_id)
    .await
    .map_err(Error::DIDResolutionError)?;
  let output_response: OutputResponse = client.get_output(&output_id).await.map_err(Error::DIDResolutionError)?;
  let output: Output = Output::try_from(&output_response.output).map_err(Error::OutputConversionError)?;

  if let Output::Alias(alias_output) = output {
    Ok((alias_id, output_id, alias_output))
  } else {
    Err(Error::NotAnAliasOutput(output_id))
  }
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

    let document = client.publish_did_output(&secret_manager, output).await.unwrap();

    let resolved = client.resolve_did(document.id()).await.unwrap();

    assert_eq!(document, resolved);
  }

  #[tokio::test]
  async fn test_client_publish_update() {
    let client: Client = client();
    let (address, secret_manager) = get_address_with_funds(&client).await;
    let initial_document = generate_document(&valid_did());
    let rent_structure: RentStructure = client.get_rent_structure().await.unwrap();

    let output = client
      .new_did(address, initial_document, Some(rent_structure.clone()))
      .await
      .unwrap();

    let mut document = client.publish_did_output(&secret_manager, output).await.unwrap();

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

    let document: StardustDocument = client.publish_did_output(&secret_manager, alias_output).await.unwrap();

    let resolved = client.resolve_did(document.id()).await.unwrap();

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

    let document: StardustDocument = client.publish_did_output(&secret_manager, output).await.unwrap();

    client
      .delete_did_output(&secret_manager, address, document.id())
      .await
      .unwrap();

    client.consolidate_funds(&secret_manager, 0, 0..1).await.unwrap();

    // It takes time for the deletion to propagate.
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let error = client.resolve_did(document.id()).await.unwrap_err();

    assert!(matches!(error, Error::DIDResolutionError(iota_client::Error::NotFound)));

    let balance: u64 = get_address_balance(&client, &address_bech32).await;

    assert_eq!(initial_balance, balance);
  }
}
