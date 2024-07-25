// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::str::FromStr;

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use identity_iota_core::StateMetadataDocument;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaObjectDataFilter;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaObjectResponseQuery;
use iota_sdk::rpc_types::IotaParsedData;
use iota_sdk::rpc_types::IotaParsedMoveObject;
use iota_sdk::rpc_types::IotaPastObjectResponse;
use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::IotaTransactionBlockResponseOptions;
use iota_sdk::rpc_types::ObjectChange;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::base_types::SequenceNumber;
use iota_sdk::types::id::UID;
use iota_sdk::types::object::Owner;
use iota_sdk::IotaClient;
use move_core_types::language_storage::StructTag;
use secret_storage::signer::Signer;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::client::IdentityClient;
use crate::client::KinesisKeySignature;
use crate::sui::move_calls;
use crate::sui::move_calls::identity::execute_update;
use crate::sui::move_calls::identity::propose_update;
use crate::Error;

use super::Multicontroller;
use super::Proposal;
use super::UnmigratedAlias;

const MODULE: &str = "identity";
const NAME: &str = "Identity";
const HISTORY_DEFAULT_PAGE_SIZE: usize = 10;

pub enum Identity {
  Legacy(UnmigratedAlias),
  FullFledged(OnChainIdentity),
}

impl Identity {
  pub fn did_document(&self, _client: &IotaClient) -> Result<IotaDocument, Error> {
    let network_name = /* TODO: read it from client */ NetworkName::try_from("iota").unwrap();
    let original_did = IotaDID::from_alias_id(self.id().object_id().to_string().as_str(), &network_name);
    let doc_bytes = self.doc_bytes().ok_or(Error::DidDocParsingFailed(
      "legacy alias output does not encode a DID document".to_owned(),
    ))?;

    StateMetadataDocument::unpack(doc_bytes)
      .and_then(|state_metadata_doc| state_metadata_doc.into_iota_document(&original_did))
      .map_err(|e| Error::DidDocParsingFailed(e.to_string()))
  }

  fn id(&self) -> &UID {
    match self {
      Self::Legacy(alias) => &alias.id,
      Self::FullFledged(identity) => &identity.id,
    }
  }

