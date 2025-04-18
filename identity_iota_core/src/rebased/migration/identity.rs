// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use identity_iota_interaction::IdentityMoveCalls;

use crate::rebased::iota::types::Number;
use crate::rebased::proposals::Upgrade;
use crate::IotaDID;
use crate::IotaDocument;
use crate::NetworkName;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;
use async_trait::async_trait;
use identity_core::common::Timestamp;
use identity_iota_interaction::ident_str;
use identity_iota_interaction::move_types::language_storage::StructTag;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::rpc_types::IotaParsedData;
use identity_iota_interaction::rpc_types::IotaParsedMoveObject;
use identity_iota_interaction::rpc_types::IotaPastObjectResponse;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::id::UID;
use identity_iota_interaction::types::object::Owner;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::OptionalSync;
use secret_storage::Signer;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::proposals::BorrowAction;
use crate::rebased::proposals::ConfigChange;
use crate::rebased::proposals::ControllerExecution;
use crate::rebased::proposals::ProposalBuilder;
use crate::rebased::proposals::SendAction;
use crate::rebased::proposals::UpdateDidDocument;
use crate::rebased::rebased_err;
use crate::rebased::transaction::TransactionInternal;
use crate::rebased::transaction::TransactionOutputInternal;
use crate::rebased::Error;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::MoveType;

use super::Multicontroller;
use super::UnmigratedAlias;

const MODULE: &str = "identity";
const NAME: &str = "Identity";
const HISTORY_DEFAULT_PAGE_SIZE: usize = 10;

/// The data stored in an on-chain identity.
pub(crate) struct IdentityData {
  pub(crate) id: UID,
  pub(crate) multicontroller: Multicontroller<Option<Vec<u8>>>,
  pub(crate) legacy_id: Option<ObjectID>,
  pub(crate) created: Timestamp,
  pub(crate) updated: Timestamp,
  pub(crate) version: u64,
  pub(crate) deleted: bool,
  pub(crate) deleted_did: bool,
}

/// An on-chain object holding a DID Document.
#[derive(Clone)]
pub enum Identity {
  /// A legacy IOTA Stardust's Identity.
  Legacy(UnmigratedAlias),
  /// An on-chain Identity.
  FullFledged(OnChainIdentity),
}

impl Identity {
  /// Returns the [`IotaDocument`] DID Document stored inside this [`Identity`].
  pub fn did_document(&self, network: &NetworkName) -> Result<IotaDocument, Error> {
    match self {
      Self::FullFledged(onchain_identity) => Ok(onchain_identity.did_doc.clone()),
      Self::Legacy(alias) => {
        let state_metadata = alias.state_metadata.as_deref().ok_or_else(|| {
          Error::DidDocParsingFailed("legacy stardust alias doesn't contain a DID Document".to_string())
        })?;
        let did = IotaDID::from_object_id(&alias.id.object_id().to_string(), network);
        StateMetadataDocument::unpack(state_metadata)
          .and_then(|state_metadata_doc| state_metadata_doc.into_iota_document(&did))
          .map_err(|e| Error::DidDocParsingFailed(e.to_string()))
      }
    }
  }
}

/// An on-chain entity that wraps an optional DID Document.
#[derive(Debug, Clone, Serialize)]
pub struct OnChainIdentity {
  id: UID,
  multi_controller: Multicontroller<Option<Vec<u8>>>,
  pub(crate) did_doc: IotaDocument,
  version: u64,
  deleted: bool,
  deleted_did: bool,
}

impl OnChainIdentity {
  /// Returns the [`ObjectID`] of this [`OnChainIdentity`].
  pub fn id(&self) -> ObjectID {
    *self.id.object_id()
  }

  /// Returns the [`IotaDocument`] contained in this [`OnChainIdentity`].
  pub fn did_document(&self) -> &IotaDocument {
    &self.did_doc
  }

  pub(crate) fn did_document_mut(&mut self) -> &mut IotaDocument {
    &mut self.did_doc
  }

