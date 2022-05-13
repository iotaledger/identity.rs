// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::deserialize_map_or_any;
use crate::common::MapStringAny;
use identity::did::ServiceEndpoint;
use identity::iota_core::IotaDIDUrl;
use identity::iota_core::IotaService;
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
    let id: IotaDIDUrl = service.id().into_serde().wasm_result()?;
    let type_: String = service.type_();
    let service_endpoint: ServiceEndpoint = deserialize_map_or_any(&service.service_endpoint())?;
    let properties: Option<identity::core::Object> = deserialize_map_or_any(&service.properties())?;

    IotaService::builder(properties.unwrap_or_default())
      .id(id)
      .type_(type_)
      .service_endpoint(service_endpoint)
      .build()
      .map(WasmService::from)
      .wasm_result()
  }

  /// Returns a copy of the `Service` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `Service` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> String {
    self.0.type_().to_owned()
  }

  /// Returns a copy of the `Service` endpoint.
  #[wasm_bindgen(js_name = serviceEndpoint)]
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

  /// Returns a copy of the custom properties on the `Service`.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
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

impl_wasm_clone!(WasmService, Service);

impl From<IotaService> for WasmService {
  fn from(service: IotaService) -> Self {
    Self(service)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "string | string[] | Map<string, string[]>")]
  pub type UServiceEndpoint;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IService")]
  pub type IService;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &IService) -> JsValue;

  #[wasm_bindgen(method, getter, js_name = type)]
  pub fn type_(this: &IService) -> String;

  #[wasm_bindgen(method, getter, js_name = serviceEndpoint)]
  pub fn service_endpoint(this: &IService) -> JsValue;

  #[wasm_bindgen(method, getter)]
  pub fn properties(this: &IService) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const I_SERVICE: &'static str = r#"
/**
 * Holds options to create a new `Service`.
 */
interface IService {
    /**
     * Identifier of the service.
     *
     * Must be a valid DIDUrl with a fragment.
     */
    readonly id: DIDUrl | string;

    /**
     * Type of service.
     *
     * E.g. "LinkedDomains" or "DIDCommMessaging".
     */
    readonly type: string;

    /**
     * A URL, set of URLs, or map of URL sets.
     *
     * NOTE: throws an error if any entry is not a valid URL string. List entries must be unique.
     */
    readonly serviceEndpoint: string | string[] | Map<string, string[]> | Record<string, string[]>;

    /**
     * Additional custom properties to embed in the service.
     *
     * WARNING: entries may overwrite existing fields and result in invalid documents.
     */
    readonly properties?: Map<string, any> | Record<string, any>;
}"#;
