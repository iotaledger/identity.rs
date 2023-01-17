// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::document::ServiceEndpoint;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

impl From<ServiceEndpoint> for UServiceEndpoint {
  fn from(endpoint: ServiceEndpoint) -> Self {
    UServiceEndpoint::from(&endpoint)
  }
}

impl From<&ServiceEndpoint> for UServiceEndpoint {
  fn from(endpoint: &ServiceEndpoint) -> Self {
    match endpoint {
      // string
      ServiceEndpoint::One(url) => JsValue::from_str(url.as_str()).unchecked_into::<UServiceEndpoint>(),
      // string[]
      ServiceEndpoint::Set(set) => set
        .iter()
        .map(|url| JsValue::from_str(url.as_str()))
        .collect::<js_sys::Array>()
        .unchecked_into::<UServiceEndpoint>(),
      // Map<string, string[]>
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

  #[wasm_bindgen(method, getter, js_name = type)]
  pub fn type_(this: &IService) -> JsValue;

  #[wasm_bindgen(method, getter, js_name = serviceEndpoint)]
  pub fn service_endpoint(this: &IService) -> JsValue;

  #[wasm_bindgen(method, getter)]
  pub fn properties(this: &IService) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const I_SERVICE: &'static str = r#"
/**
 * Base `Service` properties.
 */
interface IService {
    /**
     * Type of service.
     *
     * E.g. "LinkedDomains" or "DIDCommMessaging".
     */
    readonly type: string | string[];

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
