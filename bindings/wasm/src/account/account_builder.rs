use crate::account::account::WasmAccount;
use crate::error::{Result, WasmResult};
use identity::account::AccountBuilder;
use identity::account::IdentitySetup;
use identity::account::{Account, AccountConfig, AutoSave};
use js_sys::Promise;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder {
  account_builder: Rc<WasmRefCell<AccountBuilder>>,
}

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      account_builder: Rc::new(WasmRefCell::new(AccountBuilder::new())),
    }
  }

  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(options: AccountOptions) -> Result<PromiseAccount> {
    let promise: Promise = future_to_promise(async move {
      let default_config: AccountConfig = AccountConfig::default();

      AccountBuilder::new()
        .autopublish(options.autopublish().unwrap_or(default_config.autopublish))
        .milestone(options.milestone().unwrap_or(default_config.milestone))
        .create_identity(options.identitySetup().0)
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
    Self(AutoSave::Never)
  }
  #[wasm_bindgen]
  pub fn batch(number_of_actions: usize) -> WasmAutoSave {
    Self(AutoSave::Never)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Account>")]
  pub type PromiseAccount;

  #[wasm_bindgen(typescript_type = "AccountOptions")]
  pub type AccountOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autopublish(this: &AccountOptions) -> Option<bool>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn milestone(this: &AccountOptions) -> Option<u32>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autoSave(this: &AccountOptions) -> Option<WasmAutoSave>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn identitySetup(this: &AccountOptions) -> WasmIdentitySetup;

}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AccountOptions = {
  identitySetup: IdentitySetup,
  autopublish?: boolean,
  milestone?: number,
  autoSave?: AutoSave };
"#;
