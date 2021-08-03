use wasm_bindgen::prelude::*;

// The duck-typed JS ActorRequest interface defined in Rust.
#[wasm_bindgen]
extern "C" {
  pub type ActorRequest;

  // TODO: Can this be a getter?
  // I.e.: Does it matter if requestName is serialized?
  #[wasm_bindgen(structural, method, js_name = requestName)]
  pub fn request_name(this: &ActorRequest) -> String;
}
