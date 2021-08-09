use identity::{
  actor::{
    traits::ActorRequest as IotaActorRequest, IdentityList as ActorIdentityList,
    IdentityResolve as ActorIdentityResolve,
  },
  iota::IotaDID,
  prelude::*,
};
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;

#[wasm_bindgen]
#[derive(Debug)]
pub struct IdentityList(ActorIdentityList);

#[wasm_bindgen]
impl IdentityList {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(ActorIdentityList)
  }

  #[wasm_bindgen(js_name = requestName)]
  pub fn request_name(&self) -> String {
    self.0.request_name().into()
  }

  pub fn __serialize(&self) -> JsValue {
    JsValue::from_serde(&self.0).unwrap()
  }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct IdentityResolve(ActorIdentityResolve);

#[wasm_bindgen]
impl IdentityResolve {
  #[wasm_bindgen(constructor)]
  pub fn new(did: &str) -> Result<IdentityResolve, JsValue> {
    let did: IotaDID = did.parse().map_err(wasm_error)?;
    let resolve = ActorIdentityResolve::new(did);
    Ok(Self(resolve))
  }

  #[wasm_bindgen(js_name = requestName)]
  pub fn request_name(&self) -> String {
    self.0.request_name().into()
  }

  pub fn __serialize(&self) -> JsValue {
    JsValue::from_serde(&self.0).unwrap()
  }
}
