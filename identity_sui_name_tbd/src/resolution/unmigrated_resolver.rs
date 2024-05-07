// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use serde_json::Value;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::rpc_types::SuiParsedData;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::SuiClient;

use crate::utils::get_client;
use crate::Error;

pub const LOCAL_NETWORK: &str = "http://127.0.0.1:9000";

pub struct UnmigratedResolver {
  client: SuiClient,
}

impl UnmigratedResolver {
  pub async fn new(network: &str) -> Result<Self, Error> {
    let client = get_client(network).await?;

    Ok(Self { client })
  }

  pub async fn get_alias_output(&self, object_id: &str) -> Result<Value, Error> {
    let object_id = ObjectID::from_str(object_id)
      .map_err(|err| Error::ObjectLookup(format!("Could not parse given object id {object_id}; {err}")))?;
    let options = SuiObjectDataOptions {
      show_type: true,
      show_owner: true,
      show_previous_transaction: true,
      show_display: true,
      show_content: true,
      show_bcs: true,
      show_storage_rebate: true,
    };
    let response = self
      .client
      .read_api()
      .get_object_with_options(object_id, options)
      .await
      .map_err(|err| {
        Error::ObjectLookup(format!(
          "Could not get object with options for this object_id {object_id}; {err}"
        ))
      })?;

    let data = response.data.ok_or_else(|| {
      Error::ObjectLookup(format!(
        "Call succeeded but could not get data for object id {object_id}"
      ))
    })?;
    let content = data
      .content
      .ok_or_else(|| Error::ObjectLookup(format!("no content in retrieved object in object id {object_id}")))?;

    let SuiParsedData::MoveObject(value) = content else {
      return Err(Error::ObjectLookup(format!(
        "found data at object id {object_id} is not an object"
      )));
    };

    let alias_output = value.fields.to_json_value();

    Ok(alias_output)
  }
}
