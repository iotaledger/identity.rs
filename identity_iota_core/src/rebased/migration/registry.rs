// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::rpc_types::IotaData;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::id::ID;
use iota_interaction::IotaClientTrait;

use crate::rebased::client::IdentityClientReadOnly;

use super::get_identity;
use super::OnChainIdentity;

/// Errors that can occur during migration registry operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// An error occurred while interacting with the IOTA Client.
  #[error(transparent)]
  Client(anyhow::Error),
  /// The MigrationRegistry object was not found.
  #[error("could not locate MigrationRegistry object: {0}")]
  NotFound(String),
  /// The MigrationRegistry object is malformed.
  #[error("malformed MigrationRegistry's entry: {0}")]
  Malformed(String),
}

/// Lookup a legacy `alias_id` into the migration registry
/// to get the UID of the corresponding migrated DID document if any.
pub async fn lookup(id_client: &IdentityClientReadOnly, alias_id: ObjectID) -> Result<Option<OnChainIdentity>, Error> {
  let dynamic_field_name = serde_json::from_value(serde_json::json!({
    "type": "0x2::object::ID",
    "value": alias_id.to_string()
  }))
  .expect("valid move value");

  let identity_id = id_client
    .read_api()
    .get_dynamic_field_object(id_client.migration_registry_id(), dynamic_field_name)
    .await
    .map_err(|e| Error::Client(e.into()))?
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
    get_identity(id_client, id).await.map_err(|e| Error::Client(e.into()))
  } else {
    Ok(None)
  }
}
