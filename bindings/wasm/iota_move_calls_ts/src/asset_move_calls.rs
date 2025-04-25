// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::types::execution_status::CommandArgumentError;
use js_sys::Uint8Array;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::bindings::WasmObjectRef;
use crate::bindings::WasmSharedObjectRef;
use crate::error::TsSdkError;
use crate::error::WasmError;
use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::ObjectRef;
use iota_interaction::types::base_types::SequenceNumber;
use iota_interaction::types::TypeTag;
use iota_interaction::AssetMoveCalls;
use iota_interaction::MoveType;
use iota_interaction::ProgrammableTransactionBcs;

#[wasm_bindgen(module = "@iota/iota-interaction-ts/move_calls/asset")]
extern "C" {
  #[wasm_bindgen(js_name = "create", catch)]
  pub(crate) async fn new_asset(
    inner_bytes: &[u8],
    inner_type: &str,
    mutable: bool,
    transferable: bool,
    deletable: bool,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(catch, js_name = "remove")]
  pub(crate) async fn delete(asset: WasmObjectRef, asset_type: &str, package: &str) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(catch)]
  pub(crate) async fn update(
    asset: WasmObjectRef,
    content: &[u8],
    content_type: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(catch)]
  pub(crate) async fn transfer(
    asset: WasmObjectRef,
    asset_type: &str,
    recipient: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "acceptProposal", catch)]
  pub(crate) async fn accept_proposal(
    proposal: WasmSharedObjectRef,
    recipient_cap: WasmObjectRef,
    asset: WasmObjectRef,
    asset_type: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;

  #[wasm_bindgen(js_name = "concludeOrCancel", catch)]
  pub(crate) async fn conclude_or_cancel(
    proposal: WasmSharedObjectRef,
    sender_cap: WasmObjectRef,
    asset: WasmObjectRef,
    asset_type: &str,
    package: &str,
  ) -> Result<Uint8Array, JsValue>;
}

pub struct AssetMoveCallsTsSdk {}

impl AssetMoveCalls for AssetMoveCallsTsSdk {
  type Error = TsSdkError;

  fn new_asset<T: Serialize + MoveType>(
    inner: &T,
    mutable: bool,
    transferable: bool,
    deletable: bool,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let inner_bytes = bcs::to_bytes(inner).map_err(|_| CommandArgumentError::InvalidBCSBytes)?;
    let inner_type = T::move_type(package).to_string();
    let package = package.to_string();

    futures::executor::block_on(new_asset(
      &inner_bytes,
      &inner_type,
      mutable,
      transferable,
      deletable,
      &package,
    ))
    .map(|js_arr| js_arr.to_vec())
    .map_err(WasmError::from)
    .map_err(TsSdkError::from)
  }

  fn delete<T: MoveType>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let asset = asset.into();
    let asset_type = T::move_type(package).to_string();
    let package = package.to_string();

    futures::executor::block_on(delete(asset, &asset_type, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn transfer<T: MoveType>(
    asset: ObjectRef,
    recipient: IotaAddress,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let asset = asset.into();
    let asset_type = T::move_type(package).to_string();
    let recipient = recipient.to_string();
    let package = package.to_string();

    futures::executor::block_on(transfer(asset, &asset_type, &recipient, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn make_tx(
    _proposal: (ObjectID, SequenceNumber),
    _cap: ObjectRef,
    _asset: ObjectRef,
    _asset_type_param: TypeTag,
    _package: ObjectID,
    _function_name: &'static str,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn accept_proposal(
    proposal: (ObjectID, SequenceNumber),
    recipient_cap: ObjectRef,
    asset: ObjectRef,
    asset_type: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let proposal = (proposal.0, proposal.1, true).into();
    let asset = asset.into();
    let asset_type = asset_type.to_canonical_string(true);
    let recipient = recipient_cap.into();
    let package = package.to_string();

    futures::executor::block_on(accept_proposal(proposal, recipient, asset, &asset_type, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn conclude_or_cancel(
    proposal: (ObjectID, SequenceNumber),
    sender_cap: ObjectRef,
    asset: ObjectRef,
    asset_type: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let proposal = (proposal.0, proposal.1, true).into();
    let asset = asset.into();
    let asset_type = asset_type.to_canonical_string(true);
    let sender = sender_cap.into();
    let package = package.to_string();

    futures::executor::block_on(conclude_or_cancel(proposal, sender, asset, &asset_type, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }

  fn update<T: MoveType + Serialize>(
    asset: ObjectRef,
    new_content: &T,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let asset = asset.into();
    let content_type = T::move_type(package).to_string();
    let content = bcs::to_bytes(new_content).map_err(|_| CommandArgumentError::InvalidBCSBytes)?;
    let package = package.to_string();

    futures::executor::block_on(update(asset, &content, &content_type, &package))
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(TsSdkError::from)
  }
}
