// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::StateMetadataDocument;
use crate::sui::iota_sdk_adapter::IotaClientTraitCore;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectData;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectDataOptions;
use crate::iota_sdk_abstraction::rpc_types::IotaParsedData;
use crate::iota_sdk_abstraction::rpc_types::IotaParsedMoveObject;
use crate::iota_sdk_abstraction::rpc_types::IotaPastObjectResponse;
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use crate::iota_sdk_abstraction::types::base_types::IotaAddress;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::base_types::ObjectRef;
use crate::iota_sdk_abstraction::types::id::UID;
use crate::iota_sdk_abstraction::types::object::Owner;
use crate::iota_sdk_abstraction::types::TypeTag;
use crate::ident_str;
use crate::iota_sdk_abstraction::move_types::language_storage::StructTag;
use secret_storage::Signer;
use serde;
use serde::Deserialize;

use crate::client::IdentityClient;
use crate::client::IdentityClientReadOnly;
use crate::iota_sdk_abstraction::IotaKeySignature;
use crate::proposals::ConfigChange;
use crate::proposals::DeactiveDid;
use crate::proposals::ProposalBuilder;
use crate::proposals::UpdateDidDocument;
use crate::sui::iota_sdk_adapter::IdentityMoveCallsAdapter;
use crate::iota_sdk_abstraction::IdentityMoveCalls;
use crate::transaction::Transaction;
use crate::utils::MoveType;
use crate::Error;

use super::Multicontroller;
use super::UnmigratedAlias;

const MODULE: &str = "identity";
const NAME: &str = "Identity";
const HISTORY_DEFAULT_PAGE_SIZE: usize = 10;

/// An on-chain object holding a DID Document.
pub enum Identity {
  /// A legacy IOTA Stardust's Identity.
  Legacy(UnmigratedAlias),
  /// An on-chain Identity.
  FullFledged(OnChainIdentity),
}

impl Identity {
  /// Returns the [`IotaDocument`] DID Document stored inside this [`Identity`].
  pub fn did_document<C>(&self, client: &IdentityClientReadOnly<C>) -> Result<IotaDocument, Error> {
    let original_did = IotaDID::from_alias_id(self.id().to_string().as_str(), client.network());
    let doc_bytes = self.doc_bytes().ok_or(Error::DidDocParsingFailed(
      "legacy alias output does not encode a DID document".to_owned(),
    ))?;

    StateMetadataDocument::unpack(doc_bytes)
      .and_then(|state_metadata_doc| state_metadata_doc.into_iota_document(&original_did))
      .map_err(|e| Error::DidDocParsingFailed(e.to_string()))
  }

  fn id(&self) -> ObjectID {
    match self {
      Self::Legacy(alias) => *alias.id.object_id(),
      Self::FullFledged(identity) => identity.id(),
    }
  }

  fn doc_bytes(&self) -> Option<&[u8]> {
    match self {
      Self::FullFledged(identity) => Some(identity.multi_controller.controlled_value().as_ref()),
      Self::Legacy(alias) => alias.state_metadata.as_deref(),
    }
  }
}

/// Representation of `identity.rs`'s `Identity` Move type.
#[derive(Debug)]
pub struct OnChainIdentity {
  id: UID,
  multi_controller: Multicontroller<Vec<u8>>,
  did_doc: IotaDocument,
}

impl Deref for OnChainIdentity {
  type Target = IotaDocument;
  fn deref(&self) -> &Self::Target {
    &self.did_doc
  }
}

impl OnChainIdentity {
  /// Returns the [`ObjectID`] of this [`OnChainIdentity`].
  pub fn id(&self) -> ObjectID {
    *self.id.object_id()
  }

  /// Returns true if this [`OnChainIdentity`] is shared between multiple controllers.
  pub fn is_shared(&self) -> bool {
    self.multi_controller.controllers().len() > 1
  }

  /// Returns this [`OnChainIdentity`]'s list of active proposals.
  pub fn proposals(&self) -> &HashSet<ObjectID> {
    self.multi_controller.proposals()
  }

  /// Returns this [`OnChainIdentity`]'s controllers as the map: `controller_id -> controller_voting_power`.
  pub fn controllers(&self) -> &HashMap<ObjectID, u64> {
    self.multi_controller.controllers()
  }

  /// Returns the threshold required by this [`OnChainIdentity`] for executing a proposal.
  pub fn threshold(&self) -> u64 {
    self.multi_controller.threshold()
  }

  /// Returns the voting power of controller with ID `controller_id`, if any.
  pub fn controller_voting_power(&self, controller_id: ObjectID) -> Option<u64> {
    self.multi_controller.controller_voting_power(controller_id)
  }

