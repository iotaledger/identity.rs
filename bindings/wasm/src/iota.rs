// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Object;
use identity::iota::Client;
use identity::iota::ClientBuilder;
use identity::iota::CredentialValidator;
use identity::iota::IotaDID;
use identity::iota::Network;
use identity::iota::TxnPrinter;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::js_err;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn error(s: &str);
}

#[derive(Debug, Deserialize)]
pub enum ClientNode {
  #[serde(rename = "node")]
  Node(String),
  #[serde(rename = "nodes")]
  List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct ClientParams {
  network: Option<String>,
  #[serde(flatten)]
  node: ClientNode,
}

impl ClientParams {
  pub fn build(self, mut builder: ClientBuilder) -> ClientBuilder {
    builder = Self::build_node(builder, self.node);
    builder = Self::build_network(builder, self.network);
    builder
  }

  fn build_node(builder: ClientBuilder, node: ClientNode) -> ClientBuilder {
    match node {
      ClientNode::Node(node) => builder.node(node),
      ClientNode::List(node) => builder.nodes(node),
    }
  }

  fn build_network(builder: ClientBuilder, network: Option<String>) -> ClientBuilder {
    match network.as_deref() {
      Some("main") | Some("mainnet") => builder.network(Network::Mainnet),
      Some("com") | Some("comnet") => builder.network(Network::Comnet),
      Some("dev") | Some("devnet") => builder.network(Network::Devnet),
      Some(_) | None => builder.network(Network::Mainnet),
    }
  }
}

fn client(params: JsValue) -> Result<Client, JsValue> {
  if params.is_object() {
    let params: ClientParams = params.into_serde().map_err(js_err)?;
    params.build(ClientBuilder::new()).build().map_err(js_err)
  } else if let Some(node) = params.as_string() {
    ClientBuilder::new().node(node).build().map_err(js_err)
  } else {
    Err("Invalid Arguments for `new Client(..)`".into())
  }
}

/// Publishes a DID Document to the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen]
pub async fn publish(doc: JsValue, params: JsValue) -> Result<JsValue, JsValue> {
  client(params)?
    .publish_document(&doc.into_serde().map_err(js_err)?)
    .await
    .map_err(js_err)
    .map(|response| TxnPrinter::hash(&response).to_string())
    .map(Into::into)
}

/// Resolves the latest DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen]
pub async fn resolve(did: String, params: JsValue) -> Result<JsValue, JsValue> {
  client(params)?
    .read_document(&IotaDID::parse(did).map_err(js_err)?)
    .await
    .map_err(js_err)
    .and_then(|response| JsValue::from_serde(&response).map_err(js_err))
}

/// Validates a credential with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen(js_name = checkCredential)]
pub async fn check_credential(data: String, params: JsValue) -> Result<JsValue, JsValue> {
  CredentialValidator::new(&client(params)?)
    .check::<Object>(&data)
    .await
    .map_err(js_err)
    .and_then(|validation| JsValue::from_serde(&validation).map_err(js_err))
}

/// Validates a presentation with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }
#[wasm_bindgen(js_name = checkPresentation)]
pub async fn check_presentation(data: String, params: JsValue) -> Result<JsValue, JsValue> {
  CredentialValidator::new(&client(params)?)
    .check_presentation::<Object, Object>(&data)
    .await
    .map_err(js_err)
    .and_then(|validation| JsValue::from_serde(&validation).map_err(js_err))
}
