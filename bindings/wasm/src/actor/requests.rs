use identity::actor::{ActorRequest, Synchronous};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestRequest(String);

impl ActorRequest<Synchronous> for TestRequest {
  type Response = String;

  fn endpoint<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    std::borrow::Cow::Borrowed("test/request")
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Debug)]
#[repr(transparent)]
pub struct WasmTestRequest(TestRequest);

#[wasm_bindgen(inspectable)]
impl WasmTestRequest {
  #[wasm_bindgen(constructor)]
  pub fn new(string: String) -> Self {
    Self(TestRequest(string))
  }

  // TODO: This should probably be a separate function, and not a customized `toJSON`.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> JsValue {
    let request = serde_json::to_value(&self.0).expect("TODO");
    let json = serde_json::json!({
      "endpoint": String::from(self.0.endpoint()),
      "request": request,
    });

    JsValue::from_serde(&json).expect("TODO")
  }
}
