// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account_builder::WasmAutoSave;
use crate::did::{
  PromiseResolvedDocument, WasmDID, WasmResolvedDocument,
};
use crate::error::{WasmResult};
use crate::tangle::Client;
use identity::account::{Account, AccountBuilder, AccountStorage};
use identity::iota::{IotaDocument};
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(pub(crate) Rc<WasmRefCell<Account>>);

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  #[wasm_bindgen(js_name = testAccount)]
  pub fn test_account(&self) -> String {
    return String::from("test success");
  }

  #[wasm_bindgen(js_name = did)]
  pub fn did(&self) -> WasmDID {
    let x = self.0.as_ref().borrow();
    WasmDID::from(x.document().id().clone())
  }

  pub fn storage(&self) {
    unimplemented!() //ToDo
  }

  #[wasm_bindgen]
  pub fn autopublish(&self) -> bool {
    self.0.as_ref().borrow().autopublish()
  }

  #[wasm_bindgen]
  pub fn autosave(&self) -> WasmAutoSave {
    unimplemented!() //ToDo
  }

  #[wasm_bindgen]
  pub fn actions(&self) -> usize {
    self.0.as_ref().borrow().actions()
  }

  pub fn set_client(&self, _client: Client) {
    unimplemented!() //ToDo
  }

  pub fn state(&self) {
    unimplemented!() //ToDo
  }

  pub fn document(&self) {
    let _document: &IotaDocument = self.0.as_ref().borrow().document();
    //ToDo return a copy?
  }

  #[wasm_bindgen(js_name = resolveIdentity)]
  pub fn resolve_identity(&self) -> PromiseResolvedDocument {
    let account = self.0.clone();

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow()
        .resolve_identity()
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });
    // WARNING: this does not validate the return type. Check carefully.
    promise.unchecked_into::<PromiseResolvedDocument>()
  }

  #[wasm_bindgen(js_name = deleteIdentity)]
  pub fn delete_identity(self) -> Promise {
    let account = self.0.clone();
    let did = account.as_ref().borrow().did().to_owned();
    let storage = account.as_ref().borrow().storage_arc();
    std::mem::drop(account);

    let promise: Promise = future_to_promise(async move {
      let account = AccountBuilder::new()
        .storage(AccountStorage::Custom(storage))
        .load_identity(did)
        .await
        .wasm_result();

      match account {
        Ok(a) => a.delete_identity().await.wasm_result().map(|_| JsValue::undefined()),
        Err(e) => Err(e),
      }
    });
    promise
  }

  #[wasm_bindgen]
  pub fn publish(&mut self) -> Promise {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .publish()
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
  }
}

impl From<Account> for WasmAccount {
  fn from(account: Account) -> WasmAccount {
    WasmAccount(Rc::new(WasmRefCell::new(account)))
  }
}
