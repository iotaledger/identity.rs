// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account::WasmAccount;
use crate::did::WasmDID;
use crate::error::{Result, WasmResult};
use crate::tangle::Client as WasmClient;
use crate::tangle::WasmNetwork;
use identity::account::AccountBuilder;
use identity::account::IdentitySetup;
use identity::account::{AccountConfig, AutoSave};
use identity::iota::Client;
use js_sys::Promise;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder(Rc<WasmRefCell<AccountBuilder>>);

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<AccountBuilderOptions>) -> Self {
    let default_config: AccountConfig = AccountConfig::default();
    let mut builder = AccountBuilder::new();
    if let Some(o) = options {
      builder = builder
        .autopublish(o.autopublish().unwrap_or(default_config.autopublish))
        .milestone(o.milestone().unwrap_or(default_config.milestone));
        //todo autosave
        //todo storage
      if let Some(c) = o.client() {
        builder = builder.client(Arc::new(c.client.as_ref().clone()));
      };
    }

    Self(Rc::new(WasmRefCell::new(builder)))
  }

  #[wasm_bindgen(js_name = loadIdentity)]
  pub fn load_identity(&mut self, did: WasmDID) -> Result<PromiseAccount> {
    todo!()
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
  pub fn client(this: &AccountBuilderOptions) -> Option<WasmClient>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn milestone(this: &AccountBuilderOptions) -> Option<u32>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autoSave(this: &AccountBuilderOptions) -> Option<WasmAutoSave>;
}

//ToDo separate identitySetup.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AccountBuilderOptions = {
  autopublish?: boolean,
  milestone?: number,
  client?: Client
};
"#;
