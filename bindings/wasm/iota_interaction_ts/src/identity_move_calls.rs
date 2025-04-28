// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_iota_interaction::ControllerTokenRef;
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
use crate::common::PromiseUint8Array;
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
  #[wasm_bindgen(typescript_type = "[string, number, bool]")]
  pub(crate) type WasmControllerData;

  #[wasm_bindgen(typescript_type = "[string, string]")]
  pub(crate) type WasmTransferCouple;

  #[wasm_bindgen(typescript_type = "[ObjectRef, string]")]
  pub(crate) type WasmObjectRefAndType;

  #[wasm_bindgen(typescript_type = "Map<string, [TransactionArgument, IotaObjectData]>")]
  pub(crate) type WasmTxArgumentMap;
}

impl From<(IotaAddress, u64, bool)> for WasmControllerData {
  fn from((address, vp, can_delegate): (IotaAddress, u64, bool)) -> Self {
    let address = JsValue::from_str(&address.to_string());
    let vp = JsValue::bigint_from_str(&vp.to_string());
    let can_delegate_flag = JsValue::from_bool(can_delegate);

    let arr = js_sys::Array::new();
    arr.push(&address);
    arr.push(&vp);
    arr.push(&can_delegate_flag);

    arr.unchecked_into()
  }
}

#[wasm_bindgen(module = "@iota/iota-interaction-ts/move_calls/identity")]
extern "C" {
  #[wasm_bindgen(typescript_type = "ControllerTokenRef")]
  type WasmControllerTokenRef;

  #[wasm_bindgen(js_name = "create", catch)]
  fn identity_new(did: Option<&[u8]>, package: &str) -> Result<PromiseUint8Array, JsValue>;

  #[wasm_bindgen(js_name = "newWithControllers", catch)]
  fn identity_new_with_controllers(
    did: Option<&[u8]>,
    controllers: Vec<WasmControllerData>,
    threshold: u64,
    package: &str,
  ) -> Result<PromiseUint8Array, JsValue>;

  #[wasm_bindgen(js_name = "approve", catch)]
  async fn approve_proposal(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    proposal_type: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeDeactivation", catch)]
  fn propose_deactivation(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<PromiseUint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeDeactivation", catch)]
  async fn execute_deactivation(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeUpgrade", catch)]
  async fn propose_upgrade(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeUpgrade", catch)]
  async fn execute_upgrade(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeSend", catch)]
  async fn propose_send(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    assets: Vec<WasmTransferCouple>,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeSend", catch)]
  async fn execute_send(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    assets: Vec<WasmObjectRefAndType>,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeUpdate", catch)]
  fn propose_update(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    did_doc: Option<&[u8]>,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<PromiseUint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeUpdate", catch)]
  fn execute_update(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    package: &str,
  ) -> Result<PromiseUint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeBorrow", catch)]
  async fn propose_borrow(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    objects: Vec<String>,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeBorrow", catch)]
  async fn execute_borrow(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    objects: Vec<WasmIotaObjectData>,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTxArgumentMap),
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "createAndExecuteBorrow", catch)]
  async fn create_and_execute_borrow(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    objects: Vec<WasmIotaObjectData>,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTxArgumentMap),
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeConfigChange", catch)]
  async fn propose_config_change(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    controllers_to_add: Vec<WasmControllerData>,
    controllers_to_remove: Vec<String>,
    controllers_to_update: Vec<WasmControllerData>,
    package: &str,
    expiration: Option<u64>,
    threshold: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeConfigChange", catch)]
  async fn execute_config_change(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "proposeControllerExecution", catch)]
  async fn propose_controller_execution(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    controller_cap_id: &str,
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "executeControllerExecution", catch)]
  async fn execute_controller_execution(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    proposal: &str,
    controller_cap_ref: WasmObjectRef,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTransactionArgument),
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "createAndExecuteControllerExecution", catch)]
  async fn create_and_execute_controller_execution(
    identity: WasmSharedObjectRef,
    capability: WasmControllerTokenRef,
    controller_cap_ref: WasmObjectRef,
    intent_fn: &dyn Fn(WasmTransactionBuilder, WasmTransactionArgument),
    package: &str,
    expiration: Option<u64>,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = delegateControllerCap, catch)]
  async fn delegate_controller_cap(
    controller_cap: WasmObjectRef,
    recipient: &str,
    permissions: u32,
    package_id: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = revokeDelegationToken, catch)]
  async fn revoke_delegation_token(
    identity: WasmSharedObjectRef,
    controller_cap: WasmObjectRef,
    delegation_token_id: &str,
    package_id: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = unrevokeDelegationToken, catch)]
  async fn unrevoke_delegation_token(
    identity: WasmSharedObjectRef,
    controller_cap: WasmObjectRef,
    delegation_token_id: &str,
    package_id: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = destroyDelegationToken, catch)]
  async fn destroy_delegation_token(
    identity: WasmSharedObjectRef,
    delegation_token: WasmObjectRef,
    package_id: &str,
  ) -> Result<Uint8Array, JsValue>;
}

