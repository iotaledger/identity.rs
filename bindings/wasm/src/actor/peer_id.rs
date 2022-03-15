use std::convert::TryFrom;

use identity::actor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "peer-id")]
extern "C" {
  pub type PeerId;

  #[wasm_bindgen(method)]
  fn toBytes(this: &PeerId) -> Vec<u8>;
}

#[allow(clippy::from_over_into)]
impl Into<actor::PeerId> for PeerId {
  fn into(self) -> actor::PeerId {
    let addr_bytes = self.toBytes();
    actor::PeerId::try_from(addr_bytes).unwrap()
  }
}
