// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;
use crate::account::account::WasmAccount;
use crate::error::{Result, WasmResult};
use crate::tangle::WasmNetwork;
use identity::account::AccountBuilder;
use identity::account::IdentitySetup;
use identity::account::{AccountConfig, AutoSave};
use js_sys::Promise;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use crate::did::WasmDID;

#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder  (Rc<WasmRefCell<AccountBuilder>>);

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {

  #[wasm_bindgen(constructor)]
  pub fn new(options: AccountBuilderOptions) -> Self {
    let default_config: AccountConfig = AccountConfig::default();
    Self{
      0: Rc::new(WasmRefCell::new(AccountBuilder::new()
        .autopublish(options.autopublish().unwrap_or(default_config.autopublish))
        .milestone(options.milestone().unwrap_or(default_config.milestone))
        // .autosave(options.autoSave().unwrap_or(default_config.autosave).0)
        //ToDo Client
      ))

    }
  }

  #[wasm_bindgen(js_name = loadIdentity)]
  pub fn load_identity(&mut self, did: WasmDID) -> Result<PromiseAccount> {
    //ToDo
    panic!("Not implemented yet, storage implementation required!");
    // let builder = self.0.clone();
    // let promise: Promise = future_to_promise(async move {
    //
    //   builder
    //     .as_ref()
    //     .borrow_mut()
    //     .load_identity(did.0)
    //     .await
    //     .map(WasmAccount::from)
    //     .map(Into::into)
    //     .wasm_result()
    // });
    // Ok(promise.unchecked_into::<PromiseAccount>())
  }

  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&mut self, identity_setup: WasmIdentitySetup) -> Result<PromiseAccount> {
    let builder = self.0.clone();
    let promise: Promise = future_to_promise(async move {
      builder
        .as_ref()
        .borrow_mut()
        .create_identity(identity_setup.0)
        .await
        .map(WasmAccount::from)
        .map(Into::into)
        .wasm_result()
    });
    Ok(promise.unchecked_into::<PromiseAccount>())
  }
}

#[wasm_bindgen(js_name = IdentitySetup)]
pub struct WasmIdentitySetup(pub(crate) IdentitySetup);

#[wasm_bindgen(js_class = IdentitySetup)]
impl WasmIdentitySetup {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      0: IdentitySetup::new(),
    }
  }
}

#[wasm_bindgen(js_name = AutoSave)]
pub struct WasmAutoSave(pub(crate) AutoSave);

#[wasm_bindgen(js_class = AutoSave)]
impl WasmAutoSave {
  #[wasm_bindgen]
  pub fn never() -> WasmAutoSave {
    Self(AutoSave::Never)
  }
  #[wasm_bindgen]
  pub fn every() -> WasmAutoSave {
    Self(AutoSave::Every)
  }
  #[wasm_bindgen]
  pub fn batch(number_of_actions: usize) -> WasmAutoSave {
    Self(AutoSave::Batch(number_of_actions))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Account>")]
  pub type PromiseAccount;

  #[wasm_bindgen(typescript_type = "AccountBuilderOptions")]
  pub type AccountBuilderOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autopublish(this: &AccountBuilderOptions) -> Option<bool>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn milestone(this: &AccountBuilderOptions) -> Option<u32>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autoSave(this: &AccountBuilderOptions) -> Option<WasmAutoSave>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn client(this: &AccountBuilderOptions) -> WasmNetwork;
}

//ToDo separate identitySetup.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AccountBuilderOptions = {
  autopublish?: boolean,
  milestone?: number,
  autoSave?: AutoSave,
  client?: Network };
"#;
