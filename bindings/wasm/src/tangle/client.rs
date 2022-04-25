// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::rc::Rc;

use futures::executor;
use identity::iota::Client;
use identity::iota::ClientBuilder;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleResolve;
use identity::iota_core::DiffMessage;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDocument;
use identity::iota_core::MessageId;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::chain::DiffChainHistory;
use crate::chain::PromiseDiffChainHistory;
use crate::chain::PromiseDocumentHistory;
use crate::chain::WasmDocumentHistory;
use crate::common::PromiseBool;
use crate::did::PromiseResolvedDocument;
use crate::did::UWasmDID;
use crate::did::WasmDiffMessage;
use crate::did::WasmDocument;
use crate::did::WasmResolvedDocument;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::IClientConfig;
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
  /// Creates a new `Client` with default settings.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<WasmClient> {
    // TODO: is there a way to avoid blocking? Async constructors are not supported.
    executor::block_on(Client::new()).map(Self::from).wasm_result()
  }

  /// Creates a new `Client` with the given settings.
  #[wasm_bindgen(js_name = fromConfig)]
  pub fn from_config(config: IClientConfig) -> Result<PromiseClient> {
    let builder: ClientBuilder = ClientBuilder::try_from(config)?;
    let promise: Promise =
      future_to_promise(async move { builder.build().await.map(Self::from).map(Into::into).wasm_result() });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseClient>())
  }

  /// Returns the `Client` Tangle network.
  #[wasm_bindgen]
  pub fn network(&self) -> WasmNetwork {
    self.client.network().into()
  }

  /// Publishes a {@link Document} to the Tangle.
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
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = publishDiff)]
  pub fn publish_diff(&self, message_id: &str, diff: &WasmDiffMessage) -> Result<PromiseReceipt> {
    let message: MessageId = MessageId::from_str(message_id)
      .map_err(identity::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    let diff: DiffMessage = diff.0.clone();
    let client: Rc<Client> = self.client.clone();

    let promise: Promise = future_to_promise(async move {
      client
        .publish_diff(&message, &diff)
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

  /// Checks if a message is confirmed by a milestone.
  #[wasm_bindgen(js_name = isMessageIncluded)]
  #[allow(non_snake_case)]
  pub fn is_message_included(&self, messageId: &str) -> Result<PromiseBool> {
    let message: MessageId = MessageId::from_str(messageId)
      .map_err(identity::iota_core::Error::InvalidMessage)
      .wasm_result()?;

    let client: Rc<Client> = self.client.clone();
    let promise: Promise =
      future_to_promise(async move { client.is_message_included(&message).await.map(Into::into).wasm_result() });
    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseBool>())
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
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
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
}

impl From<Rc<Client>> for WasmClient {
  fn from(client: Rc<Client>) -> Self {
    Self { client }
  }
}

impl From<Client> for WasmClient {
  fn from(client: Client) -> Self {
    Self {
      client: Rc::new(client),
    }
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Client>")]
  pub type PromiseClient;
}
