use crate::account::account::WasmAccount;
use crate::error::{Result, WasmResult};
use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::IdentitySetup;
use js_sys::Promise;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder {
  account_builder: Rc<Mutex<AccountBuilder>>,
}

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      account_builder: Rc::new(Mutex::new(AccountBuilder::new().autopublish(false))),
    }
  }

  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&mut self, input: WasmIdentitySetup) -> Result<Promise> {
    let account_builder: Rc<Mutex<AccountBuilder>> = self.account_builder.clone();

    let promise: Promise = future_to_promise(async move {
      let mut guard = account_builder.lock().wasm_result()?;
      let res = (*guard).create_identity(input.0).await;
      let res2 = res.map(WasmAccount::from).map(Into::into).wasm_result();
      res2
    });
    Ok(promise)
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