  pub(crate) fn multicontroller(&self) -> &Multicontroller<Vec<u8>> {
    &self.multi_controller
  }

  pub(crate) async fn get_controller_cap<S, C>(&self, client: &IdentityClient<S, C>) -> Result<ObjectRef, Error>
  where
    C: IotaClientTraitCore,
  {
    let controller_cap_tag = StructTag::from_str(&format!("{}::multicontroller::ControllerCap", client.package_id()))
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    client
      .find_owned_ref(controller_cap_tag, |obj_data| {
        self.multi_controller.has_member(obj_data.object_id)
      })
      .await?
      .ok_or_else(|| Error::Identity("this address has no control over the requested identity".to_string()))
  }

  /// Updates this [`OnChainIdentity`]'s DID Document.
  pub fn update_did_document(self, updated_doc: IotaDocument) -> ProposalBuilder<UpdateDidDocument> {
    ProposalBuilder::new(self, UpdateDidDocument::new(updated_doc))
  }

  /// Updates this [`OnChainIdentity`]'s configuration.
  pub fn update_config(self) -> ProposalBuilder<ConfigChange> {
    ProposalBuilder::new(self, ConfigChange::default())
  }

  /// Deactivates the DID Document represented by this [`OnChainIdentity`].
  pub fn deactivate_did(self) -> ProposalBuilder<DeactiveDid> {
    ProposalBuilder::new(self, DeactiveDid::new())
  }

  /// Returns historical data for this [`OnChainIdentity`].
  pub async fn get_history<C>(
    &self,
    client: &IdentityClientReadOnly<C>,
    last_version: Option<&IotaObjectData>,
    page_size: Option<usize>,
  ) -> Result<Vec<IotaObjectData>, Error>
  where
      C: IotaClientTraitCore + Sync,
  {
    let identity_ref = client
      .get_object_ref_by_id(self.id())
      .await?
      .ok_or_else(|| Error::InvalidIdentityHistory("no reference to identity loaded".to_string()))?;
    let object_id = identity_ref.object_id();

    let mut history: Vec<IotaObjectData> = vec![];
    let mut current_version = if let Some(last_version_value) = last_version {
      // starting version given, this will be skipped in paging
      last_version_value.clone()
    } else {
      // no version given, this version will be included in history
      let version = identity_ref.version();
      let response = client.get_past_object(object_id, version).await?;
      let latest_version = if let IotaPastObjectResponse::VersionFound(response_value) = response {
        response_value
      } else {
        return Err(Error::InvalidIdentityHistory(format!(
          "could not find current version {version} of object {object_id}, response {response:?}"
        )));
      };
      history.push(latest_version.clone()); // include current version in history if we start from now
      latest_version
    };

    // limit lookup count to prevent locking on large histories
    let page_size = page_size.unwrap_or(HISTORY_DEFAULT_PAGE_SIZE);
    while history.len() < page_size {
      let lookup = get_previous_version(client, current_version).await?;
      if let Some(value) = lookup {
        current_version = value;
        history.push(current_version.clone());
      } else {
        break;
      }
    }

    Ok(history)
  }
}

pub fn has_previous_version(history_item: &IotaObjectData) -> Result<bool, Error> {
  if let Some(Owner::Shared { initial_shared_version }) = history_item.owner {
    Ok(history_item.version != initial_shared_version)
  } else {
    Err(Error::InvalidIdentityHistory(format!(
      "provided history item does not seem to be a valid identity; {history_item}"
    )))
  }
}

async fn get_previous_version<C: IotaClientTraitCore,>(
  client: &IdentityClientReadOnly<C>,
  iod: IotaObjectData,
) -> Result<Option<IotaObjectData>, Error> {
  client.get_previous_version(iod).await
}

/// Returns the [`OnChainIdentity`] having ID `object_id`, if it exists.
pub async fn get_identity<C: IotaClientTraitCore>(
  client: &IdentityClientReadOnly<C>,
  object_id: ObjectID,
) -> Result<Option<OnChainIdentity>, Error> {
  let response = client
    .read_api()
    .get_object_with_options(object_id, IotaObjectDataOptions::new().with_content())
    .await
    .map_err(|err| {
      Error::ObjectLookup(format!(
        "Could not get object with options for this object_id {object_id}; {err}"
      ))
    })?;

  // no issues with call but
  let Some(data) = response.data else {
    // call was successful but not data for alias id
    return Ok(None);
  };

  let content = data
    .content
    .ok_or_else(|| Error::ObjectLookup(format!("no content in retrieved object in object id {object_id}")))?;

  let IotaParsedData::MoveObject(value) = content else {
    return Err(Error::ObjectLookup(format!(
      "found data at object id {object_id} is not an object"
    )));
  };

  if !is_identity(&value) {
    return Ok(None);
  }

  #[derive(Deserialize)]
  struct TempOnChainIdentity {
    id: UID,
    did_doc: Multicontroller<Vec<u8>>,
  }

  let TempOnChainIdentity {
    id,
    did_doc: multi_controller,
  } = serde_json::from_value::<TempOnChainIdentity>(value.fields.to_json_value()).map_err(|err| {
    Error::ObjectLookup(format!(
      "could not parse identity document with object id {object_id}; {err}"
    ))
  })?;
  let original_did = IotaDID::from_alias_id(id.object_id().to_string().as_str(), client.network());
  let did_doc = StateMetadataDocument::unpack(multi_controller.controlled_value())
    .and_then(|state_metadata_doc| state_metadata_doc.into_iota_document(&original_did))
    .map_err(|e| Error::DidDocParsingFailed(e.to_string()))?;

  Ok(Some(OnChainIdentity {
    id,
    multi_controller,
    did_doc,
  }))
}

