// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::ops::Deref;
use std::rc::Rc;

use futures::executor;
use identity::core::FromJson;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::iota::Client as IotaClient;
use identity::iota::CredentialValidator;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::TangleResolve;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::chain::DiffChainHistory;
use crate::chain::WasmDocumentHistory;
use crate::did::WasmDocument;
use crate::did::WasmDocumentDiff;
use crate::error::Result;
use crate::error::WasmResult;
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
  pub fn new() -> Result<Client> {
    Self::from_config(&mut Config::new())
  }

  /// Creates a new `Client` with settings from the given `Config`.
  #[wasm_bindgen(js_name = fromConfig)]
  pub fn from_config(config: &mut Config) -> Result<Client> {
    let future = config.take_builder()?.build();
    let output = executor::block_on(future).wasm_result();

    output.map(Self::from_client)
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  #[wasm_bindgen(js_name = fromNetwork)]
  pub fn from_network(network: WasmNetwork) -> Result<Client> {
    let future = IotaClient::from_network(network.into());
    let output = executor::block_on(future).wasm_result();

    output.map(Self::from_client)
  }

  /// Returns the `Client` Tangle network.
  #[wasm_bindgen]
  pub fn network(&self) -> WasmNetwork {
    self.client.network().into()
  }

  /// Publishes an `IotaDocument` to the Tangle.
  #[wasm_bindgen(js_name = publishDocument)]
  pub fn publish_document(&self, document: &JsValue) -> Result<Promise> {
    let document: IotaDocument = document.into_serde().wasm_result()?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_document(&document)
        .await
        .wasm_result()
        .and_then(|receipt| JsValue::from_serde(&receipt).wasm_result())
    });

    Ok(promise)
  }

  /// Publishes a `DocumentDiff` to the Tangle.
  #[wasm_bindgen(js_name = publishDiff)]
  pub fn publish_diff(&self, message_id: &str, diff: WasmDocumentDiff) -> Result<Promise> {
    let message: MessageId = MessageId::from_str(message_id).wasm_result()?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_diff(&message, diff.deref())
        .await
        .wasm_result()
        .and_then(|receipt| JsValue::from_serde(&receipt).wasm_result())
    });

    Ok(promise)
  }

  /// Publishes arbitrary JSON data to the specified index on the Tangle.
  #[wasm_bindgen(js_name = publishJSON)]
  pub fn publish_json(&self, index: &str, data: &JsValue) -> Result<Promise> {
    let client: Rc<IotaClient> = self.client.clone();

    let index = index.to_owned();
    let value: serde_json::Value = data.into_serde().wasm_result()?;
    let promise: Promise = future_to_promise(async move {
      client
        .publish_json(&index, &value)
        .await
        .wasm_result()
        .and_then(|receipt| JsValue::from_serde(&receipt).wasm_result())
    });

    Ok(promise)
  }

  #[wasm_bindgen]
  pub fn resolve(&self, did: &str) -> Result<Promise> {
    #[derive(Serialize)]
    pub struct DocWrapper<'a> {
      document: &'a IotaDocument,
      #[serde(rename = "messageId")]
      message_id: &'a MessageId,
    }

    let client: Rc<IotaClient> = self.client.clone();
    let did: IotaDID = did.parse().wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      client
        .resolve(&did)
        .await
        .map(WasmDocument::from)
        .map(JsValue::from)
        .wasm_result()
    });

    Ok(promise)
  }

  /// Returns the message history of the given DID.
  #[wasm_bindgen(js_name = resolveHistory)]
  pub fn resolve_history(&self, did: &str) -> Result<Promise> {
    let did: IotaDID = did.parse().wasm_result()?;
    let client: Rc<IotaClient> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .resolve_history(&did)
        .await
        .map(WasmDocumentHistory::from)
        .map(JsValue::from)
        .wasm_result()
    });

    Ok(promise)
  }

  /// Returns the [`DiffChainHistory`] of a diff chain starting from a document on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the tangle and have a valid message id and
  /// authentication method.
  #[wasm_bindgen(js_name = resolveDiffHistory)]
  pub fn resolve_diff_history(&self, document: &WasmDocument) -> Result<Promise> {
    let client: Rc<IotaClient> = self.client.clone();
    let iota_document: IotaDocument = document.0.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .resolve_diff_history(&iota_document)
        .await
        .map(DiffChainHistory::from)
        .map(JsValue::from)
        .wasm_result()
    });

    Ok(promise)
  }

  /// Validates a credential with the DID Document from the Tangle.
  #[wasm_bindgen(js_name = checkCredential)]
  pub fn check_credential(&self, data: &str) -> Result<Promise> {
    let client: Rc<IotaClient> = self.client.clone();
    let data: Credential = Credential::from_json(&data).wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_credential(data)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }

  /// Validates a presentation with the DID Document from the Tangle.
  #[wasm_bindgen(js_name = checkPresentation)]
  pub fn check_presentation(&self, data: &str) -> Result<Promise> {
    let client: Rc<IotaClient> = self.client.clone();
    let data: Presentation = Presentation::from_json(&data).wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_presentation(data)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }
}
