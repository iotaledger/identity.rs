// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use iota_client::api_types::responses::OutputResponse;
use iota_client::secret::SecretManager;
use iota_client::Client;

use crate::block::address::Address;
use crate::block::output::unlock_condition::AddressUnlockCondition;
use crate::block::output::AliasId;
use crate::block::output::AliasOutput;
use crate::block::output::BasicOutputBuilder;
use crate::block::output::Output;
use crate::block::output::OutputId;
use crate::block::output::RentStructure;
use crate::block::output::UnlockCondition;
use crate::block::payload::transaction::TransactionEssence;
use crate::block::payload::Payload;
use crate::block::Block;
use crate::client::identity_client::validate_network;
use crate::error::Result;
use crate::Error;
use crate::NetworkName;
use crate::StardustDID;
use crate::StardustDocument;
use crate::StardustIdentityClient;
use crate::StardustIdentityClientExt;

/// An extension trait for [`Client`] that provides helper functions for publication
/// and deletion of DID documents in Alias Outputs.
#[async_trait::async_trait(?Send)]
pub trait StardustClientExt: StardustIdentityClient {
  /// Publish the given `alias_output` with the provided `secret_manager`, and returns
  /// the DID document extracted from the published block.
  ///
  /// Note that only the state controller of an Alias Output is allowed to update its state.
  /// This will attempt to move tokens to or from the state controller address to match
  /// the storage deposit amount specified on `alias_output`.
  ///
  /// This method modifies the on-ledger state.
  async fn publish_did_output(
    &self,
    secret_manager: &SecretManager,
    alias_output: AliasOutput,
  ) -> Result<StardustDocument>;

  /// Destroy the Alias Output containing the given `did`, sending its tokens to a new Basic Output
  /// unlockable by `address`.
  ///
  /// Note that only the governor of an Alias Output is allowed to destroy it.
  ///
  /// # WARNING
  ///
  /// This destroys the Alias Output and DID document, rendering the DID permanently unrecoverable.
  async fn delete_did_output(&self, secret_manager: &SecretManager, address: Address, did: &StardustDID) -> Result<()>;
}

/// An extension trait for [`Client`] that provides helper functions for publication
/// and deletion of DID documents in Alias Outputs.
#[async_trait::async_trait(?Send)]
impl StardustClientExt for Client {
  async fn publish_did_output(
    &self,
    secret_manager: &SecretManager,
    alias_output: AliasOutput,
  ) -> Result<StardustDocument> {
    let block: Block = publish_output(self, secret_manager, alias_output)
      .await
      .map_err(|err| Error::DIDUpdateError("publish_did_output: publish failed", Some(err)))?;
    let network: NetworkName = self.network_name().await?;

    extract_documents_from_block(&network, &block)?
      .into_iter()
      .next()
      .ok_or(Error::DIDUpdateError(
        "publish_did_output: no document found in published block",
        None,
      ))
  }

  async fn delete_did_output(&self, secret_manager: &SecretManager, address: Address, did: &StardustDID) -> Result<()> {
    validate_network(self, did).await?;

    let alias_id: AliasId = AliasId::try_from(did)?;
    let (output_id, alias_output) = self.get_alias_output(alias_id).await?;

    let basic_output = BasicOutputBuilder::new_with_amount(alias_output.amount())
      .map_err(Error::BasicOutputBuildError)?
      .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
      .finish_output()
      .map_err(Error::BasicOutputBuildError)?;

    let block: Block = self
      .block()
      .with_secret_manager(secret_manager)
      .with_input(output_id.into())
      .map_err(|err| Error::DIDUpdateError("delete_did_output: invalid block input", Some(err)))?
      .with_outputs(vec![basic_output])
      .map_err(|err| Error::DIDUpdateError("delete_did_output: invalid block output", Some(err)))?
      .finish()
      .await
      .map_err(|err| Error::DIDUpdateError("delete_did_output: publish failed", Some(err)))?;
    let _ = self
      .retry_until_included(&block.id(), None, None)
      .await
      .map_err(|err| Error::DIDUpdateError("delete_did_output: publish retry failed or timed-out", Some(err)))?;

    Ok(())
  }
}

#[async_trait::async_trait(?Send)]
impl StardustIdentityClient for Client {
  async fn get_network_hrp(&self) -> Result<String> {
    self
      .get_network_info()
      .await
      .map_err(Error::DIDResolutionError)?
      .bech32_hrp
      .ok_or_else(|| Error::InvalidNetworkName("".to_owned()))
  }

  async fn get_alias_output(&self, id: AliasId) -> Result<(OutputId, AliasOutput)> {
    let output_id: OutputId = self.alias_output_id(id).await.map_err(Error::DIDResolutionError)?;
    let output_response: OutputResponse = self.get_output(&output_id).await.map_err(Error::DIDResolutionError)?;
    let output: Output = Output::try_from(&output_response.output).map_err(Error::OutputConversionError)?;

    if let Output::Alias(alias_output) = output {
      Ok((output_id, alias_output))
    } else {
      Err(Error::NotAnAliasOutput(output_id))
    }
  }

  async fn get_rent_structure(&self) -> Result<RentStructure> {
    Client::get_rent_structure(self)
      .await
      .map_err(|err| Error::DIDUpdateError("get_rent_structure failed", Some(err)))
  }
}

/// Publishes an `alias_output`.
/// Returns the block that the output was included in.
async fn publish_output(
  client: &Client,
  secret_manager: &SecretManager,
  alias_output: AliasOutput,
) -> iota_client::Result<Block> {
  let block: Block = client
    .block()
    .with_secret_manager(secret_manager)
    .with_outputs(vec![alias_output.into()])?
    .finish()
    .await?;

  let _ = client.retry_until_included(&block.id(), None, None).await?;

  Ok(block)
}

/// Returns all DID documents of the Alias Outputs contained in the payload's transaction, if any.
fn extract_documents_from_block(network: &NetworkName, block: &Block) -> Result<Vec<StardustDocument>> {
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
                .map_err(|_| Error::OutputIdConversionError(format!("output index {index} must fit into a u16")))?,
            )
            .map_err(|err| Error::OutputIdConversionError(err.to_string()))?,
          )
        } else {
          alias_output.alias_id().to_owned()
        };

        let did: StardustDID = StardustDID::new(alias_id.deref(), network);
        documents.push(StardustDocument::unpack(&did, alias_output.state_metadata(), true)?);
      }
    }
  }

  Ok(documents)
}
