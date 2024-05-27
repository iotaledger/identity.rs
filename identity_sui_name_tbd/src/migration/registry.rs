// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::OnceLock;

use serde;
use serde::Deserialize;
use sui_sdk::rpc_types::EventFilter;
use sui_sdk::rpc_types::SuiData;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::SuiClient;

use super::Document;

static MIGRATION_REGISTRY_ID: OnceLock<ObjectID> = OnceLock::new();

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  ClientError(anyhow::Error),
  #[error("could not locate MigrationRegistry object: {0}")]
  NotFound(String),
  #[error("malformed MigrationRegistry's entry: {0}")]
  Malformed(String),
}

#[derive(Deserialize)]
struct MigrationRegistryCreatedEvent {
  #[allow(dead_code)]
  id: ObjectID,
}

async fn migration_registry_id(sui_client: &SuiClient) -> Result<ObjectID, Error> {
  if let Some(registry_id) = MIGRATION_REGISTRY_ID.get() {
    return Ok(*registry_id);
  }

  let event_tag = {
    let package_id = std::env::var("IDENTITY_IOTA_PKG_ID").expect("set IDENTITY_IOTA_PKG_ID");
    let tag = format!("{package_id}::migration_registry::MigrationRegistryCreated");
    sui_sdk::types::parse_sui_struct_tag(&tag)
      .map_err(|e| Error::NotFound(format!("invalid event tag \"{tag}\": {}", e)))?
  };

  let mut returned_events = sui_client
    .event_api()
    .query_events(EventFilter::MoveEventType(event_tag), None, Some(1), false)
    .await
    .map_err(|e| Error::ClientError(e.into()))?
    .data;
  let event = if !returned_events.is_empty() {
    returned_events.swap_remove(0)
  } else {
    return Err(Error::NotFound(String::from("no \"MigrationRegistryCreated\" event")));
  };

  let registry_id = serde_json::from_value::<MigrationRegistryCreatedEvent>(event.parsed_json)
    .map(|e| e.id)
    .map_err(|e| Error::NotFound(format!("Malformed \"MigrationRegistryEvent\": {}", e)))?;

  let _ = MIGRATION_REGISTRY_ID.set(registry_id);
  Ok(registry_id)
}

/// Lookup a legacy `alias_id` into the migration registry
/// to get the UID of the corresponding migrated DID document if any.
pub async fn lookup(sui_client: &SuiClient, alias_id: ObjectID) -> Result<Option<Document>, Error> {
  let dynamic_field_name = serde_json::from_value(serde_json::json!({
    "type": "0x2::object::ID",
    "value": alias_id.to_string()
  }))
  .expect("valid move value");

  sui_client
    .read_api()
    .get_dynamic_field_object(migration_registry_id(sui_client).await?, dynamic_field_name)
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