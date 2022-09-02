// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::OneOrMany;
use identity_iota::did::ServiceEndpoint;
use identity_iota::iota_core::IotaDIDUrl;
use identity_iota::iota_core::IotaService;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common::deserialize_map_or_any;
use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::did::IService;
use crate::did::UServiceEndpoint;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDIDUrl;

/// A `Service` adhering to the IOTA UTXO DID method specification.
#[wasm_bindgen(js_name = IotaService, inspectable)]
pub struct WasmIotaService(pub(crate) IotaService);

#[wasm_bindgen(js_class = IotaService)]
impl WasmIotaService {
  #[wasm_bindgen(constructor)]
  pub fn new(service: IIotaService) -> Result<WasmIotaService> {
    let id: IotaDIDUrl = service.id().into_serde().wasm_result()?;

    let base_service: &IService = service.as_ref();
    let types: OneOrMany<String> = service.type_().into_serde().wasm_result()?;
    let service_endpoint: ServiceEndpoint = deserialize_map_or_any(&base_service.service_endpoint())?;
    let properties: Option<identity_iota::core::Object> = deserialize_map_or_any(&base_service.properties())?;

    IotaService::builder(properties.unwrap_or_default())
      .id(id)
      .types(types)
      .service_endpoint(service_endpoint)
      .build()
      .map(WasmIotaService)
      .wasm_result()
  }

  /// Returns a copy of the `Service` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmIotaDIDUrl {
    WasmIotaDIDUrl::from(self.0.id().clone())
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

impl_wasm_json!(WasmIotaService, IotaService);
impl_wasm_clone!(WasmIotaService, IotaService);

impl From<IotaService> for WasmIotaService {
  fn from(service: IotaService) -> Self {
    Self(service)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IIotaService", extends = IService)]
  pub type IIotaService;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &IIotaService) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const I_STARDUST_SERVICE: &'static str = r#"
/**
 * Holds options to create a new `IotaService`.
 */
interface IIotaService extends IService {
    /**
     * Identifier of the service.
     *
     * Must be a valid DIDUrl with a fragment.
     */
    readonly id: IotaDIDUrl | string;
}"#;
