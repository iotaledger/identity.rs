// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use identity_iota_core::StateMetadataDocument;
use identity_storage::JwkStorage;
use identity_storage::KeyIdStorage;
use serde;
use serde::Deserialize;
use serde::Serialize;
use sui_sdk::rpc_types::OwnedObjectRef;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::rpc_types::SuiParsedData;
use sui_sdk::rpc_types::SuiParsedMoveObject;
use sui_sdk::rpc_types::SuiTransactionBlockEffects;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::id::UID;
use sui_sdk::types::object::Owner;
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::transaction::Transaction;
use sui_sdk::SuiClient;

use crate::client::IdentityClient;
use crate::sui::move_calls;
use crate::Error;

use super::Multicontroller;
use super::UnmigratedAlias;

const MODULE: &str = "identity";
const NAME: &str = "Identity";
pub enum Identity {
  Legacy(UnmigratedAlias),
  FullFledged(OnChainIdentity),
}

impl Identity {
  pub fn did_document(&self, _client: &SuiClient) -> Result<IotaDocument, Error> {
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
}

pub async fn get_identity(client: &SuiClient, object_id: ObjectID) -> Result<Option<OnChainIdentity>, Error> {
  let options = SuiObjectDataOptions {
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

  let content = data
    .content
    .ok_or_else(|| Error::ObjectLookup(format!("no content in retrieved object in object id {object_id}")))?;

  let SuiParsedData::MoveObject(value) = content else {
    return Err(Error::ObjectLookup(format!(
      "found data at object id {object_id} is not an object"
    )));
  };

  if !is_identity(&value) {
    return Ok(None);
  }

  serde_json::from_value(value.fields.to_json_value()).map_err(|err| {
    Error::ObjectLookup(format!(
      "could not parse identity document with object id {object_id}; {err}"
    ))
  })
}

fn is_identity(value: &SuiParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}

#[derive(Debug)]
pub struct IdentityBuilder<'a> {
  package_id: ObjectID,
  did_doc: &'a [u8],
  threshold: Option<u64>,
  controllers: HashMap<SuiAddress, u64>,
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

  pub fn controller(mut self, address: SuiAddress, voting_power: u64) -> Self {
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
    I: IntoIterator<Item = (SuiAddress, u64)>,
  {
    controllers
      .into_iter()
      .fold(self, |builder, (addr, vp)| builder.controller(addr, vp))
  }

  pub async fn finish<K, I>(self, client: &IdentityClient<K, I>) -> Result<ObjectID, Error>
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
    let tx_data = client
      .get_transaction_data(programmable_transaction, gas_budget)
      .await?;
    let kinesis_signature = client.sign_transaction_data(&tx_data).await?;

    // execute tx
    let response = client
      .quorum_driver_api()
      .execute_transaction_block(
        Transaction::from_data(tx_data, vec![kinesis_signature]),
        SuiTransactionBlockResponseOptions::full_content(),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await?;

    let created = match response.clone().effects {
      Some(SuiTransactionBlockEffects::V1(effects)) => effects.created,
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
    let new_identity = match &new_identities[..] {
      [value] => value,
      _ => {
        return Err(Error::TransactionUnexpectedResponse(format!(
          "could not find new identity in response: {response:?}"
        )));
      }
    };

    Ok(new_identity.object_id())
  }
}
