use identity_core::{did::DID as CoreDID, utils::decode_b58};
use identity_iota::did::IotaDID;
use wasm_bindgen::prelude::*;

use crate::js_err;

#[derive(Debug, Deserialize)]
pub struct DIDParams {
    key: String,
    network: Option<String>,
    shard: Option<String>,
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DID(pub(crate) IotaDID);

#[wasm_bindgen]
impl DID {
    fn create(key: impl AsRef<str>, network: Option<&str>, shard: Option<&str>) -> Result<Self, JsValue> {
        let public: Vec<u8> = decode_b58(key.as_ref()).map_err(js_err)?;

        IotaDID::with_network_and_shard(&public, network, shard)
            .map_err(js_err)
            .map(Self)
    }

    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> Result<DID, JsValue> {
        if params.is_object() {
            let params: DIDParams = params.into_serde().map_err(js_err)?;

            Self::create(params.key, params.network.as_deref(), params.shard.as_deref())
        } else if let Some(key) = params.as_string() {
            Self::create(key, None, None)
        } else {
            panic!("Invalid Arguments for `new DID(..)`");
        }
    }

    #[wasm_bindgen]
    pub fn parse(did_string: String) -> Result<DID, JsValue> {
        Ok(Self(IotaDID::parse(did_string).map_err(js_err)?))
    }

    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.0.network().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn shard(&self) -> JsValue {
        if let Some(shard) = self.0.shard() {
            shard.into()
        } else {
            JsValue::NULL
        }
    }

    #[wasm_bindgen(getter)]
    pub fn method_id(&self) -> String {
        self.0.method_id().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.0.create_address_hash()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl DID {
    pub fn parse_from_did(did: CoreDID) -> Result<DID, JsValue> {
        IotaDID::try_from_did(did).map_err(js_err).map(Self)
    }
}
