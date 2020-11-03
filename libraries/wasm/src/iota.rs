use identity_iota::{
    client::{Client, ClientBuilder, TransactionPrinter},
    did::IotaDID,
    network::Network,
    vc::{CredentialValidation, CredentialValidator},
};
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

#[wasm_bindgen]
pub async fn publish(doc: JsValue, params: JsValue) -> Result<JsValue, JsValue> {
    client(params)?
        .create_document(&doc.into_serde().map_err(js_err)?)
        .send()
        .await
        .map_err(js_err)
        .map(|response| TransactionPrinter::hash(&response.tail).to_string())
        .map(Into::into)
}

#[wasm_bindgen]
pub async fn resolve(did: String, params: JsValue) -> Result<JsValue, JsValue> {
    client(params)?
        .read_document(&IotaDID::parse(did).map_err(js_err)?)
        .send()
        .await
        .map_err(js_err)
        .and_then(|response| JsValue::from_serde(&response).map_err(js_err))
}

#[wasm_bindgen(js_name = checkCredential)]
pub async fn check_credential(data: String, params: JsValue) -> Result<bool, JsValue> {
    let client = client(params)?;
    let validator: CredentialValidator<'_> = CredentialValidator::new(&client);
    let validation: CredentialValidation = validator.check(&data).await.map_err(js_err)?;
    Ok(validation.verified)
}
