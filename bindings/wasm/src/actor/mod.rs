mod handler;
mod interface;
mod multiaddr;
mod peer_id;
mod requests;

use crate::{
  actor::interface::{ActorRequest, IActorRequest},
  error::WasmResult,
};
use identity::{
  actor::{
    primitives::{NetCommander, RequestMessage, ResponseMessage},
    ActorBuilder, RemoteSendError,
  },
  prelude::*,
};
use js_sys::Promise;
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use self::{handler::Json, multiaddr::Multiaddr, peer_id::PeerId};

#[wasm_bindgen(js_name = "Actor")]
pub struct WasmActor {
  // TODO: Maybe replace with WasmRefCell
  actor: Rc<RefCell<identity::actor::Actor>>,
}

#[wasm_bindgen]
impl WasmActor {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<WasmActor, JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug).module_prefix("identity"));

    #[allow(unused_unsafe)]
    let transport = unsafe { libp2p::wasm_ext::ffi::websocket_transport() };
    let transport = libp2p::wasm_ext::ExtTransport::new(transport);

    let actor = futures::executor::block_on(async {
      let actor = ActorBuilder::new().build_with_transport(transport).await.expect("TODO");
      actor
    });

    Ok(Self {
      actor: Rc::new(RefCell::new(actor)),
    })
  }

  #[wasm_bindgen(js_name = addAddress)]
  pub fn add_address(&self, peer_id: PeerId, addr: Multiaddr) -> Result<Promise, JsValue> {
    let addr = addr.into();
    let peer_id = peer_id.into();

    let comm_clone = self.actor.clone();

    let promise = future_to_promise(async move {
      log::info!("Adding peer {} with address {}", peer_id, addr);
      comm_clone.borrow_mut().add_address(peer_id, addr).await.expect("TODO");

      Ok(JsValue::undefined())
    });

    Ok(promise)
  }

  #[wasm_bindgen(js_name = sendRequest)]
  pub fn send_request(&self, peer_id: PeerId, request: &IActorRequest) -> Result<Promise, JsValue> {
    let peer_id = peer_id.into();

    let request: ActorRequest = request.into_serde().wasm_result()?;



    let request_vec = serde_json::to_vec(&request.request).expect("TODO: Add constructors for serialization errors");

    // TODO: Endpoint validation.
    let message = RequestMessage::new(request.endpoint, identity::actor::RequestMode::Synchronous, request_vec)
      .expect("TODO RequestMessage");

    let mut commander: NetCommander = self.actor.borrow().commander.clone();

    let promise = future_to_promise(async move {
      let response: ResponseMessage = commander
        .send_request(peer_id, message)
        .await
        .expect("TODO ResponseMessage");

      let response: Vec<u8> = serde_json::from_slice::<Result<Vec<u8>, RemoteSendError>>(&response.0)
        .expect("TODO deserialization")
        .expect("TODO RemoteSendError");

      let value = serde_json::from_slice::<Json>(&response).expect("TODO serialize to JSON");

      Ok(JsValue::from_serde(&value).expect("TODO convert to JsValue"))
    });

    Ok(promise)
  }

  #[wasm_bindgen]
  pub fn shutdown(&self) -> Result<Promise, JsValue> {
    let mut commander = self.actor.borrow().commander.clone();

    let promise = future_to_promise(async move {
      
      commander.shutdown().await.expect("TODO");

      Ok(JsValue::undefined())
    });

    Ok(promise)
  }
}

// #[wasm_bindgen]
// pub struct HandlerBuilder {
//   builder: _HandlerBuilder,
// }

// #[wasm_bindgen]
// impl HandlerBuilder {
//   #[wasm_bindgen(js_name = addHandlerMethod)]
//   pub fn add_handler_method(self, endpoint: &str, method: js_sys::Function) {
//     let fun = AsyncFn::new(|handler: JsValue, request: JSON| {
//       let method_clone = method.clone();

//       async move {
//         // SAFETY: A JSON value can always be successfully converted
//         let val = JsValue::from_serde(&request.0).unwrap();
//         let promise = method_clone.call1(&handler, &val).unwrap();

//         let promise = js_sys::Promise::from(promise);
//         let result = wasm_bindgen_futures::JsFuture::from(promise).await;

//         // TODO: Is this correct?
//         match result {
//           Ok(js_val) => JSON(js_val.into_serde().unwrap()),
//           Err(js_val) => JSON(js_val.into_serde().unwrap()),
//         }
//       }
//     });

//     self.builder.add_method("endpoint", fun);
//   }
// }
