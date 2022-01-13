use wasm_bindgen::prelude::*;

use identity::account::IdentityState as IdentityState_;

use crate::did::WasmDocument;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WasmIdentityState(pub(crate) IdentityState_);

#[wasm_bindgen]
impl WasmIdentityState {
  #[wasm_bindgen]
  pub fn new(document: WasmDocument) -> WasmIdentityState {
    WasmIdentityState(IdentityState_::new(document.into()))
  }
}

impl From<IdentityState_> for WasmIdentityState {
  fn from(identity_state: IdentityState_) -> Self {
    WasmIdentityState(identity_state)
  }
}
