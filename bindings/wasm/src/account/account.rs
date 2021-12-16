use crate::did::WasmDID;
use crate::error::{Result, WasmResult};
use identity::account::{Account, IdentityUpdater, MethodSecret, Update};
use identity::did::{MethodScope, MethodType};
use identity::iota::TangleRef;
use js_sys::Promise;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use identity::core::OneOrMany;
use identity::core::OneOrMany::{Many, One};
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(Rc<WasmRefCell<Account>>);

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

  #[wasm_bindgen(js_name = createMethod)]
  pub fn create_method(&mut self, input: &CreateMethodInput) -> Result<Promise> {
    let elements: OneOrMany<String> = input.elements().into_serde().wasm_result()?;
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("logging works");

    if let One(el) = elements.clone() {
      log::info!("one");
      log::info!("{}", el);
    }


    if let Many(el) = elements.clone() {
      log::info!("many");
      log::info!("first {} second {}", el.get(0).unwrap(), el.get(1).unwrap());
    }




    let fragment = input.fragment().unwrap();
    let account = self.0.clone();

    let promise = future_to_promise(async move {
      let update = Update::CreateMethod {
        type_: MethodType::Ed25519VerificationKey2018,
        fragment,
        method_secret: None,
        scope: MethodScope::VerificationMethod,
      };

      let res = account.as_ref().borrow_mut().process_update(update).await.wasm_result();
      return Ok(JsValue::from("hello".to_owned()));
    });

    Ok(promise)
  }
}

impl From<Account> for WasmAccount {
  fn from(account: Account) -> WasmAccount {
    WasmAccount(Rc::new(WasmRefCell::new(account)))
  }
}

#[wasm_bindgen]
extern "C" {
  pub type CreateMethodInput;

  #[wasm_bindgen(structural, getter, method)]
  pub fn fragment(this: &CreateMethodInput) -> Option<String>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn elements(this: &CreateMethodInput) -> JsValue;

}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type CreateMethodInput = {
  "fragment": string,
  "elements": string | string[]
};
"#;
