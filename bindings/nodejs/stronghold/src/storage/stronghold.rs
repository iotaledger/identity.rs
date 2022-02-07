// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use hashbrown::HashMap;
use identity_account::storage::Storage;
use identity_account::storage::Stronghold;
use identity_account::stronghold::Snapshot;
use identity_wasm::did::WasmDID;
use napi::Result;
use tokio::sync::Mutex;

use crate::error::NapiResult;

#[napi(js_name = Stronghold)]
#[derive(Debug)]
pub struct JsStronghold(pub(crate) Stronghold);

#[napi]
impl JsStronghold {
  //#[napi]
  //pub async fn new(
  //  snapshot: String,
  //  password: String,
  //  dropsave: Option<bool>,
  //) -> Result<NodeStronghold> {
  //  Ok(NodeStronghold(
  //    Stronghold::new(snapshot.as_str(), password.as_str(), dropsave)
  //      .await
  //      .napi_result()?,
  //  ))
  //}

  /// Returns whether save-on-drop is enabled.
  #[napi(getter)]
  pub fn dropsave(&self) -> bool {
    self.0.dropsave()
  }

  /// Set whether to save the storage changes on drop.
  /// Default: true
  #[napi(setter)]
  pub fn set_dropsave(&mut self, dropsave: bool) {
    self.0.set_dropsave(dropsave);
  }

  //  #[napi]
  //  pub async fn set_password(&self, password: Vec<u32>) -> Result<()> {
  //    unimplemented!();
  //  }
  //
  //  #[napi]
  //  pub fn flush_changes(&self) -> Result<()> {
  //    unimplemented!();
  //  }
}

