// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::str::FromStr;

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use identity_iota_core::StateMetadataDocument;
use identity_storage::JwkStorage;
use identity_storage::KeyIdStorage;
use iota_sdk::rpc_types::IotaObjectDataFilter;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaObjectResponseQuery;
use iota_sdk::rpc_types::IotaParsedData;
use iota_sdk::rpc_types::IotaParsedMoveObject;
use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::id::UID;
use iota_sdk::types::object::Owner;
use iota_sdk::IotaClient;
use move_core_types::language_storage::StructTag;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::client::IdentityClient;
use crate::sui::move_calls;
use crate::sui::move_calls::identity::execute_update;
use crate::sui::move_calls::identity::propose_update;
use crate::Error;

use super::Multicontroller;
use super::Proposal;
use super::UnmigratedAlias;

const MODULE: &str = "identity";
const NAME: &str = "Identity";

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
  pub obj_ref: Option<ObjectRef>,
}

impl OnChainIdentity {
  /// Returns true if this [`OnChainIdentity`] is shared between multiple controllers.
  pub fn is_shared(&self) -> bool {
    self.did_doc.controllers().len() > 1
  }
  pub fn proposals(&self) -> &HashMap<String, Proposal> {
    self.did_doc.proposals()
  }
  async fn get_controller_cap<K, I>(&self, client: &IdentityClient<K, I>) -> Result<ObjectRef, Error>
  where
    K: JwkStorage,
    I: KeyIdStorage,
  {
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
}

#[derive(Debug)]
pub enum ProposalAction {
  UpdateDocument(IotaDocument),
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

  pub async fn finish<K, I>(self, client: &IdentityClient<K, I>) -> Result<Option<&'i Proposal>, Error>
  where
    K: JwkStorage,
    I: KeyIdStorage,
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
        identity.obj_ref.unwrap(),
        controller_cap,
        key.as_str(),
        did_doc.pack().unwrap(),
        expiration,
        client.package_id(),
      ),
    }
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let gas_budget =
      gas_budget.ok_or_else(|| Error::GasIssue("missing `gas_budget` in proposal builder".to_string()))?;
    let _ = client.execute_transaction(tx, gas_budget).await?;

    *identity = get_identity(client, *identity.id.object_id()).await?.unwrap();
    let can_execute =
      identity.did_doc.controller_voting_power(controller_cap.0).unwrap() > identity.did_doc.threshold();
    if identity.is_shared() && !can_execute {
      return Ok(identity.proposals().get(&key));
    }

    let tx = execute_update(
      identity.obj_ref.unwrap(),
      controller_cap,
      key.as_str(),
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let _ = client.execute_transaction(tx, gas_budget).await?;
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
  identity.obj_ref = Some(object_ref);

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

  pub async fn finish<K, I>(self, client: &IdentityClient<K, I>) -> Result<OnChainIdentity, Error>
  where
    K: JwkStorage,
    I: KeyIdStorage,
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
    let response = client.execute_transaction(programmable_transaction, gas_budget).await?;

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
