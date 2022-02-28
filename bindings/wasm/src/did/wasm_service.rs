// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::ServiceEndpoint;
use identity::iota::IotaService;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::did::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;

/// A DID Document Service used to enable trusted interactions associated
/// with a DID subject.
///
/// See: https://www.w3.org/TR/did-core/#services
#[wasm_bindgen(js_name = Service, inspectable)]
pub struct WasmService(pub(crate) IotaService);

#[wasm_bindgen(js_class = Service)]
impl WasmService {
  #[wasm_bindgen(constructor)]
  pub fn new(service: IService) -> Result<WasmService> {
    service.into_serde::<IotaService>().map(WasmService::from).wasm_result()
  }

  /// Returns a copy of the `Service` id.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `Service` type.
  #[wasm_bindgen(getter = type)]
  pub fn type_(&self) -> String {
    self.0.type_().to_owned()
  }

  /// Returns a copy of the `Service` endpoint.
  #[wasm_bindgen(getter = serviceEndpoint)]
  pub fn service_endpoint(&self) -> UServiceEndpoint {
    match self.0.service_endpoint() {
      // string
      ServiceEndpoint::One(url) => JsValue::from_str(url.as_str()).unchecked_into::<UServiceEndpoint>(),
      // [string]
      ServiceEndpoint::Set(set) => set
        .iter()
        .map(|url| JsValue::from_str(url.as_str()))
        .collect::<js_sys::Array>()
        .unchecked_into::<UServiceEndpoint>(),
      // Map<string, [string]>
      ServiceEndpoint::Map(map) => {
        let js_map: js_sys::Map = js_sys::Map::new();
        for (key, urls) in map.into_iter() {
          js_map.set(
            &JsValue::from_str(key.as_str()),
            &urls
              .iter()
              .map(|url| JsValue::from_str(url.as_str()))
              .collect::<js_sys::Array>(),
          );
        }
        js_map.unchecked_into::<UServiceEndpoint>()
      }
    }
  }

  /// Returns a copy of the custom `Service` properties.
  #[wasm_bindgen(js_name = properties)]
  pub fn properties(&self) -> Result<MapStringAny> {
    let js_map: js_sys::Map = js_sys::Map::new();
    for (key, value) in self.0.properties().iter() {
      js_map.set(
        &JsValue::from_str(key.as_str()),
        &JsValue::from_serde(&value).wasm_result()?,
      );
    }
    Ok(js_map.unchecked_into::<MapStringAny>())
  }

  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Service` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmService> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl From<IotaService> for WasmService {
  fn from(service: IotaService) -> Self {
    Self(service)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "string | string[] | Map<string, string[]>")]
  pub type UServiceEndpoint;

  #[wasm_bindgen(typescript_type = "Map<string, any>")]
  pub type MapStringAny;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IService")]
  pub type IService;
}

#[wasm_bindgen(typescript_custom_section)]
const I_SERVICE: &'static str = r#"
/** Holds options to create a new `Service`. */
interface IService {
    /** Identifier of the service.
    *
    * Must be a valid DIDUrl with a fragment.
    */
    readonly id: DIDUrl | string;

    /** Type of service.
    *
    * E.g. "LinkedDomains" or "DIDCommMessaging".
    */
    readonly type: string;

    /** A URL, set of URLs, or map of URL sets.
    *
    * NOTE: throws an error if any entry is not a valid URL string. List entries must be unique.
    */
    readonly serviceEndpoint: string | string[] | Map<string, string[]>;

    /** Additional custom properties to embed in the service.
    *
    * WARNING: entries may overwrite existing fields and result in invalid documents.
    */
    readonly properties?: Map<string, any>;
}"#;
