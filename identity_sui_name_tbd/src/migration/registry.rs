// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::rpc_types::IotaData;
use iota_sdk::types::base_types::ObjectID;

use crate::client::IdentityClientReadOnly;

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
pub async fn lookup(
  iota_client: &IdentityClientReadOnly,
  alias_id: ObjectID,
) -> Result<Option<OnChainIdentity>, Error> {
  let dynamic_field_name = serde_json::from_value(serde_json::json!({
    "type": "0x2::object::ID",
    "value": alias_id.to_string()
  }))
  .expect("valid move value");

  iota_client
    .read_api()
    .get_dynamic_field_object(iota_client.migration_registry_id(), dynamic_field_name)
    .await
    .map_err(|e| Error::ClientError(e.into()))?
    .data
    .map(|data| {
      data
        .content
        .and_then(|content| content.try_into_move())
        .and_then(|move_object| serde_json::from_value(move_object.fields.to_json_value()).ok())
        .ok_or(Error::Malformed(
          "invalid MigrationRegistry's Entry encoding".to_string(),
        ))
    })
    .transpose()
}
