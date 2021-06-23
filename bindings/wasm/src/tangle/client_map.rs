// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use futures::executor;
use identity::iota::Client as IotaClient;
use identity::iota::ClientMap as IotaClientMap;
use identity::iota::DocumentDiff;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::TangleRef;
use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::tangle::Config;
use crate::tangle::WasmNetwork;
use crate::utils::err;

type Shared<T> = Rc<RefCell<T>>;

#[wasm_bindgen]
#[derive(Debug)]
pub struct ClientMap {
  client: Shared<IotaClientMap>,
}

#[wasm_bindgen]
impl ClientMap {
  fn from_client(client: IotaClientMap) -> ClientMap {
    Self {
      client: Rc::new(RefCell::new(client)),
    }
  }

  #[wasm_bindgen(constructor)]
  pub fn new() -> ClientMap {
    Self::from_client(IotaClientMap::new())
  }

  pub fn from_config(config: &mut Config) -> Result<ClientMap, JsValue> {
    let future = config.take_builder()?.build();
    let output = executor::block_on(future).map_err(err)?;

    Ok(Self::from_client(IotaClientMap::from_client(output)))
  }

  pub fn from_network(network: WasmNetwork) -> Result<ClientMap, JsValue> {
    let future = IotaClient::from_network(network.into());
    let output = executor::block_on(future).map_err(err)?;

    Ok(Self::from_client(IotaClientMap::from_client(output)))
  }

  pub fn publish_document(&self, document: &JsValue) -> Result<Promise, JsValue> {
    let document: IotaDocument = document.into_serde().map_err(err)?;
    let client: Shared<IotaClientMap> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .borrow()
        .publish_document(&document)
        .await
        .map_err(err)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(err))
    });

    Ok(promise)
  }

  pub fn publish_diff(&self, message_id: &str, value: &JsValue) -> Result<Promise, JsValue> {
    let diff: DocumentDiff = value.into_serde().map_err(err)?;
    let message: MessageId = MessageId::from_str(message_id).map_err(err)?;
    let client: Shared<IotaClientMap> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .borrow()
        .publish_diff(&message, &diff)
        .await
        .map_err(err)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(err))
    });

    Ok(promise)
  }

  pub fn resolve(&self, did: &str) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    pub struct DocWrapper<'a> {
      document: &'a IotaDocument,
      #[serde(rename = "messageId")]
      message_id: &'a MessageId,
    }

    let client: Shared<IotaClientMap> = self.client.clone();
    let did: IotaDID = did.parse().map_err(err)?;

    let promise: Promise = future_to_promise(async move {
      client.borrow().resolve(&did).await.map_err(err).and_then(|document| {
        let wrapper = DocWrapper {
          document: &document,
          message_id: document.message_id(),
        };

        JsValue::from_serde(&wrapper).map_err(err)
      })
    });

    Ok(promise)
  }
}

impl Default for ClientMap {
  fn default() -> Self {
    Self::new()
  }
}
