// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::HashSet;
use std::hash::Hash;

use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::rpc_types::SuiParsedData;
use sui_sdk::rpc_types::SuiParsedMoveObject;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::id::UID;
use sui_sdk::SuiClient;

use crate::Error;

const MODULE: &str = "document";
const NAME: &str = "Document";

#[derive(Debug, Deserialize, Serialize)]
pub struct SuiVecSet<T: Hash + Eq> {
  contents: HashSet<T>,
}

impl<T: Hash + Eq> Deref for SuiVecSet<T> {
  type Target = HashSet<T>;
  fn deref(&self) -> &Self::Target {
    &self.contents
  }
}

impl<T: Hash + Eq> DerefMut for SuiVecSet<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.contents
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
  pub id: UID,
  pub doc: Vec<u8>,
  pub can_trade_controllers: bool,
  pub controllers: SuiVecSet<ObjectID>,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub threshold: u64,
}

pub async fn get_identity_document(client: &SuiClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
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

  if !is_document(&value) {
    return Ok(None);
  }

  serde_json::from_value(value.fields.to_json_value()).map_err(|err| {
    Error::ObjectLookup(format!(
      "could not parse identity document with object id {object_id}; {err}"
    ))
  })
}

fn is_document(value: &SuiParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}
