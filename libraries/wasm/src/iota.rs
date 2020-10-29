use identity_core::resolver::MetaDocument;
use identity_iota::{
    did::IotaDID,
    io::TangleWriter,
    network::{Network, NodeList},
    resolver::TangleResolver,
    utils::encode_trits,
};
use wasm_bindgen::prelude::*;

use crate::{doc::Doc, js_err};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[wasm_bindgen]
pub async fn publish(doc: Doc, node: String) -> Result<JsValue, JsValue> {
    let nodelist: NodeList = NodeList::with_network_and_nodes(Network::Mainnet, vec![node]);
    let writer: TangleWriter = TangleWriter::new(&nodelist).map_err(js_err)?;

    let hash: _ = writer.write_document(&doc.0).await.map_err(js_err)?;
    let hash: String = encode_trits(hash.as_trits());

    Ok(hash.into())
}

#[wasm_bindgen]
pub async fn resolve(did: String, node: String) -> Result<JsValue, JsValue> {
    let nodelist: NodeList = NodeList::with_network_and_nodes(Network::Mainnet, vec![node]);
    let resolver: TangleResolver = TangleResolver::with_nodes(nodelist);

    let did: IotaDID = did.parse().map_err(js_err)?;
    let res: MetaDocument = resolver.document(&did).await.map_err(js_err)?;

    JsValue::from_serde(&res).map_err(js_err)
}
