// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::protocol::ProtocolParameters;

use crate::block::address::Address;
use crate::block::output::feature::SenderFeature;
use crate::block::output::unlock_condition::GovernorAddressUnlockCondition;
use crate::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use crate::block::output::AliasId;
use crate::block::output::AliasOutput;
use crate::block::output::AliasOutputBuilder;
use crate::block::output::Feature;
use crate::block::output::OutputId;
use crate::block::output::RentStructure;
use crate::block::output::UnlockCondition;
use crate::Error;
use crate::IotaDID;
use crate::IotaDocument;
use crate::NetworkName;
use crate::Result;

/// Helper functions necessary for the [`IotaIdentityClientExt`] trait.
#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
pub trait IotaIdentityClient {
  /// Resolve an Alias identifier, returning its latest [`OutputId`] and [`AliasOutput`].
  async fn get_alias_output(&self, alias_id: AliasId) -> Result<(OutputId, AliasOutput)>;
  /// Get the protocol parameters of the node we are trying to connect to.
  async fn get_protocol_parameters(&self) -> Result<ProtocolParameters>;
}

/// An extension trait that provides helper functions for publication
/// and resolution of DID documents in Alias Outputs.
///
/// This trait is not intended to be implemented directly, a blanket implementation is
/// provided for [`IotaIdentityClient`] implementers.
#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
pub trait IotaIdentityClientExt: IotaIdentityClient {
  /// Create a DID with a new Alias Output containing the given `document`.
  ///
  /// The `address` will be set as the state controller and governor unlock conditions.
  /// The minimum required token deposit amount will be set according to the given
  /// `rent_structure`, which will be fetched from the node if not provided.
  /// The returned Alias Output can be further customised before publication, if desired.
  ///
  /// NOTE: This does *not* publish the Alias Output.
  ///
  /// # Errors
  ///
  /// - [`Error::DIDUpdateError`] when retrieving the `RentStructure` fails.
  /// - [`Error::AliasOutputBuildError`] when building the Alias Output fails.
  async fn new_did_output(
    &self,
    address: Address,
    document: IotaDocument,
    rent_structure: Option<RentStructure>,
  ) -> Result<AliasOutput> {
    let rent_structure: RentStructure = if let Some(rent) = rent_structure {
      rent
    } else {
      self.get_rent_structure().await?
    };

    AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
      .with_state_index(0)
      .with_foundry_counter(0)
      .with_state_metadata(document.pack()?)
      .add_feature(Feature::Sender(SenderFeature::new(address)))
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(address),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        address,
      )))
      .finish()
      .map_err(Error::AliasOutputBuildError)
  }

  /// Fetches the associated Alias Output and updates it with `document` in its state metadata.
  /// The storage deposit on the output is left unchanged. If the size of the document increased,
  /// the amount should be increased manually.
  ///
  /// NOTE: This does *not* publish the updated Alias Output.
  ///
  /// # Errors
  ///
  /// Returns `Err` when failing to resolve the DID contained in `document`.
  async fn update_did_output(&self, document: IotaDocument) -> Result<AliasOutput> {
    let id: AliasId = AliasId::from(document.id());
    let (_, alias_output) = self.get_alias_output(id).await?;

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(document.pack()?);

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(id);
    }

    alias_output_builder.finish().map_err(Error::AliasOutputBuildError)
  }

  /// Removes the DID document from the state metadata of its Alias Output,
  /// effectively deactivating it. The storage deposit on the output is left unchanged,
  /// and should be reallocated manually.
  ///
  /// Deactivating does not destroy the output. Hence, it can be re-activated by publishing
  /// an update containing a DID document.
  ///
  /// NOTE: this does *not* publish the updated Alias Output.
  ///
  /// # Errors
  ///
  /// Returns `Err` when failing to resolve the `did`.
  async fn deactivate_did_output(&self, did: &IotaDID) -> Result<AliasOutput> {
    let alias_id: AliasId = AliasId::from(did);
    let (_, alias_output) = self.get_alias_output(alias_id).await?;

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(Vec::new());

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    alias_output_builder.finish().map_err(Error::AliasOutputBuildError)
  }

  /// Resolve a [`IotaDocument`]. Returns an empty, deactivated document if the state metadata
  /// of the Alias Output is empty.
  ///
  /// # Errors
  ///
  /// - [`NetworkMismatch`](Error::NetworkMismatch) if the network of the DID and client differ.
  /// - [`NotFound`](iota_sdk::client::Error::NoOutput) if the associated Alias Output was not found.
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument> {
    validate_network(self, did).await?;

    let id: AliasId = AliasId::from(did);
    let (_, alias_output) = self.get_alias_output(id).await?;
    IotaDocument::unpack_from_output(did, &alias_output, true)
  }

  /// Fetches the [`AliasOutput`] associated with the given DID.
  ///
  /// # Errors
  ///
  /// - [`NetworkMismatch`](Error::NetworkMismatch) if the network of the DID and client differ.
  /// - [`NotFound`](iota_sdk::client::Error::NoOutput) if the associated Alias Output was not found.
  async fn resolve_did_output(&self, did: &IotaDID) -> Result<AliasOutput> {
    validate_network(self, did).await?;

    let id: AliasId = AliasId::from(did);
    self.get_alias_output(id).await.map(|(_, alias_output)| alias_output)
  }

  /// Returns the network name of the client, which is the
  /// Bech32 human-readable part (HRP) of the network.
  ///
  /// E.g. "iota", "atoi", "smr", "rms".
  async fn network_name(&self) -> Result<NetworkName> {
    self.get_network_hrp().await.and_then(NetworkName::try_from)
  }

  /// Return the rent structure of the network, indicating the byte costs for outputs.
  async fn get_rent_structure(&self) -> Result<RentStructure> {
    self
      .get_protocol_parameters()
      .await
      .map(|parameters| *parameters.rent_structure())
  }

  /// Gets the token supply of the node we're connecting to.
  async fn get_token_supply(&self) -> Result<u64> {
    self
      .get_protocol_parameters()
      .await
      .map(|parameters| parameters.token_supply())
  }

  /// Return the Bech32 human-readable part (HRP) of the network.
  ///
  /// E.g. "iota", "atoi", "smr", "rms".
  async fn get_network_hrp(&self) -> Result<String> {
    self
      .get_protocol_parameters()
      .await
      .map(|parameters| parameters.bech32_hrp().to_string())
  }
}

impl<T> IotaIdentityClientExt for T where T: IotaIdentityClient {}

pub(super) async fn validate_network<T>(client: &T, did: &IotaDID) -> Result<()>
where
  T: IotaIdentityClient + ?Sized,
{
  let network_hrp: String = client
    .get_protocol_parameters()
    .await
    .map(|parameters| parameters.bech32_hrp().to_string())?;
  if did.network_str() != network_hrp.as_str() {
    return Err(Error::NetworkMismatch {
      expected: did.network_str().to_owned(),
      actual: network_hrp,
    });
  };
  Ok(())
}
