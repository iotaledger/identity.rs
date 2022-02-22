// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::ops::Deref;
use std::rc::Rc;

use futures::executor;
use identity::core::FromJson;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::iota::Client;
use identity::iota::CredentialValidator;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleResolve;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::chain::DiffChainHistory;
use crate::chain::PromiseDiffChainHistory;
use crate::chain::PromiseDocumentHistory;
use crate::chain::WasmDocumentHistory;
use crate::did::PromiseResolvedDocument;
use crate::did::UWasmDID;
use crate::did::WasmDiffMessage;
use crate::did::WasmDocument;
use crate::did::WasmResolvedDocument;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::Config;
use crate::tangle::PromiseReceipt;
use crate::tangle::WasmNetwork;
use crate::tangle::WasmReceipt;

#[wasm_bindgen(js_name = Client)]
#[derive(Debug)]
pub struct WasmClient {
  pub(crate) client: Rc<Client>,
}

#[wasm_bindgen(js_class = Client)]
impl WasmClient {
  fn from_client(client: Client) -> WasmClient {
    Self {
      client: Rc::new(client),
    }
  }

  /// Creates a new `Client` with default settings.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<WasmClient> {
    Self::from_config(&mut Config::new())
  }

  /// Creates a new `Client` with settings from the given `Config`.
  #[wasm_bindgen(js_name = fromConfig)]
  pub fn from_config(config: &mut Config) -> Result<WasmClient> {
    let future = config.take_builder()?.build();
    let output = executor::block_on(future).wasm_result();

    output.map(Self::from_client)
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  #[wasm_bindgen(js_name = fromNetwork)]
  pub fn from_network(network: WasmNetwork) -> Result<WasmClient> {
    let future = Client::from_network(network.into());
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
  pub fn publish_document(&self, document: &WasmDocument) -> Result<PromiseReceipt> {
    let document: IotaDocument = document.0.clone();
    let client: Rc<Client> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_document(&document)
        .await
        .map(WasmReceipt)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseReceipt>())
  }

  /// Publishes a `DiffMessage` to the Tangle.
  #[wasm_bindgen(js_name = publishDiff)]
  pub fn publish_diff(&self, message_id: &str, diff: WasmDiffMessage) -> Result<PromiseReceipt> {
    let message: MessageId = MessageId::from_str(message_id).wasm_result()?;
    let client: Rc<Client> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_diff(&message, diff.deref())
        .await
        .map(WasmReceipt)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseReceipt>())
  }

  /// Publishes arbitrary JSON data to the specified index on the Tangle.
  #[wasm_bindgen(js_name = publishJSON)]
  pub fn publish_json(&self, index: String, data: &JsValue) -> Result<PromiseReceipt> {
    let value: serde_json::Value = data.into_serde().wasm_result()?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .publish_json(&index, &value)
        .await
        .map(WasmReceipt)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseReceipt>())
  }

  /// Publishes arbitrary JSON data to the specified index on the Tangle.
  /// Retries (promotes or reattaches) the message until it’s included (referenced by a milestone).
  /// Default interval is 5 seconds and max attempts is 40.
  #[wasm_bindgen(js_name = publishJsonWithRetry)]
  pub fn publish_json_with_retry(
    &self,
    index: String,
    data: &JsValue,
    interval: Option<u32>,
    max_attempts: Option<u32>,
  ) -> Result<Promise> {
    let value: serde_json::Value = data.into_serde().wasm_result()?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .publish_json_with_retry(
          &index,
          &value,
          interval.map(|interval| interval as u64),
          max_attempts.map(|max_attempts| max_attempts as u64),
        )
        .await
        .wasm_result()
        .and_then(|receipt| JsValue::from_serde(&receipt).wasm_result())
    });

    Ok(promise)
  }

  /// Fetch the DID document specified by the given `DID`.
  #[wasm_bindgen]
  pub fn resolve(&self, did: UWasmDID) -> Result<PromiseResolvedDocument> {
    let did: IotaDID = IotaDID::try_from(did)?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .resolve(&did)
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseResolvedDocument>())
  }

  /// Returns the message history of the given DID.
  #[wasm_bindgen(js_name = resolveHistory)]
  pub fn resolve_history(&self, did: UWasmDID) -> Result<PromiseDocumentHistory> {
    let did: IotaDID = IotaDID::try_from(did)?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .resolve_history(&did)
        .await
        .map(WasmDocumentHistory::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseDocumentHistory>())
  }

  /// Returns the `DiffChainHistory` of a diff chain starting from a document on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the tangle and have a valid message id and
  /// capability invocation method.
  #[wasm_bindgen(js_name = resolveDiffHistory)]
  pub fn resolve_diff_history(&self, document: &WasmResolvedDocument) -> Result<PromiseDiffChainHistory> {
    let resolved_document: ResolvedIotaDocument = document.0.clone();

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      client
        .resolve_diff_history(&resolved_document)
        .await
        .map(DiffChainHistory::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseDiffChainHistory>())
  }

  /// Validates a credential with the DID Document from the Tangle.
  // TODO: move out of client to dedicated verifier
  #[wasm_bindgen(js_name = checkCredential)]
  pub fn check_credential(&self, data: &str, options: WasmVerifierOptions) -> Result<Promise> {
    let data: Credential = Credential::from_json(&data).wasm_result()?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_credential(data, options.0)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }

  /// Validates a presentation with the DID Document from the Tangle.
  // TODO: move out of client to dedicated verifier
  #[wasm_bindgen(js_name = checkPresentation)]
  pub fn check_presentation(&self, data: &str, options: WasmVerifierOptions) -> Result<Promise> {
    let data: Presentation = Presentation::from_json(&data).wasm_result()?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise = future_to_promise(async move {
      CredentialValidator::new(&*client)
        .validate_presentation(data, options.0)
        .await
        .wasm_result()
        .and_then(|output| JsValue::from_serde(&output).wasm_result())
    });

    Ok(promise)
  }
}

impl From<Rc<Client>> for WasmClient {
  fn from(client: Rc<Client>) -> Self {
    Self { client }
  }
}
