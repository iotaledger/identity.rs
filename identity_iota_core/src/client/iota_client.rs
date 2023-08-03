// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use iota_sdk::client::api::input_selection::Burn;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::types::block::protocol::ProtocolParameters;

use crate::block::address::Address;
use crate::block::output::unlock_condition::AddressUnlockCondition;
use crate::block::output::AliasId;
use crate::block::output::AliasOutput;
use crate::block::output::BasicOutputBuilder;
use crate::block::output::Output;
use crate::block::output::OutputId;
use crate::block::output::UnlockCondition;
use crate::block::Block;
use crate::client::identity_client::validate_network;
use crate::error::Result;
use crate::Error;
use crate::IotaDID;
use crate::IotaDocument;
use crate::IotaIdentityClient;
use crate::IotaIdentityClientExt;
use crate::NetworkName;

/// An extension trait for [`Client`] that provides helper functions for publication
/// and deletion of DID documents in Alias Outputs.
#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
pub trait IotaClientExt: IotaIdentityClient {
  /// Publish the given `alias_output` with the provided `secret_manager`, and returns
  /// the DID document extracted from the published block.
  ///
  /// Note that only the state controller of an Alias Output is allowed to update its state.
  /// This will attempt to move tokens to or from the state controller address to match
  /// the storage deposit amount specified on `alias_output`.
  ///
  /// This method modifies the on-ledger state.
  async fn publish_did_output(&self, secret_manager: &SecretManager, alias_output: AliasOutput)
    -> Result<IotaDocument>;

  /// Destroy the Alias Output containing the given `did`, sending its tokens to a new Basic Output
  /// unlockable by `address`.
  ///
  /// Note that only the governor of an Alias Output is allowed to destroy it.
  ///
  /// # WARNING
  ///
  /// This destroys the Alias Output and DID document, rendering them permanently unrecoverable.
  async fn delete_did_output(&self, secret_manager: &SecretManager, address: Address, did: &IotaDID) -> Result<()>;
}

#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
impl IotaClientExt for Client {
  async fn publish_did_output(
    &self,
    secret_manager: &SecretManager,
    alias_output: AliasOutput,
  ) -> Result<IotaDocument> {
    let block: Block = publish_output(self, secret_manager, alias_output)
      .await
      .map_err(|err| Error::DIDUpdateError("publish_did_output: publish failed", Some(Box::new(err))))?;
    let network: NetworkName = self.network_name().await?;

    IotaDocument::unpack_from_block(&network, &block)?
      .into_iter()
      .next()
      .ok_or(Error::DIDUpdateError(
        "publish_did_output: no document found in published block",
        None,
      ))
  }

  async fn delete_did_output(&self, secret_manager: &SecretManager, address: Address, did: &IotaDID) -> Result<()> {
    validate_network(self, did).await?;

    let alias_id: AliasId = AliasId::from(did);
    let (output_id, alias_output) = self.get_alias_output(alias_id).await?;

    let basic_output = BasicOutputBuilder::new_with_amount(alias_output.amount())
      .with_native_tokens(alias_output.native_tokens().clone())
      .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
      .finish_output(self.deref().get_token_supply().await.map_err(Error::TokenSupplyError)?)
      .map_err(Error::BasicOutputBuildError)?;

    let block: Block = self
      .build_block()
      .with_secret_manager(secret_manager)
      .with_input(output_id.into())
      .map_err(|err| Error::DIDUpdateError("delete_did_output: invalid block input", Some(Box::new(err))))?
      .with_outputs(vec![basic_output])
      .map_err(|err| Error::DIDUpdateError("delete_did_output: invalid block output", Some(Box::new(err))))?
      .with_burn(Burn::new().add_alias(alias_id))
      .finish()
      .await
      .map_err(|err| Error::DIDUpdateError("delete_did_output: publish failed", Some(Box::new(err))))?;
    let _ = self
      .retry_until_included(&block.id(), None, None)
      .await
      .map_err(|err| {
        Error::DIDUpdateError(
          "delete_did_output: publish retry failed or timed-out",
          Some(Box::new(err)),
        )
      })?;

    Ok(())
  }
}

#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
impl IotaIdentityClient for Client {
  async fn get_protocol_parameters(&self) -> Result<ProtocolParameters> {
    self
      .deref()
      .get_protocol_parameters()
      .await
      .map_err(Error::ProtocolParametersError)
  }

  async fn get_alias_output(&self, id: AliasId) -> Result<(OutputId, AliasOutput)> {
    let output_id: OutputId = self.alias_output_id(id).await.map_err(Error::DIDResolutionError)?;
    let output: Output = self
      .get_output(&output_id)
      .await
      .map_err(Error::DIDResolutionError)?
      .into_output();

    if let Output::Alias(alias_output) = output {
      Ok((output_id, alias_output))
    } else {
      Err(Error::NotAnAliasOutput(output_id))
    }
  }
}

/// Publishes an `alias_output`.
/// Returns the block that the output was included in.
async fn publish_output(
  client: &Client,
  secret_manager: &SecretManager,
  alias_output: AliasOutput,
) -> iota_sdk::client::error::Result<Block> {
  let block: Block = client
    .build_block()
    .with_secret_manager(secret_manager)
    .with_outputs(vec![alias_output.into()])?
    .finish()
    .await?;

  let _ = client.retry_until_included(&block.id(), None, None).await?;

  Ok(block)
}
