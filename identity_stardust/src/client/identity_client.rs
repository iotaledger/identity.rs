// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::DIDError;

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
use crate::NetworkName;
use crate::Result;
use crate::StardustDID;
use crate::StardustDocument;

impl TryFrom<&StardustDID> for AliasId {
  type Error = Error;

  fn try_from(did: &StardustDID) -> std::result::Result<Self, Self::Error> {
    let tag_bytes: [u8; StardustDID::TAG_BYTES_LEN] =
      prefix_hex::decode(did.tag()).map_err(|_| DIDError::InvalidMethodId)?;
    Ok(AliasId::new(tag_bytes))
  }
}

/// Helper functions necessary for the [`StardustIdentityClientExt`] trait.
#[async_trait::async_trait(? Send)]
pub trait StardustIdentityClient {
  /// Return the Bech32 human-readable part (HRP) of the network.
  ///
  /// E.g. "iota", "atoi", "smr", "rms".
  async fn get_network_hrp(&self) -> Result<String>;

  /// Resolve an Alias identifier, returning its latest [`OutputId`] and [`AliasOutput`].
  async fn get_alias_output(&self, alias_id: AliasId) -> Result<(OutputId, AliasOutput)>;

  /// Return the rent structure of the network, indicating the byte costs for outputs.
  async fn get_rent_structure(&self) -> Result<RentStructure>;
}

/// An extension trait that provides helper functions for publication
/// and resolution of DID documents in Alias Outputs.
///
/// This trait is not intended to be implemented directly, a blanket implementation is
/// provided for [`StardustIdentityClient`] implementers.
#[async_trait::async_trait(? Send)]
pub trait StardustIdentityClientExt: StardustIdentityClient {
  /// Create a DID with a new Alias Output containing the given `document`.
  ///
  /// The `address` will be set as the state controller and governor unlock conditions.
  /// The minimum required token deposit amount will be set according to the given
  /// `rent_structure`, which will be fetched from the node if not provided.
  /// The returned Alias Output can be further customised before publication, if desired.
  ///
  /// NOTE: this does *not* publish the Alias Output.
  ///
  /// # Errors
  ///
  /// - [`Error::DIDUpdateError`] when retrieving the `RentStructure` fails.
  /// - [`Error::AliasOutputBuildError`] when building the Alias Output fails.
  async fn new_did_output(
    &self,
    address: Address,
    document: StardustDocument,
    rent_structure: Option<RentStructure>,
  ) -> Result<AliasOutput> {
    let rent_structure: RentStructure = if let Some(rent) = rent_structure {
      rent
    } else {
      self.get_rent_structure().await?
    };

    AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
      .map_err(Error::AliasOutputBuildError)?
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
  /// NOTE: this does *not* publish the updated Alias Output.
  ///
  /// # Errors
  ///
  /// Returns `Err` when failing to resolve the DID contained in `document`.
  async fn update_did_output(&self, document: StardustDocument) -> Result<AliasOutput> {
    let id: AliasId = AliasId::try_from(document.id())?;
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
  async fn deactivate_did_output(&self, did: &StardustDID) -> Result<AliasOutput> {
    let alias_id: AliasId = AliasId::try_from(did)?;
    let (_, alias_output) = self.get_alias_output(alias_id).await?;

    let mut alias_output_builder: AliasOutputBuilder = AliasOutputBuilder::from(&alias_output)
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(Vec::new());

    if alias_output.alias_id().is_null() {
      alias_output_builder = alias_output_builder.with_alias_id(alias_id);
    }

    alias_output_builder.finish().map_err(Error::AliasOutputBuildError)
  }

  /// Resolve a [`StardustDocument`]. Returns an empty, deactivated document if the state metadata
  /// of the Alias Output is empty.
  ///
  /// # Errors
  ///
  /// - [`NetworkMismatch`](Error::NetworkMismatch) if the network of the DID and client differ.
  /// - [`NotFound`](iota_client::Error::NotFound) if the associated Alias Output was not found.
  async fn resolve_did(&self, did: &StardustDID) -> Result<StardustDocument> {
    validate_network(self, did).await?;

    let id: AliasId = AliasId::try_from(did)?;
    let (_, alias_output) = self.get_alias_output(id).await?;

    let document: &[u8] = alias_output.state_metadata();
    StardustDocument::unpack(did, document, true)
  }

  /// Fetches the [`AliasOutput`] associated with the given DID.
  ///
  /// # Errors
  ///
  /// - [`NetworkMismatch`](Error::NetworkMismatch) if the network of the DID and client differ.
  /// - [`NotFound`](iota_client::Error::NotFound) if the associated Alias Output was not found.
  async fn resolve_did_output(&self, did: &StardustDID) -> Result<AliasOutput> {
    validate_network(self, did).await?;

    let id: AliasId = AliasId::try_from(did)?;
    self.get_alias_output(id).await.map(|(_, alias_output)| alias_output)
  }

  /// Returns the network name of the client, which is the
  /// Bech32 human-readable part (HRP) of the network.
  ///
  /// E.g. "iota", "atoi", "smr", "rms".
  async fn network_name(&self) -> Result<NetworkName> {
    self.get_network_hrp().await.and_then(NetworkName::try_from)
  }
}

impl<T> StardustIdentityClientExt for T where T: StardustIdentityClient {}

pub(super) async fn validate_network<T>(client: &T, did: &StardustDID) -> Result<()>
where
  T: StardustIdentityClient + ?Sized,
{
  let network_hrp: String = client.get_network_hrp().await?;
  if did.network_str() != network_hrp.as_str() {
    return Err(Error::NetworkMismatch {
      expected: did.network_str().to_owned(),
      actual: network_hrp,
    });
  };
  Ok(())
}
