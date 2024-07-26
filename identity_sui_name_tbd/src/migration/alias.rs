// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaParsedData;
use iota_sdk::rpc_types::IotaParsedMoveObject;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::id::UID;
use iota_sdk::IotaClient;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;

const MODULE: &str = "alias";
const NAME: &str = "Alias";

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmigratedAlias {
  /// The ID of the Alias = hash of the Output ID that created the Alias Output in Stardust.
  /// This is the AliasID from Stardust.
  pub id: UID,

  /// The last State Controller address assigned before the migration.
  pub legacy_state_controller: Option<String>,
  /// A counter increased by 1 every time the alias was state transitioned.
  pub state_index: u32,
  /// State metadata that can be used to store additional information.
  pub state_metadata: Option<Vec<u8>>,

  /// The sender feature.
  pub sender: Option<String>,
  /// The metadata feature.
  pub metadata: Option<Vec<u8>>,

  /// The immutable issuer feature.
  pub immutable_issuer: Option<String>,
  /// The immutable metadata feature.
  pub immutable_metadata: Option<Vec<u8>>,
}

pub async fn get_alias(client: &IotaClient, object_id: ObjectID) -> Result<Option<UnmigratedAlias>, Error> {
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
        "could not get object with options for this object_id {object_id}; {err}"
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

  if !is_alias(&value) {
    return Ok(None);
  }

  let alias: UnmigratedAlias = serde_json::from_value(value.fields.to_json_value()).map_err(|err| {
    Error::ParsingFailed(format!(
      "could not parse result for object id {object_id} to `UnmigratedAlias`; {err}"
    ))
  })?;

  Ok(Some(alias))
}

fn is_alias(value: &IotaParsedMoveObject) -> bool {
  // if available we might also check if object stems from expected module
  // but how would this act upon package updates?
  value.type_.module.as_ident_str().as_str() == MODULE && value.type_.name.as_ident_str().as_str() == NAME
}
