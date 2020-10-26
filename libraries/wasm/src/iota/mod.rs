use identity_core::did::DID;
use identity_iota::{
    network::{Network, NodeList},
    resolver::TangleResolver,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Iota {}

#[wasm_bindgen]
impl Iota {
    #[wasm_bindgen(js_name = "ResolveDID")]
    pub async fn resolve_did(id: String) -> Result<String, JsValue> {
        console_error_panic_hook::set_once();
        let mut resolver = TangleResolver::new();
        let nodes = vec!["https://nodes.iota.org:443", "https://nodes.thetangle.org:443"];
        let nodelist = NodeList::with_network_and_nodes(Network::Mainnet, nodes);
        resolver.set_nodes(nodelist);
        let did = DID::parse_from_str(id).unwrap();
        let res = resolver.document(&did).await.unwrap();

        Ok(res.data.to_string())
    }
}