  /// Returns whether the [IotaDocument] contained in this [OnChainIdentity] has been deleted.
  /// Once a DID Document is deleted, it cannot be reactivated.
  ///
  /// When calling [OnChainIdentity::did_document] on an Identity whose DID Document
  /// had been deleted, an *empty* and *deactivated* [IotaDocument] will be returned.
  pub fn has_deleted_did(&self) -> bool {
    self.deleted_did
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

  pub(crate) fn multicontroller(&self) -> &Multicontroller<Option<Vec<u8>>> {
    &self.multi_controller
  }

  pub(crate) async fn get_controller_cap<S>(&self, client: &IdentityClient<S>) -> Result<ObjectRef, Error> {
    let controller_cap_tag = StructTag::from_str(&format!("{}::controller::ControllerCap", client.package_id()))
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    client
      .find_owned_ref(controller_cap_tag, |obj_data| {
        self.multi_controller.has_member(obj_data.object_id)
      })
      .await?
      .ok_or_else(|| Error::Identity("this address has no control over the requested identity".to_string()))
  }

  /// Updates this [`OnChainIdentity`]'s DID Document.
  pub fn update_did_document(&mut self, updated_doc: IotaDocument) -> ProposalBuilder<'_, UpdateDidDocument> {
    ProposalBuilder::new(self, UpdateDidDocument::new(updated_doc))
  }

  /// Updates this [`OnChainIdentity`]'s configuration.
  pub fn update_config(&mut self) -> ProposalBuilder<'_, ConfigChange> {
    ProposalBuilder::new(self, ConfigChange::default())
  }

  /// Deactivates the DID Document represented by this [`OnChainIdentity`].
  pub fn deactivate_did(&mut self) -> ProposalBuilder<'_, UpdateDidDocument> {
    ProposalBuilder::new(self, UpdateDidDocument::deactivate())
  }

  /// Deletes the DID Document contained in this [OnChainIdentity].
  pub fn delete_did(&mut self) -> ProposalBuilder<'_, UpdateDidDocument> {
    ProposalBuilder::new(self, UpdateDidDocument::delete())
  }

  /// Upgrades this [`OnChainIdentity`]'s version to match the package's.
  pub fn upgrade_version(&mut self) -> ProposalBuilder<'_, Upgrade> {
    ProposalBuilder::new(self, Upgrade)
  }

  /// Sends assets owned by this [`OnChainIdentity`] to other addresses.
  pub fn send_assets(&mut self) -> ProposalBuilder<'_, SendAction> {
    ProposalBuilder::new(self, SendAction::default())
  }

  /// Borrows assets owned by this [`OnChainIdentity`] to use them in a custom transaction.
  pub fn borrow_assets(&mut self) -> ProposalBuilder<'_, BorrowAction> {
    ProposalBuilder::new(self, BorrowAction::default())
  }

  /// Borrows a `ControllerCap` with ID `controller_cap` owned by this identity in a transaction.
  /// This proposal is used to perform operation on a sub-identity controlled
  /// by this one.
  pub fn controller_execution(&mut self, controller_cap: ObjectID) -> ProposalBuilder<'_, ControllerExecution> {
    let action = ControllerExecution::new(controller_cap, self);
    ProposalBuilder::new(self, action)
  }

  /// Returns historical data for this [`OnChainIdentity`].
  pub async fn get_history(
    &self,
    client: &IdentityClientReadOnly,
    last_version: Option<&IotaObjectData>,
    page_size: Option<usize>,
  ) -> Result<Vec<IotaObjectData>, Error> {
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

/// Returns the previous version of the given `history_item`.
pub fn has_previous_version(history_item: &IotaObjectData) -> Result<bool, Error> {
  if let Some(Owner::Shared { initial_shared_version }) = history_item.owner {
    Ok(history_item.version != initial_shared_version)
  } else {
    Err(Error::InvalidIdentityHistory(format!(
      "provided history item does not seem to be a valid identity; {history_item}"
    )))
  }
}

async fn get_previous_version(
  client: &IdentityClientReadOnly,
  iod: IotaObjectData,
) -> Result<Option<IotaObjectData>, Error> {
  client.get_previous_version(iod).await.map_err(rebased_err)
}

/// Returns the [`OnChainIdentity`] having ID `object_id`, if it exists.
pub async fn get_identity(
  client: &IdentityClientReadOnly,
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
    // call was successful but no data for alias id
    return Ok(None);
  };

  let network = client.network();
  let did = IotaDID::from_object_id(&object_id.to_string(), network);
  let Some(IdentityData {
    id,
    multicontroller,
    legacy_id,
    created,
    updated,
    version,
    deleted,
    deleted_did,
  }) = unpack_identity_data(&did, &data)?
  else {
    return Ok(None);
  };
  let legacy_did = legacy_id.map(|legacy_id| IotaDID::from_object_id(&legacy_id.to_string(), client.network()));

  let did_doc = multicontroller
    .controlled_value()
    .as_deref()
    .map(|did_doc_bytes| IotaDocument::from_iota_document_data(did_doc_bytes, true, &did, legacy_did, created, updated))
    .transpose()
    .map_err(|e| Error::DidDocParsingFailed(e.to_string()))?
    .unwrap_or_else(|| {
      let mut empty_did_doc = IotaDocument::new(network);
      empty_did_doc.metadata.deactivated = Some(true);

      empty_did_doc
    });

  Ok(Some(OnChainIdentity {
    id,
    multi_controller: multicontroller,
    did_doc,
    version,
    deleted,
    deleted_did,
  }))
}

