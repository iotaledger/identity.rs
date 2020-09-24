use identity_core::did::DID;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);
  #[wasm_bindgen(js_namespace = console)]
  pub fn error(s: &str);
}

#[wasm_bindgen(js_name = "Greet")]
pub fn greet() -> Result<String, JsValue> {
  console_error_panic_hook::set_once();

  let did = DID {
    method_name: "iota".into(),
    id_segments: vec!["123456".into()],
    ..Default::default()
  }
  .init()
  .unwrap();
  Ok(did.to_string())
}
