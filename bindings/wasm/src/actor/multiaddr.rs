use std::convert::TryFrom;

use identity::actor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "multiaddr")]
extern "C" {
  pub type Multiaddr;

  #[wasm_bindgen(method, getter)]
  fn bytes(this: &Multiaddr) -> Vec<u8>;
}

#[allow(clippy::from_over_into)]
impl Into<actor::Multiaddr> for Multiaddr {
  fn into(self) -> actor::Multiaddr {
    let addr_bytes = self.bytes();
    actor::Multiaddr::try_from(addr_bytes).unwrap()
  }
}
