use identity_iota::core::Object;
use identity_iota::iota::client_dummy::Multicontroller;
use identity_iota::iota::client_dummy::ObjectID;
use identity_iota::iota::client_dummy::Proposal;
use wasm_bindgen::prelude::*;

use crate::common::MapStringAny;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(js_name = Proposal)]
pub struct WasmProposal(pub(crate) Proposal);

#[wasm_bindgen(js_name = Multicontroller)]
pub struct WasmMulticontroller(pub(crate) Multicontroller<Vec<u8>>);

#[wasm_bindgen(js_class = Multicontroller)]
impl WasmMulticontroller {
  /// TODO: remove this, added to test interface in ts
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(Multicontroller::<Vec<u8>>::new(vec![]))
  }

  #[wasm_bindgen(js_name = controlledValue)]
  pub fn controlled_value(&self) -> Vec<u8> {
    self.0.controlled_value().clone()
  }

  pub fn threshold(&self) -> u64 {
    self.0.threshold()
  }

  #[wasm_bindgen(js_name = controllerVotingPower)]
  pub fn controller_voting_power(&self, controller_cap_id: ObjectID) -> Option<u64> {
    self.0.controller_voting_power(controller_cap_id)
  }

  pub fn proposals(&self) -> Result<MapStringAny, JsValue> {
    let object_result: Result<Object, JsValue> = self
      .0
      .proposals()
      .iter()
      .map(|(k, v)| {
        serde_json::to_value(v)
          .map(|json_value| (k.to_owned(), json_value))
          .map_err(|err| JsValue::from_str(&format!("failed to serialize value; {err}")))
      })
      .collect();

    MapStringAny::try_from(object_result?)
  }

  /// Behaves as as alias for `controlled_value`, as TypeScript does not reflect consuming `self`
  /// very well. Using the `Multicontroller` afterwards instance would not produce a compiler error
  /// but would return a "null pointer passed to rust" error.
  ///
  /// TODO: consider removing this function from the bindings
  #[wasm_bindgen(js_name = intoInner)]
  pub fn into_inner(&self) -> Vec<u8> {
    self.controlled_value()
  }

  #[wasm_bindgen(js_name = hasMember)]
  pub fn has_member(&self, cap_id: ObjectID) -> bool {
    self.0.has_member(cap_id)
  }
}