fn is_identity(value: &IotaParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}

/// Unpack identity data from given `IotaObjectData`
///
/// # Errors:
/// * in case given data for DID is not an object
/// * parsing identity data from object fails
pub(crate) fn unpack_identity_data(did: &IotaDID, data: &IotaObjectData) -> Result<Option<IdentityData>, Error> {
  let content = data
    .clone()
    .content
    .ok_or_else(|| Error::ObjectLookup(format!("no content in retrieved object in object id {did}")))?;
  let IotaParsedData::MoveObject(value) = content else {
    return Err(Error::ObjectLookup(format!(
      "given data for DID {did} is not an object"
    )));
  };
  if !is_identity(&value) {
    return Ok(None);
  }

  #[derive(Deserialize)]
  struct TempOnChainIdentity {
    id: UID,
    did_doc: Multicontroller<Option<Vec<u8>>>,
    legacy_id: Option<ObjectID>,
    created: Number<u64>,
    updated: Number<u64>,
    version: Number<u64>,
    deleted: bool,
    deleted_did: bool,
  }

  let TempOnChainIdentity {
    id,
    did_doc: multicontroller,
    legacy_id,
    created,
    updated,
    version,
    deleted,
    deleted_did,
  } = serde_json::from_value::<TempOnChainIdentity>(value.fields.to_json_value())
    .map_err(|err| Error::ObjectLookup(format!("could not parse identity document with DID {did}; {err}")))?;

  // Parse DID document timestamps
  let created = {
    let timestamp_ms: u64 = created.try_into().expect("Move string-encoded u64 are valid u64");
    // `Timestamp` requires a timestamp expressed in seconds.
    Timestamp::from_unix(timestamp_ms as i64 / 1000).expect("On-chain clock produces valid timestamps")
  };
  let updated = {
    let timestamp_ms: u64 = updated.try_into().expect("Move string-encoded u64 are valid u64");
    // `Timestamp` requires a timestamp expressed in seconds.
    Timestamp::from_unix(timestamp_ms as i64 / 1000).expect("On-chain clock produces valid timestamps")
  };
  let version = version.try_into().expect("Move string-encoded u64 are valid u64");

  Ok(Some(IdentityData {
    id,
    multicontroller,
    legacy_id,
    created,
    updated,
    version,
    deleted,
    deleted_did,
  }))
}

/// Builder-style struct to create a new [`OnChainIdentity`].
#[derive(Debug)]
pub struct IdentityBuilder {
  did_doc: IotaDocument,
  threshold: Option<u64>,
  controllers: HashMap<IotaAddress, u64>,
}

impl IdentityBuilder {
  /// Initializes a new builder for an [`OnChainIdentity`], where the passed `did_doc` will be
  /// used as the identity's DID Document.
  /// ## Warning
  /// Validation of `did_doc` is deferred to [`CreateIdentityTx`].
  pub fn new(did_doc: IotaDocument) -> Self {
    Self {
      did_doc,
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

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TransactionInternal for CreateIdentityTx {
  type Output = OnChainIdentity;
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let IdentityBuilder {
      did_doc,
      threshold,
      controllers,
    } = self.0;
    let did_doc = StateMetadataDocument::from(did_doc)
      .pack(StateMetadataEncoding::default())
      .map_err(|e| Error::DidDocSerialization(e.to_string()))?;
    let programmable_transaction = if controllers.is_empty() {
      IdentityMoveCallsAdapter::new_identity(Some(&did_doc), client.package_id()).await?
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
      IdentityMoveCallsAdapter::new_with_controllers(Some(&did_doc), controllers, threshold, client.package_id())?
    };

    let response = client.execute_transaction(programmable_transaction, gas_budget).await?;

    let created = response.effects_created().ok_or_else(|| {
      Error::TransactionUnexpectedResponse("could not find effects_created in transaction".to_string())
    })?;
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
          "could not find new identity in response: {}",
          response.to_string()
        )));
      }
    };

    get_identity(client, new_identity_id)
      .await
      .and_then(|identity| identity.ok_or_else(|| Error::ObjectLookup(new_identity_id.to_string())))
      .map(move |identity| TransactionOutputInternal {
        output: identity,
        response,
      })
  }
}
