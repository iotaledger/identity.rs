// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Uint8Array;
use std::cell::Cell;
use std::collections::HashSet;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::bindings::WasmIotaObjectData;
use crate::bindings::WasmObjectRef;
use crate::bindings::WasmSharedObjectRef;
use crate::bindings::WasmTransactionArgument;
use crate::bindings::WasmTransactionBuilder;
use crate::error::TsSdkError;
use crate::error::WasmError;
use crate::transaction_builder::TransactionBuilderTsSdk;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::BorrowIntentFnInternalT;
use identity_iota_interaction::ControllerIntentFnInternalT;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::MoveType;
use identity_iota_interaction::ProgrammableTransactionBcs;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "[string, number]")]
  pub(crate) type WasmControllerCouple;

  #[wasm_bindgen(typescript_type = "[string, string]")]
  pub(crate) type WasmTransferCouple;

  #[wasm_bindgen(typescript_type = "[ObjectRef, string]")]
  pub(crate) type WasmObjectRefAndType;

  #[wasm_bindgen(typescript_type = "Map<string, [TransactionArgument, IotaObjectData]>")]
  pub(crate) type WasmTxArgumentMap;
}

#[wasm_bindgen(module = "move_calls/identity")]
extern "C" {
  #[wasm_bindgen(js_name = "new_", catch)]
  async fn identity_new(did: &[u8], package: &str) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "newWithControllers", catch)]
  async fn identity_new_with_controllers(
    did: &[u8],
    controllers: Vec<WasmControllerCouple>,
    threshold: u64,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "approve", catch)]
  async fn approve_proposal(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    proposal_type: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeDeactivation", catch)]
  async fn propose_deactivation(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeDeactivation", catch)]
  async fn execute_deactivation(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeUpgrade", catch)]
  async fn propose_upgrade(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeUpgrade", catch)]
  async fn execute_upgrade(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeSend", catch)]
  async fn propose_send(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    assets: Vec<WasmTransferCouple>,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeSend", catch)]
  async fn execute_send(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    assets: Vec<WasmObjectRefAndType>,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeUpdate", catch)]
  async fn propose_update(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    did_doc: &[u8],
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeUpdate", catch)]
  async fn execute_update(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeBorrow", catch)]
  async fn propose_borrow(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    objects: Vec<String>,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeBorrow", catch)]
  async fn execute_borrow(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    objects: Vec<WasmIotaObjectData>,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTxArgumentMap),
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeConfigChange", catch)]
  async fn propose_config_change(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    controllers_to_add: Vec<WasmControllerCouple>,
    controllers_to_remove: Vec<String>,
    controllers_to_update: Vec<WasmControllerCouple>,
    package: &str,
    expiration: Option<u64>,
    threshold: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeConfigChange", catch)]
  async fn execute_config_change(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeControllerExecution", catch)]
  async fn propose_controller_execution(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    controller_cap_id: &str,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeControllerExecution", catch)]
  async fn execute_controller_execution(
    identity: WasmSharedObjectRef,
    capability: WasmObjectRef,
    proposal: &str,
    controller_cap_ref: WasmObjectRef,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTransactionArgument),
    package: &str,
  ) -> Result<Uint8Array, JsValue>;
}

pub struct IdentityMoveCallsTsSdk {}

impl IdentityMoveCalls for IdentityMoveCallsTsSdk {
  type Error = TsSdkError;
  type NativeTxBuilder = WasmTransactionBuilder;

  fn propose_borrow(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    objects: Vec<ObjectID>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let package_id = package_id.to_string();
    let objects = objects.into_iter().map(|obj| obj.to_string()).collect();

    futures::executor::block_on(propose_borrow(
      identity,
      controller_cap,
      objects,
      &package_id,
      expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn execute_borrow<F: BorrowIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let proposal = proposal_id.to_string();
    let package = package.to_string();
    let objects = objects
      .into_iter()
      .map(|obj| serde_wasm_bindgen::to_value(&obj).map(WasmIotaObjectData::from))
      .collect::<Result<Vec<_>, _>>()
      .map_err(WasmError::from)?;

    // Use cell to move `intent_fn` inside `closure` without actually moving it.
    // This ensures that `closure` is an `impl Fn(..)` instead of `impl FnOnce(..)` like `intent_fn`.
    let wrapped_intent_fn = Cell::new(Some(intent_fn));
    let closure = |tx_builder: WasmTransactionBuilder, args: WasmTxArgumentMap| {
      let mut builder = TransactionBuilderTsSdk::new(tx_builder);
      let args = serde_wasm_bindgen::from_value(args.into()).expect("failed to convert JS argument map");
      wrapped_intent_fn.take().unwrap()(&mut builder, &args);
    };

    futures::executor::block_on(execute_borrow(
      identity, capability, &proposal, objects, &closure, &package,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn create_and_execute_borrow<F: BorrowIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> { todo!() }

  fn propose_config_change<I1, I2>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    expiration: Option<u64>,
    threshold: Option<u64>,
    controllers_to_add: I1,
    controllers_to_remove: HashSet<ObjectID>,
    controllers_to_update: I2,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    I1: IntoIterator<Item=(IotaAddress, u64)>,
    I2: IntoIterator<Item=(ObjectID, u64)>,
  {
    let identity = identity.try_into()?;
    let capability = controller_cap.into();
    let package = package.to_string();

    let controllers_to_add = controllers_to_add
      .into_iter()
      .map(|controller| serde_wasm_bindgen::to_value(&controller).map(WasmControllerCouple::from))
      .collect::<Result<Vec<_>, _>>()
      .map_err(WasmError::from)?;
    let controllers_to_remove = controllers_to_remove
      .into_iter()
      .map(|controller| controller.to_string())
      .collect();
    let controllers_to_update = controllers_to_update
      .into_iter()
      .map(|controller| serde_wasm_bindgen::to_value(&controller).map(WasmControllerCouple::from))
      .collect::<Result<Vec<_>, _>>()
      .map_err(WasmError::from)?;

    futures::executor::block_on(propose_config_change(
      identity,
      capability,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
      &package,
      expiration,
      threshold,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn execute_config_change(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = controller_cap.into();
    let proposal = proposal_id.to_string();
    let package = package.to_string();

    futures::executor::block_on(execute_config_change(identity, capability, &proposal, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn propose_controller_execution(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    controller_cap_id: ObjectID,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let package_id = package_id.to_string();
    let borrowed_cap = controller_cap_id.to_string();

    futures::executor::block_on(propose_controller_execution(
      identity,
      controller_cap,
      &borrowed_cap,
      &package_id,
      expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn execute_controller_execution<F: ControllerIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let proposal = proposal_id.to_string();
    let package = package.to_string();
    let borrowing_cap = borrowing_controller_cap_ref.into();

    // Use cell to move `intent_fn` inside `closure` without actually moving it.
    // This ensures that `closure` is an `impl Fn(..)` instead of `impl FnOnce(..)` like `intent_fn`.
    let wrapped_intent_fn = Cell::new(Some(intent_fn));
    let closure = |tx_builder: WasmTransactionBuilder, args: WasmTransactionArgument| {
      let mut builder = TransactionBuilderTsSdk::new(tx_builder);
      let args = serde_wasm_bindgen::from_value(args.into()).expect("failed to convert JS argument map");
      wrapped_intent_fn.take().unwrap()(&mut builder, &args);
    };

    futures::executor::block_on(execute_controller_execution(
      identity,
      capability,
      &proposal,
      borrowing_cap,
      &closure,
      &package,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let package = package_id.to_string();
    futures::executor::block_on(identity_new(did_doc, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn new_with_controllers<C>(
    did_doc: &[u8],
    controllers: C,
    threshold: u64,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    C: IntoIterator<Item = (IotaAddress, u64)>,
  {
    let package = package_id.to_string();
    let controllers = controllers
      .into_iter()
      .map(|controller| serde_wasm_bindgen::to_value(&controller).map(|js_value| js_value.unchecked_into()))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| WasmError::from(e))?;

    futures::executor::block_on(identity_new_with_controllers(did_doc, controllers, threshold, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn propose_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let package = package_id.to_string();

    futures::executor::block_on(propose_deactivation(identity, capability, &package, expiration))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn execute_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let proposal = proposal_id.to_string();
    let package = package_id.to_string();

    futures::executor::block_on(execute_deactivation(identity, capability, &proposal, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn approve_proposal<T: MoveType>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = controller_cap.into();
    let proposal_id = proposal_id.to_string();
    let package_id = package.to_string();

    futures::executor::block_on(approve_proposal(
      identity,
      controller_cap,
      &proposal_id,
      &T::move_type(package).to_canonical_string(true),
      &package_id,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn propose_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let package_id = package_id.to_string();
    let transfer_map = transfer_map
      .into_iter()
      .map(|tx| serde_wasm_bindgen::to_value(&tx).map(JsValue::into))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| WasmError::from(e))?;

    futures::executor::block_on(propose_send(
      identity,
      controller_cap,
      transfer_map,
      &package_id,
      expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn create_and_execute_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> { todo!() }

  fn execute_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let proposal = proposal_id.to_string();
    let package_id = package.to_string();
    let objects = objects
      .into_iter()
      .map(|tx| serde_wasm_bindgen::to_value(&tx).map(JsValue::into))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| WasmError::from(e))?;

    futures::executor::block_on(execute_send(identity, controller_cap, &proposal, objects, &package_id))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn propose_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    did_doc: impl AsRef<[u8]>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let did_doc = did_doc.as_ref();
    let package_id = package_id.to_string();

    futures::executor::block_on(propose_update(
      identity,
      controller_cap,
      did_doc,
      &package_id,
      expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn execute_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let proposal = proposal_id.to_string();
    let package_id = package_id.to_string();

    futures::executor::block_on(execute_update(identity, controller_cap, &proposal, &package_id))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn propose_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let package = package_id.to_string();

    futures::executor::block_on(propose_upgrade(identity, capability, &package, expiration))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn execute_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let proposal = proposal_id.to_string();
    let package = package_id.to_string();

    futures::executor::block_on(execute_upgrade(identity, capability, &proposal, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }
}
