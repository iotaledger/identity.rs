use std::{cell::RefCell, convert::TryFrom, rc::Rc};

use crate::utils::err;
use futures::executor;
use identity::{
  actor::{self, actor_builder::ActorBuilder},
  prelude::*,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
// #[derive(Debug)]
pub struct IdentityActor {
  // TODO: Maybe replace with WasmRefCell
  comm: Rc<RefCell<identity::actor::Actor>>,
}

#[wasm_bindgen]
impl IdentityActor {
  pub fn new() -> Result<IdentityActor, JsValue> {
    let transport = unsafe { libp2p::wasm_ext::ffi::websocket_transport() };
    let transport = libp2p::wasm_ext::ExtTransport::new(transport);
    let comm = executor::block_on(async {
      let comm = ActorBuilder::new().build_with_transport(transport).await.map_err(err);
      comm
    })?;

    Ok(Self {
      comm: Rc::new(RefCell::new(comm)),
    })
  }

  // #[wasm_bindgen(js_name = handleRequests)]
  // pub fn handle_requests(&self) -> Result<Promise, JsValue> {
  //   let comm_clone = self.comm.clone();
  //   let promise = future_to_promise(async move {
  //     comm_clone
  //       .handle_requests()
  //       .await
  //       .map(|_| JsValue::undefined())
  //       .map_err(err)
  //   });

  //   Ok(promise)
  // }

  #[wasm_bindgen(js_name = addPeer)]
  pub fn add_peer(&self, peer_id: PeerId, addr: Multiaddr) -> Result<Promise, JsValue> {
    let addr = addr.into();
    let peer_id = peer_id.into();

    let comm_clone = self.comm.clone();

    let promise = future_to_promise(async move {
      comm_clone.borrow_mut().add_peer(peer_id, addr).await;

      Ok(JsValue::undefined())
    });

    Ok(promise)
  }

  #[wasm_bindgen(js_name = sendRequest)]
  pub fn send_request(&self, peer_id: PeerId) -> Result<Promise, JsValue> {
    let peer_id = peer_id.into();

    let comm_clone = self.comm.clone();

    let promise = future_to_promise(async move {
      // TODO: Most likely unsafe to borrow_mut
      let retval = comm_clone
        .borrow_mut()
        .send_request(peer_id, identity::actor::storage::requests::IdentityList)
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