fn is_identity(value: &IotaParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}

/// Builder-style struct to create a new [`OnChainIdentity`].
#[derive(Debug)]
pub struct IdentityBuilder {
  did_doc: Vec<u8>,
  threshold: Option<u64>,
  controllers: HashMap<IotaAddress, u64>,
}

impl IdentityBuilder {
  /// Initializes a new builder for an [`OnChainIdentity`], where the passed `did_doc` will be
  /// used as the identity's DID Document.
  /// ## Warning
  /// Validation of `did_doc` is deferred to [`CreateIdentityTx`].
  pub fn new(did_doc: &[u8]) -> Self {
    Self {
      did_doc: did_doc.to_vec(),
      threshold: None,
      controllers: HashMap::new(),
    }
  }

  /// Gives `address` the capability to act as a controller with voting power `voting_power`.
  pub fn controller(mut self, address: IotaAddress, voting_power: u64) -> Self {
    self.controllers.insert(address, voting_power);
    self
  }

  /// Sets the identity's threshold.
  pub fn threshold(mut self, threshold: u64) -> Self {
    self.threshold = Some(threshold);
    self
  }

  /// Sets multiple controllers in a single step. See [`IdentityBuilder::controller`].
  pub fn controllers<I>(self, controllers: I) -> Self
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    controllers
      .into_iter()
      .fold(self, |builder, (addr, vp)| builder.controller(addr, vp))
  }

  /// Turns this builder into a [`Transaction`], ready to be executed.
  pub fn finish(self) -> CreateIdentityTx {
    CreateIdentityTx(self)
  }
}

impl MoveType for OnChainIdentity {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
      address: package.into(),
      module: ident_str!("identity").into(),
      name: ident_str!("Identity").into(),
      type_params: vec![],
    }))
  }
}

/// A [`Transaction`] for creating a new [`OnChainIdentity`] from an [`IdentityBuilder`].
#[derive(Debug)]
pub struct CreateIdentityTx(IdentityBuilder);

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
impl Transaction for CreateIdentityTx {
  type Output = OnChainIdentity;
  async fn execute_with_opt_gas<S, C>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S, C>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
    C: IotaClientTraitCore + Sync,
  {
    let IdentityBuilder {
      did_doc,
      threshold,
      controllers,
    } = self.0;
    let programmable_transaction = if controllers.is_empty() {
      IdentityMoveCallsAdapter::new_identity(&did_doc, client.package_id())?
    } else {
      let threshold = match threshold {
        Some(t) => t,
        None if controllers.len() == 1 => *controllers
          .values()
          .next()
          .ok_or_else(|| Error::Identity("could not get controller".to_string()))?,
        None => {
          return Err(Error::TransactionBuildingFailed(
            "Missing field `threshold` in identity creation".to_owned(),
          ))
        }
      };
      IdentityMoveCallsAdapter::new_with_controllers(&did_doc, controllers, threshold, client.package_id())?
    };

    let response = client.execute_transaction(programmable_transaction, gas_budget).await?;

    let created = match response.effects_created() {
      Some(effects) => effects,
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find effects in transaction response: {}", response.to_string()
        )));
      }
    };
    let new_identities: Vec<OwnedObjectRef> = created
      .into_iter()
      .filter(|elem| {
        matches!(
          elem.owner,
          Owner::Shared {
            initial_shared_version: _,
          }
        )
      })
      .collect();
    let new_identity_id = match &new_identities[..] {
      [value] => value.object_id(),
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find new identity in response: {}", response.to_string()
        )));
      }
    };

    get_identity(client, new_identity_id)
      .await
      .and_then(|identity| identity.ok_or_else(|| Error::ObjectLookup(new_identity_id.to_string())))
  }
}
