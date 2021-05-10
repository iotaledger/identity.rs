// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::Client;
use identity::iota::ClientBuilder;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::iota::Network;
use identity::iota::PresentationValidation;
use wasm_bindgen::prelude::*;

use crate::utils::err;
use crate::wasm_did::WasmDID;
use crate::wasm_document::WasmDocument;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ClientNode {
  None,
  Node(String),
  List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct ClientParams {
  network: Option<String>,
  #[serde(alias = "nodes")]
  node: Option<ClientNode>,
}

async fn client(params: JsValue) -> Result<Client, JsValue> {
  if params.is_object() {
    let params: ClientParams = params.into_serde().map_err(err)?;

    let network: Network = params.network.as_deref().map(Network::from_name).unwrap_or_default();

    let builder: ClientBuilder = match params.node.unwrap_or(ClientNode::None) {
      ClientNode::Node(node) => ClientBuilder::new().node(node),
      ClientNode::List(node) => ClientBuilder::new().nodes(node),
      ClientNode::None => ClientBuilder::new(),
    };

    builder.network(network).build().await.map_err(err)
  } else if let Some(node) = params.as_string() {
    ClientBuilder::new().node(node).build().await.map_err(err)
  } else if params.is_undefined() {
    Client::new().await.map_err(err)
  } else {
    Err("Invalid Arguments for `new Client(..)`".into())
  }
}

/// Publishes a DID Document to the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen]
pub async fn publish(document: JsValue, params: JsValue) -> Result<JsValue, JsValue> {
  let client: Client = client(params).await?;
  let document: WasmDocument = WasmDocument::from_json(&document)?;

  client
    .publish_document(&document.0)
    .await
    .map_err(err)
    .map(|message_id| message_id.to_string())
    .map(Into::into)
}

/// Resolves the latest DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen]
pub async fn resolve(did: String, params: JsValue) -> Result<JsValue, JsValue> {
  let client: Client = client(params).await?;
  let did: WasmDID = WasmDID::parse(&did)?;

  client
    .read_document(&did.0)
    .await
    .map_err(err)
    .and_then(|response| JsValue::from_serde(&response).map_err(err))
}

/// Validates a credential with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen(js_name = checkCredential)]
pub async fn check_credential(data: String, params: JsValue) -> Result<JsValue, JsValue> {
  let client: Client = client(params).await?;

  let status: CredentialValidation = CredentialValidator::new(&client).check(&data).await.map_err(err)?;

  JsValue::from_serde(&status).map_err(err)
}

/// Validates a presentation with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen(js_name = checkPresentation)]
pub async fn check_presentation(data: String, params: JsValue) -> Result<JsValue, JsValue> {
  let client: Client = client(params).await?;

  let status: PresentationValidation = CredentialValidator::new(&client)
    .check_presentation(&data)
    .await
    .map_err(err)?;

  JsValue::from_serde(&status).map_err(err)
}
