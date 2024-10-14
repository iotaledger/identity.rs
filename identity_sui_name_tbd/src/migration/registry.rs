// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::sui::iota_sdk_adapter::IotaClientTraitCore;
use crate::iota_sdk_abstraction::rpc_types::IotaData;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::id::ID;

use crate::client::IdentityClientReadOnly;

use super::get_identity;
use super::OnChainIdentity;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  ClientError(anyhow::Error),
  #[error("could not locate MigrationRegistry object: {0}")]
  NotFound(String),
  #[error("malformed MigrationRegistry's entry: {0}")]
  Malformed(String),
}

/// Lookup a legacy `alias_id` into the migration registry
/// to get the UID of the corresponding migrated DID document if any.
pub async fn lookup<C: IotaClientTraitCore>(
  iota_client: &IdentityClientReadOnly<C>,
  alias_id: ObjectID,
) -> Result<Option<OnChainIdentity>, Error> {
  let dynamic_field_name = serde_json::from_value(serde_json::json!({
    "type": "0x2::object::ID",
    "value": alias_id.to_string()
  }))
  .expect("valid move value");

  let identity_id = iota_client
    .read_api()
    .get_dynamic_field_object(iota_client.migration_registry_id(), dynamic_field_name)
    .await
    .map_err(|e| Error::ClientError(e.into()))?
    .data
    .map(|data| {
      data
        .content
        .and_then(|content| content.try_into_move())
        .and_then(|move_object| move_object.fields.to_json_value().get_mut("value").map(std::mem::take))
        .and_then(|value| serde_json::from_value::<ID>(value).map(|id| id.bytes).ok())
        .ok_or(Error::Malformed(
          "invalid MigrationRegistry's Entry encoding".to_string(),
        ))
    })
    .transpose()?;

  if let Some(id) = identity_id {
    get_identity(iota_client, id)
      .await
      .map_err(|e| Error::ClientError(e.into()))
  } else {
    Ok(None)
  }
}