impl From<ControllerTokenRef> for WasmControllerTokenRef {
  fn from(value: ControllerTokenRef) -> Self {
    use js_sys::Object;    
    use js_sys::Reflect;

    let wasm_object_ref = WasmObjectRef::from(value.object_ref());
    let js_object = Object::new();
    let _ = Reflect::set(&js_object, &JsValue::from_str("objectRef"), &wasm_object_ref);

    let wasm_type = {
      let type_name = match value {
        ControllerTokenRef::Controller(_) => "ControllerCap",
        ControllerTokenRef::Delegate(_) => "DelegationToken",
      };

      JsValue::from_str(type_name)
    };

    let _ = Reflect::set(&js_object, &JsValue::from_str("type"), &wasm_type);

    js_object.unchecked_into()
  }
}

pub struct IdentityMoveCallsTsSdk {}

#[async_trait(?Send)]
impl IdentityMoveCalls for IdentityMoveCallsTsSdk {
  type Error = TsSdkError;
  type NativeTxBuilder = WasmTransactionBuilder;

  fn propose_borrow(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
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
    capability: ControllerTokenRef,
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
    capability: ControllerTokenRef,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let package = package_id.to_string();
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

    futures::executor::block_on(create_and_execute_borrow(
      identity, capability, objects, &closure, &package, expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn propose_config_change<I1, I2>(
    identity: OwnedObjectRef,
    controller_cap: ControllerTokenRef,
    expiration: Option<u64>,
    threshold: Option<u64>,
    controllers_to_add: I1,
    controllers_to_remove: HashSet<ObjectID>,
    controllers_to_update: I2,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    I1: IntoIterator<Item = (IotaAddress, u64)>,
    I2: IntoIterator<Item = (ObjectID, u64)>,
  {
    let identity = identity.try_into()?;
    let capability = controller_cap.into();
    let package = package.to_string();

    let controllers_to_add = controllers_to_add
      .into_iter()
      .map(|controller| serde_wasm_bindgen::to_value(&controller).map(WasmControllerData::from))
      .collect::<Result<Vec<_>, _>>()
      .map_err(WasmError::from)?;
    let controllers_to_remove = controllers_to_remove
      .into_iter()
      .map(|controller| controller.to_string())
      .collect();
    let controllers_to_update = controllers_to_update
      .into_iter()
      .map(|controller| serde_wasm_bindgen::to_value(&controller).map(WasmControllerData::from))
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
    controller_cap: ControllerTokenRef,
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
    capability: ControllerTokenRef,
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
    capability: ControllerTokenRef,
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

  fn create_and_execute_controller_execution<F>(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    expiration: Option<u64>,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    F: ControllerIntentFnInternalT<Self::NativeTxBuilder>,
  {
    let identity = identity.try_into()?;
    let capability = capability.into();
    let package = package_id.to_string();
    let borrowing_cap = borrowing_controller_cap_ref.into();

    // Use cell to move `intent_fn` inside `closure` without actually moving it.
    // This ensures that `closure` is an `impl Fn(..)` instead of `impl FnOnce(..)` like `intent_fn`.
    let wrapped_intent_fn = Cell::new(Some(intent_fn));
    let closure = |tx_builder: WasmTransactionBuilder, args: WasmTransactionArgument| {
      let mut builder = TransactionBuilderTsSdk::new(tx_builder);
      let args = serde_wasm_bindgen::from_value(args.into()).expect("failed to convert JS argument map");
      wrapped_intent_fn.take().unwrap()(&mut builder, &args);
    };

    futures::executor::block_on(create_and_execute_controller_execution(
      identity,
      capability,
      borrowing_cap,
      &closure,
      &package,
      expiration,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  async fn new_identity(
    did_doc: Option<&[u8]>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let package = package_id.to_string();

    identity_new(did_doc, &package)
      .map_err(WasmError::from)?
      .to_programmable_transaction_bcs()
      .await
  }

  async fn new_with_controllers<C>(
    did_doc: Option<&[u8]>,
    controllers: C,
    threshold: u64,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    C: IntoIterator<Item = (IotaAddress, u64, bool)>,
  {
    let package = package_id.to_string();
    let controllers = controllers.into_iter().map(Into::into).collect();

    identity_new_with_controllers(did_doc, controllers, threshold, &package)
      .map_err(WasmError::from)?
      .to_programmable_transaction_bcs()
      .await
  }

  fn approve_proposal<T: MoveType>(
    identity: OwnedObjectRef,
    controller_cap: ControllerTokenRef,
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
    capability: ControllerTokenRef,
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
    _identity: OwnedObjectRef,
    _capability: ControllerTokenRef,
    _transfer_map: Vec<(ObjectID, IotaAddress)>,
    _expiration: Option<u64>,
    _objects: Vec<(ObjectRef, TypeTag)>,
    _package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
    todo!()
  }

  fn execute_send(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
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

  async fn propose_update(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    did_doc: Option<&[u8]>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let package_id = package_id.to_string();

    propose_update(identity, controller_cap, did_doc, &package_id, expiration)
      .map_err(WasmError::from)?
      .to_programmable_transaction_bcs()
      .await
  }

  async fn execute_update(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = capability.into();
    let proposal = proposal_id.to_string();
    let package_id = package_id.to_string();

    execute_update(identity, controller_cap, &proposal, &package_id)
      .map_err(WasmError::from)?
      .to_programmable_transaction_bcs()
      .await
  }

  fn propose_upgrade(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
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
    capability: ControllerTokenRef,
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

  async fn delegate_controller_cap(
    controller_cap: ObjectRef,
    recipient: IotaAddress,
    permissions: u32,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let controller_cap = controller_cap.into();
    let recipient = recipient.to_string();
    let package = package.to_string();

    delegate_controller_cap(controller_cap, &recipient, permissions, &package)
      .await
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  async fn revoke_delegation_token(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    delegation_token_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = controller_cap.into();
    let token_id = delegation_token_id.to_string();
    let package = package.to_string();

    revoke_delegation_token(identity, controller_cap, &token_id, &package)
      .await
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  async fn unrevoke_delegation_token(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    delegation_token_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let controller_cap = controller_cap.into();
    let token_id = delegation_token_id.to_string();
    let package = package.to_string();

    unrevoke_delegation_token(identity, controller_cap, &token_id, &package)
      .await
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  async fn destroy_delegation_token(
    identity: OwnedObjectRef,
    delegation_token: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let identity = identity.try_into()?;
    let token = delegation_token.into();
    let package = package.to_string();

    destroy_delegation_token(identity, token, &package)
      .await
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }
}
