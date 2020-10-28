use identity_iota::{
    did::IotaDID,
    network::{Network, NodeList},
    resolver::TangleResolver,
};
use wasm_bindgen::prelude::*;
#[allow(dead_code)]
#[wasm_bindgen(js_name = "ResolveDID")]
pub async fn resolve_did(id: String, node: String) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    let mut resolver = TangleResolver::new();
    let nodelist = NodeList::with_network_and_nodes(Network::Mainnet, vec![node]);
    resolver.set_nodes(nodelist);
    let did = IotaDID::parse(id).unwrap();
    let res = resolver.document(&did).await.unwrap();

    Ok(res.data.to_string())
}