  fn doc_bytes(&self) -> Option<&[u8]> {
    match self {
      Self::FullFledged(identity) => Some(identity.did_doc.controlled_value().as_ref()),
      Self::Legacy(alias) => alias.state_metadata.as_deref(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnChainIdentity {
  pub id: UID,
  pub did_doc: Multicontroller<Vec<u8>>,
  #[serde(skip)]
  pub obj_ref: Option<OwnedObjectRef>,
}

impl OnChainIdentity {
  /// Returns true if this [`OnChainIdentity`] is shared between multiple controllers.
  pub fn is_shared(&self) -> bool {
    self.did_doc.controllers().len() > 1
  }
  pub fn proposals(&self) -> &HashMap<String, Proposal> {
    self.did_doc.proposals()
  }
  async fn get_controller_cap(&self, client: &IdentityClient) -> Result<ObjectRef, Error> {
    let controller_cap_tag = StructTag::from_str(&format!("{}::multicontroller::ControllerCap", client.package_id()))
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(controller_cap_tag));

    let mut cursor = None;
    loop {
      let mut page = client
        .read_api()
        .get_owned_objects(client.sender_address()?, Some(filter.clone()), cursor, None)
        .await?;
      let cap = std::mem::take(&mut page.data).into_iter().find_map(|res| {
        res
          .data
          .map(|obj| obj.object_ref())
          .filter(|cap_ref| self.did_doc.has_member(cap_ref.0))
      });
      cursor = page.next_cursor;
      if let Some(cap) = cap {
        return Ok(cap);
      }
      if !page.has_next_page {
        break;
      }
    }

    Err(Error::Identity(
      "this address has no control over the requested identity".to_string(),
    ))
  }

  pub fn update_did_document(&mut self, updated_doc: IotaDocument) -> ProposalBuilder {
    ProposalBuilder::new(self, ProposalAction::UpdateDocument(updated_doc))
  }

  pub fn deactivate_did(&mut self) -> ProposalBuilder {
    ProposalBuilder::new(self, ProposalAction::Deactivate)
  }

  pub async fn get_history(
    &self,
    client: &IdentityClient,
    last_version: Option<&IotaObjectData>,
    page_size: Option<usize>,
  ) -> Result<Vec<IotaObjectData>, Error> {
    let identity_ref = self
      .obj_ref
      .as_ref()
      .ok_or_else(|| Error::InvalidIdentityHistory("no reference to identity loaded".to_string()))?;
    let object_id = identity_ref.object_id();

    let mut history: Vec<IotaObjectData> = vec![];
    let mut current_version = if let Some(last_version_value) = last_version {
      // starting version given, this will be skipped in paging
      last_version_value.clone()
    } else {
      // no version given, this version will be included in history
      let version = identity_ref.version();
      let response = get_past_object(client, object_id, version).await?;
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

async fn get_past_object(
  client: &IdentityClient,
  object_id: ObjectID,
  version: SequenceNumber,
) -> Result<IotaPastObjectResponse, Error> {
  client
    .iota_client()
    .read_api()
    .try_get_parsed_past_object(object_id, version, IotaObjectDataOptions::full_content())
    .await
    .map_err(|err| {
      Error::InvalidIdentityHistory(format!("could not look up object {object_id} version {version}; {err}"))
    })
}

async fn get_previous_version(client: &IdentityClient, iod: IotaObjectData) -> Result<Option<IotaObjectData>, Error> {
  // try to get digest of previous tx
  // if we requested the prev tx and it isn't returned, this should be the oldest state
  let prev_tx_digest = if let Some(value) = iod.previous_transaction {
    value
  } else {
    return Ok(None);
  };

  // resolve previous tx
  let prev_tx_response = client
    .iota_client()
    .read_api()
    .get_transaction_with_options(
      prev_tx_digest,
      IotaTransactionBlockResponseOptions::new().with_object_changes(),
    )
    .await
    .unwrap();

  // check for updated/created changes
  let (created, other_changes): (Vec<ObjectChange>, _) = prev_tx_response
    .clone()
    .object_changes
    .ok_or_else(|| {
      Error::InvalidIdentityHistory(format!(
        "could not find object changes for object {} in transaction {prev_tx_digest}",
        iod.object_id
      ))
    })?
    .into_iter()
    .filter(|elem| iod.object_id.eq(&elem.object_id()))
    .partition(|elem| matches!(elem, ObjectChange::Created { .. }));

  // previous tx contain create tx, so there is no previous version
  if created.len() == 1 {
    return Ok(None);
  }

  let mut previous_versions: Vec<SequenceNumber> = other_changes
    .iter()
    .filter_map(|elem| match elem {
      ObjectChange::Mutated { previous_version, .. } => Some(*previous_version),
      _ => None,
    })
    .collect();

  previous_versions.sort();

  let earliest_previous = if let Some(value) = previous_versions.first() {
    value
  } else {
    return Ok(None); // no mutations in prev tx, so no more versions can be found
  };

  let past_obj_response = get_past_object(client, iod.object_id, *earliest_previous).await?;
  match past_obj_response {
    IotaPastObjectResponse::VersionFound(value) => Ok(Some(value)),
    _ => Err(Error::InvalidIdentityHistory(format!(
      "could not find previous version, past object response: {past_obj_response:?}"
    ))),
  }
}

#[derive(Debug)]
pub enum ProposalAction {
  UpdateDocument(IotaDocument),
  Deactivate,
}

#[derive(Debug)]
pub struct ProposalBuilder<'i> {
  key: Option<String>,
  identity: &'i mut OnChainIdentity,
  expiration: Option<u64>,
  action: ProposalAction,
  gas_budget: Option<u64>,
}

impl<'i> ProposalBuilder<'i> {
  pub fn new(identity: &'i mut OnChainIdentity, action: ProposalAction) -> Self {
    Self {
      key: None,
      identity,
      expiration: None,
      action,
      gas_budget: None,
    }
  }

  pub fn expiration_epoch(mut self, exp: u64) -> Self {
    self.expiration = Some(exp);
    self
  }

  pub fn key(mut self, key: String) -> Self {
    self.key = Some(key);
    self
  }

  pub fn gas_budget(mut self, amount: u64) -> Self {
    self.gas_budget = Some(amount);
    self
  }

  pub async fn finish<S>(self, client: &IdentityClient, signer: &S) -> Result<Option<&'i Proposal>, Error>
  where
    S: Signer<KinesisKeySignature> + Send + Sync,
  {
    let ProposalBuilder {
      key,
      identity,
      expiration,
      action,
      gas_budget,
    } = self;
    let key = if let Some(key) = key {
      key
    } else if !identity.is_shared() {
      use rand::distributions::DistString;
      let mut rng = rand::thread_rng();

      rand::distributions::Alphanumeric.sample_string(&mut rng, 5)
    } else {
      return Err(Error::TransactionBuildingFailed(
        "no key specified for proposal".to_string(),
      ));
    };
    let controller_cap = identity.get_controller_cap(client).await?;
    let tx = match action {
      ProposalAction::UpdateDocument(did_doc) => propose_update(
        identity.obj_ref.clone().unwrap(),
        controller_cap,
        key.as_str(),
        did_doc.pack().unwrap(),
        expiration,
        client.package_id(),
      ),
      ProposalAction::Deactivate => propose_update(
        identity.obj_ref.clone().unwrap(),
        controller_cap,
        key.as_str(),
        vec![],
        expiration,
        client.package_id(),
      ),
    }
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let gas_budget =
      gas_budget.ok_or_else(|| Error::GasIssue("missing `gas_budget` in proposal builder".to_string()))?;
    let _ = client.execute_transaction(tx, gas_budget, signer).await?;

    // refresh object references to get latest sequence numbers for them
    *identity = get_identity(client, *identity.id.object_id()).await?.unwrap();
    let can_execute =
      identity.did_doc.controller_voting_power(controller_cap.0).unwrap() > identity.did_doc.threshold();
    if identity.is_shared() && !can_execute {
      return Ok(identity.proposals().get(&key));
    }
    let controller_cap = identity.get_controller_cap(client).await?;

    let tx = execute_update(
      identity.obj_ref.clone().unwrap(),
      controller_cap,
      key.as_str(),
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let _ = client.execute_transaction(tx, gas_budget, signer).await?;
    *identity = get_identity(client, *identity.id.object_id()).await?.unwrap();

    Ok(None)
  }
}

pub async fn get_identity(client: &IotaClient, object_id: ObjectID) -> Result<Option<OnChainIdentity>, Error> {
  let options = IotaObjectDataOptions {
    show_type: true,
    show_owner: true,
    show_previous_transaction: true,
    show_display: true,
    show_content: true,
    show_bcs: true,
    show_storage_rebate: true,
  };
  let response = client
    .read_api()
    .get_object_with_options(object_id, options)
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

  let object_ref = data.object_ref();
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

  let mut identity = serde_json::from_value::<OnChainIdentity>(value.fields.to_json_value()).map_err(|err| {
    Error::ObjectLookup(format!(
      "could not parse identity document with object id {object_id}; {err}"
    ))
  })?;
  identity.obj_ref = Some(OwnedObjectRef {
    owner: data.owner.unwrap(),
    reference: object_ref.into(),
  });

  Ok(Some(identity))
}

fn is_identity(value: &IotaParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}

#[derive(Debug)]
pub struct IdentityBuilder<'a> {
  package_id: ObjectID,
  did_doc: &'a [u8],
  threshold: Option<u64>,
  controllers: HashMap<IotaAddress, u64>,
  gas_budget: Option<u64>,
}

impl<'a> IdentityBuilder<'a> {
  pub fn new(did_doc: &'a [u8], package_id: ObjectID) -> Self {
    Self {
      did_doc,
      package_id,
      threshold: None,
      controllers: HashMap::new(),
      gas_budget: None,
    }
  }

  pub fn controller(mut self, address: IotaAddress, voting_power: u64) -> Self {
    self.controllers.insert(address, voting_power);
    self
  }

  pub fn threshold(mut self, threshold: u64) -> Self {
    self.threshold = Some(threshold);
    self
  }

  pub fn gas_budget(mut self, gas_budget: u64) -> Self {
    self.gas_budget = Some(gas_budget);
    self
  }

  pub fn controllers<I>(self, controllers: I) -> Self
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    controllers
      .into_iter()
      .fold(self, |builder, (addr, vp)| builder.controller(addr, vp))
  }

  pub async fn finish<S>(self, client: &IdentityClient, signer: &S) -> Result<OnChainIdentity, Error>
  where
    S: Signer<KinesisKeySignature> + Send + Sync,
  {
    let IdentityBuilder {
      package_id,
      did_doc,
      threshold,
      controllers,
      gas_budget,
    } = self;

    let programmable_transaction = if controllers.is_empty() {
      move_calls::identity::new(did_doc, package_id)?
    } else {
      let threshold = match threshold {
        Some(t) => t,
        None if controllers.len() == 1 => *controllers.values().next().unwrap(),
        None => {
          return Err(Error::TransactionBuildingFailed(
            "Missing field `threshold` in identity creation".to_owned(),
          ))
        }
      };
      move_calls::identity::new_with_controllers(did_doc, controllers, threshold, package_id)?
    };

    let gas_budget = gas_budget.ok_or_else(|| Error::GasIssue("Missing gas budget in identity creation".to_owned()))?;
    let response = client
      .execute_transaction(programmable_transaction, gas_budget, signer)
      .await?;

    let created = match response.clone().effects {
      Some(IotaTransactionBlockEffects::V1(effects)) => effects.created,
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find effects in transaction response: {response:?}"
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
          "could not find new identity in response: {response:?}"
        )));
      }
    };

    get_identity(client, new_identity_id)
      .await
      .and_then(|identity| identity.ok_or_else(|| Error::ObjectLookup(new_identity_id.to_string())))
  }
}
