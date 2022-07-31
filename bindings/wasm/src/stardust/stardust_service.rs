// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::OneOrMany;
use identity_iota::did::ServiceEndpoint;
use identity_stardust::StardustDIDUrl;
use identity_stardust::StardustService;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common::deserialize_map_or_any;
use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::did::IService;
use crate::did::UServiceEndpoint;
use crate::error::Result;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDIDUrl;

/// A `Service` adhering to the IOTA UTXO DID method specification.
#[wasm_bindgen(js_name = StardustService, inspectable)]
pub struct WasmStardustService(pub(crate) StardustService);

#[wasm_bindgen(js_class = StardustService)]
impl WasmStardustService {
  #[wasm_bindgen(constructor)]
  pub fn new(service: IStardustService) -> Result<WasmStardustService> {
    let id: StardustDIDUrl = service.id().into_serde().wasm_result()?;

    let i_service: &IService = service.as_ref();
    let types: OneOrMany<String> = service.type_().into_serde().wasm_result()?;
    let service_endpoint: ServiceEndpoint = deserialize_map_or_any(&i_service.service_endpoint())?;
    let properties: Option<identity_iota::core::Object> = deserialize_map_or_any(&i_service.properties())?;

    StardustService::builder(properties.unwrap_or_default())
      .id(id)
      .types(types)
      .service_endpoint(service_endpoint)
      .build()
      .map(WasmStardustService)
      .wasm_result()
  }

  /// Returns a copy of the `Service` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmStardustDIDUrl {
    WasmStardustDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `Service` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> ArrayString {
    self
      .0
      .type_()
      .iter()
      .cloned()
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Returns a copy of the `Service` endpoint.
  #[wasm_bindgen(js_name = serviceEndpoint)]
  pub fn service_endpoint(&self) -> UServiceEndpoint {
    UServiceEndpoint::from(self.0.service_endpoint())
  }

  /// Returns a copy of the custom properties on the `Service`.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
  }
}

impl_wasm_json!(WasmStardustService, StardustService);
impl_wasm_clone!(WasmStardustService, StardustService);

impl From<StardustService> for WasmStardustService {
  fn from(service: StardustService) -> Self {
    Self(service)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IStardustService", extends = IService)]
  pub type IStardustService;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &IStardustService) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const I_STARDUST_SERVICE: &'static str = r#"
/**
 * Holds options to create a new `StardustService`.
 */
interface IStardustService extends IService {
    /**
     * Identifier of the service.
     *
     * Must be a valid DIDUrl with a fragment.
     */
    readonly id: StardustDIDUrl | string;
}"#;
