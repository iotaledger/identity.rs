// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use futures::executor;
use identity::core::FromJson;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::iota::Client as IotaClient;
use identity::iota::CredentialValidator;
use identity::iota::DocumentDiff;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::TangleRef;
use identity::iota::TangleResolve;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::error::wasm_error;
use crate::tangle::Config;
use crate::tangle::WasmNetwork;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Client {
  pub(crate) client: Rc<IotaClient>,
}

#[wasm_bindgen]
impl Client {
  fn from_client(client: IotaClient) -> Client {
    Self {
      client: Rc::new(client),
    }
  }

  /// Creates a new `Client` with default settings.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<Client, JsValue> {
    Self::from_config(&mut Config::new())
  }

  /// Creates a new `Client` with settings from the given `Config`.
  #[wasm_bindgen(js_name = fromConfig)]
  pub fn from_config(config: &mut Config) -> Result<Client, JsValue> {
    let future = config.take_builder()?.build();
    let output = executor::block_on(future).map_err(wasm_error);

    output.map(Self::from_client)
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  #[wasm_bindgen(js_name = fromNetwork)]
  pub fn from_network(network: WasmNetwork) -> Result<Client, JsValue> {
    let future = IotaClient::from_network(network.into());
    let output = executor::block_on(future).map_err(wasm_error);

    output.map(Self::from_client)
  }

  /// Returns the `Client` Tangle network.
  #[wasm_bindgen]
  pub fn network(&self) -> WasmNetwork {
    self.client.network().into()
  }

  /// Publishes an `IotaDocument` to the Tangle.
  #[wasm_bindgen(js_name = publishDocument)]
  pub fn publish_document(&self, document: &JsValue) -> Result<Promise, JsValue> {
    let document: IotaDocument = document.into_serde().map_err(wasm_error)?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_document(&document)
        .await
        .map_err(wasm_error)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(wasm_error))
    });

    Ok(promise)
  }

  /// Publishes a `DocumentDiff` to the Tangle.
  #[wasm_bindgen(js_name = publishDiff)]
  pub fn publish_diff(&self, message_id: &str, value: &JsValue) -> Result<Promise, JsValue> {
    let diff: DocumentDiff = value.into_serde().map_err(wasm_error)?;
    let message: MessageId = MessageId::from_str(message_id).map_err(wasm_error)?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_diff(&message, &diff)
        .await
        .map_err(wasm_error)
        .and_then(|receipt| JsValue::from_serde(&receipt).map_err(wasm_error))
    });

    Ok(promise)
  }

  #[wasm_bindgen]
  pub fn resolve(&self, did: &str) -> Result<Promise, JsValue> {
    #[derive(Serialize)]
    pub struct DocWrapper<'a> {
      document: &'a IotaDocument,
      #[serde(rename = "messageId")]
      message_id: &'a MessageId,
    }

    let client: Rc<IotaClient> = self.client.clone();
    let did: IotaDID = did.parse().map_err(wasm_error)?;

    let promise: Promise = future_to_promise(async move {
      client.resolve(&did).await.map_err(wasm_error).and_then(|document| {
        let wrapper = DocWrapper {
          document: &document,
          message_id: document.message_id(),
        };

        JsValue::from_serde(&wrapper).map_err(wasm_error)
      })
    });

    Ok(promise)
  }

  /// Validates a credential with the DID Document from the Tangle.
  #[wasm_bindgen(js_name = checkCredential)]
  pub fn check_credential(&self, data: &str) -> Result<Promise, JsValue> {
    let client: Rc<IotaClient> = self.client.clone();
    let data: Credential = Credential::from_json(&data).map_err(wasm_error)?;

    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_credential(data)
        .await
        .map_err(wasm_error)
        .and_then(|output| JsValue::from_serde(&output).map_err(wasm_error))
    });

    Ok(promise)
  }

  /// Validates a presentation with the DID Document from the Tangle.
  #[wasm_bindgen(js_name = checkPresentation)]
  pub fn check_presentation(&self, data: &str) -> Result<Promise, JsValue> {
    let client: Rc<IotaClient> = self.client.clone();
    let data: Presentation = Presentation::from_json(&data).map_err(wasm_error)?;

    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_presentation(data)
        .await
        .map_err(wasm_error)
        .and_then(|output| JsValue::from_serde(&output).map_err(wasm_error))
    });

    Ok(promise)
  }
}
