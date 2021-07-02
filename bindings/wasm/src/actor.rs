use std::{
  convert::{Infallible, TryFrom, TryInto},
  rc::Rc,
  sync::Arc,
};

use crate::utils::err;
use futures::executor;
use identity::{actor, prelude::*};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
// #[derive(Debug)]
pub struct IdentityActor {
  comm: Rc<identity::actor::IdentityCommunicator>,
}

#[wasm_bindgen]
impl IdentityActor {
  pub fn new() -> Result<IdentityActor, JsValue> {
    let comm = executor::block_on(async {
      let comm = identity::actor::IdentityCommunicator::new().await;
      comm
    });

    Ok(Self { comm: Rc::new(comm) })
  }

  #[wasm_bindgen(js_name = handleRequests)]
  pub fn handle_requests(&self) -> Result<Promise, JsValue> {
    let comm_clone = self.comm.clone();
    let promise = future_to_promise(async move {
      comm_clone
        .handle_requests()
        .await
        .map(|_| JsValue::undefined())
        .map_err(err)
    });

    Ok(promise)
  }

  #[wasm_bindgen(js_name = addPeer)]
  pub fn add_peer(&self, peer_id: PeerId, addr: Multiaddr) {
    let addr = addr.into();
    let peer_id = peer_id.into();
    self.comm.add_peer(peer_id, addr);
  }

  #[wasm_bindgen(js_name = sendCommand)]
  pub fn send_message(&self, peer_id: PeerId, message: NamedMessage) -> Result<Promise, JsValue> {
    let peer_id = peer_id.into();

    let comm_clone = self.comm.clone();

    let promise = future_to_promise(async move {
      let retval = comm_clone
        .send_command::<serde_json::Value, _>(peer_id, message.0)
        .await;

      match retval {
        Ok(value) => JsValue::from_serde(&value).map_err(err),
        Err(error) => Err(err(error)),
      }
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
pub struct NamedMessage(actor::NamedMessage);

#[wasm_bindgen(module = "multiaddr")]
extern "C" {
  pub type Multiaddr;

  #[wasm_bindgen(method, getter)]
  fn bytes(this: &Multiaddr) -> Vec<u8>;
}

impl Into<actor::Multiaddr> for Multiaddr {
  fn into(self) -> actor::Multiaddr {
    let addr_bytes = self.bytes();
    actor::Multiaddr::try_from(addr_bytes).unwrap()
  }
}

#[wasm_bindgen(module = "peer-id")]
extern "C" {
  pub type PeerId;

  #[wasm_bindgen(method)]
  fn toBytes(this: &PeerId) -> Vec<u8>;
}

impl Into<actor::PeerId> for PeerId {
  fn into(self) -> actor::PeerId {
    let addr_bytes = self.toBytes();
    actor::PeerId::try_from(addr_bytes).unwrap()
  }
}
