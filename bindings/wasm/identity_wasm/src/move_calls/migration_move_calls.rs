// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_move_calls::MigrationMoveCalls;
use iota_interaction::rpc_types::OwnedObjectRef;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::ObjectRef;
use iota_interaction::ProgrammableTransactionBcs;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use iota_interaction_ts::bindings::WasmObjectRef;
use iota_interaction_ts::bindings::WasmSharedObjectRef;
use iota_interaction_ts::error::TsSdkError;
use iota_interaction_ts::error::WasmError;

#[wasm_bindgen(module = "@iota/iota-move-calls-identity/move_calls")]
extern "C" {
  #[wasm_bindgen(js_name = "migrateDidOutput", catch)]
  async fn migrate_did_output_impl(
    did_output: WasmObjectRef,
    migration_registry: WasmSharedObjectRef,
    package: &str,
    creation_timestamp: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;
}

pub struct MigrationMoveCallsTsSdk {}

impl MigrationMoveCalls for MigrationMoveCallsTsSdk {
  type Error = TsSdkError;

  fn migrate_did_output(
    did_output: ObjectRef,
    creation_timestamp: Option<u64>,
    migration_registry: OwnedObjectRef,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    let did_output = did_output.into();
    let package = package.to_string();
    let migration_registry = migration_registry.try_into()?;

    futures::executor::block_on(migrate_did_output_impl(
      did_output,
      migration_registry,
      &package,
      creation_timestamp,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(Self::Error::from)
  }
}
