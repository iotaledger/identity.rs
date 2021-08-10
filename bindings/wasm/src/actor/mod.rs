mod interface;
mod multiaddr;
mod peer_id;
mod requests;

use std::{borrow::Cow, cell::RefCell, rc::Rc};

use identity::{
  actor::{
    actor::HandlerBuilder as _HandlerBuilder, actor_builder::ActorBuilder, asyncfn::AsyncFn,
    traits::ActorRequest as IotaActorRequest,
  },
  prelude::*,
};
use js_sys::{Function, Promise};
use libp2p::identity::Keypair;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::error::wasm_error;

use self::{interface::ActorRequest, multiaddr::Multiaddr, peer_id::PeerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSON(serde_json::Value);

impl IotaActorRequest for JSON {
  type Response = JSON;

  fn request_name<'cow>(&self) -> Cow<'cow, str> {
    // SAFETY: Is never called from the actor since this type
    // is never used to call `send_request, but only `send_named_request` instead.
    panic!("`request_name` on `JSON` should not be called");
  }
}

#[wasm_bindgen]
pub struct IdentityActor {
  // TODO: Maybe replace with WasmRefCell
  comm: Rc<RefCell<identity::actor::Actor>>,
}

#[wasm_bindgen]
impl IdentityActor {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<IdentityActor, JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));

    #[allow(unused_unsafe)]
    let transport = unsafe { libp2p::wasm_ext::ffi::websocket_transport() };
    let transport = libp2p::wasm_ext::ExtTransport::new(transport);

    let comm = futures::executor::block_on(async {
      let keys = Keypair::generate_ed25519();

      let executor = |fut| {
        wasm_bindgen_futures::spawn_local(fut);
      };

      let comm = ActorBuilder::new()
        .keys(identity::actor::InitKeypair::IdKeys(keys))
        .build_with_transport_and_executor(transport, executor)
        .await
        .map_err(wasm_error);
      comm
    })?;

    Ok(Self {
      comm: Rc::new(RefCell::new(comm)),
    })
  }

  #[wasm_bindgen(js_name = addPeer)]
  pub fn add_peer(&self, peer_id: PeerId, addr: Multiaddr) -> Result<Promise, JsValue> {
    let addr = addr.into();
    let peer_id = peer_id.into();

    let comm_clone = self.comm.clone();

    let promise = future_to_promise(async move {
      log::info!("Adding peer {} with address {}", peer_id, addr);
      comm_clone.borrow_mut().add_peer(peer_id, addr).await;

      Ok(JsValue::undefined())
    });

    Ok(promise)
  }

  #[wasm_bindgen(js_name = addHandler)]
  pub fn add_handler(&self, object: JsValue) -> HandlerBuilder {
    // self.comm.borrow_mut().add_handler(object)
    todo!()
  }

  #[wasm_bindgen(js_name = sendRequest)]
  pub fn send_request(&self, peer_id: PeerId, request: ActorRequest) -> Result<Promise, JsValue> {
    let peer_id = peer_id.into();

    let comm_clone = self.comm.clone();

    // TODO: It is not guaranteed that this function exists.
    let request_name = request.request_name();

    let js_val: JsValue = request.into();

    // SAFETY: Unsafe can be removed, but rust-analyzer displays a hard error in that case.
    // Only our Rust-defined ActorRequests *should* have this method, so we can use it
    // to differentiate between a native JsValue and one we provided, as they differ in how they are serialized.
    let serialize_property = unsafe { js_sys::Reflect::get(&js_val, &JsValue::from_str("__serialize")) }?;

    let json: serde_json::Value = if serialize_property.is_function() {
      let serialize_method: Function = serialize_property.into();
      // SAFETY: We implement this function in Rust with a `JsValue` return type.
      // If the function is implemented in JS and an exception is thrown, then a panic is ok.
      let res: JsValue = serialize_method.call0(&js_val).unwrap();
      // SAFETY: We can always succesfully parse the result of JSON.stringify into a `serde_json::Value`
      res.into_serde().unwrap()
    } else {
      // SAFETY: We can always succesfully parse the result of JSON.stringify into a `serde_json::Value`
      js_val.into_serde().unwrap()
    };

    let request = JSON(json);

    let promise = future_to_promise(async move {
      log::info!("Sending request {:?} to endpoint {:?}", request.0, request_name);

      // TODO: Most likely unsafe to borrow_mut
      let response = comm_clone
        .borrow_mut()
        .send_named_request(peer_id, &request_name, request)
        .await;

      log::info!("Response: {:?}", response);

      match response {
        Ok(value) => JsValue::from_serde(&value).map_err(wasm_error),
        Err(error) => Err(wasm_error(error)),
      }
    });

    Ok(promise)
  }
}

#[wasm_bindgen]
pub struct HandlerBuilder {
  builder: _HandlerBuilder,
}

#[wasm_bindgen]
impl HandlerBuilder {
  #[wasm_bindgen(js_name = addHandlerMethod)]
  pub fn add_handler_method(self, endpoint: &str, method: js_sys::Function) {
    let fun = AsyncFn::new(|handler: JsValue, request: JSON| {
      let method_clone = method.clone();

      async move {
        // SAFETY: A JSON value can always be successfully converted
        let val = JsValue::from_serde(&request.0).unwrap();
        let promise = method_clone.call1(&handler, &val).unwrap();

        let promise = js_sys::Promise::from(promise);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;

        // TODO: Is this correct?
        match result {
          Ok(js_val) => JSON(js_val.into_serde().unwrap()),
          Err(js_val) => JSON(js_val.into_serde().unwrap()),
        }
      }
    });

    self.builder.add_method("endpoint", fun);
  }
}
