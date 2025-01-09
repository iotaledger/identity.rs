use anyhow::anyhow;
use identity_iota_interaction::ident_str;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::object::Owner;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use identity_iota_interaction::MigrationMoveCalls;
use identity_iota_interaction::ProgrammableTransactionBcs;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::bindings::WasmObjectRef;
use crate::bindings::WasmSharedObjectRef;
use crate::error::TsSdkError;
use crate::error::WasmError;

#[wasm_bindgen(module = "move_calls")]
extern "C" {
  #[wasm_bindgen(js_name = "migrateDidOutput", catch)]
  async fn migrate_did_output_impl(
    did_output: WasmObjectRef,
    migration_registry: WasmSharedObjectRef,
    package: &str,
    creation_timestamp: Option<u64>,
  ) -> Result<Vec<u8>, JsValue>;
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
    let migration_registry = {
      let Owner::Shared { initial_shared_version } = migration_registry.owner else {
        return anyhow!("migration registry must be a shared object ref");
      };
      let obj_id = migration_registry.object_id();

      (obj_id, initial_shared_version, true).into()
    };

    let future = migrate_did_output_impl(did_output, migration_registry, &package.to_string(), creation_timestamp);
    futures::executor::block_on(future)
      .map_err(WasmError::from)
      .map_err(Self::Error::from)
  }
}
