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
  async fn new_did_output(
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
  async fn update_did_output(&self, document: StardustDocument) -> Result<AliasOutput> {
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
