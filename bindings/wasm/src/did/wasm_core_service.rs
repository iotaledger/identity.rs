// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::deserialize_map_or_any;
use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::did::wasm_core_url::WasmCoreDIDUrl;
use crate::did::IService;
use crate::did::UServiceEndpoint;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::core::OneOrMany;
use identity_iota::did::CoreDIDUrl;
use identity_iota::document::Service;
use identity_iota::document::ServiceEndpoint;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// A DID Document Service used to enable trusted interactions associated with a DID subject.
#[wasm_bindgen(js_name = CoreService, inspectable)]
pub struct WasmCoreService(pub(crate) Service);

#[wasm_bindgen(js_class = CoreService)]
impl WasmCoreService {
  #[wasm_bindgen(constructor)]
  pub fn new(service: ICoreService) -> Result<WasmCoreService> {
    let id: CoreDIDUrl = service.id().into_serde().wasm_result()?;

    let base_service: &IService = service.as_ref();
    let types: OneOrMany<String> = service.type_().into_serde().wasm_result()?;
    let service_endpoint: ServiceEndpoint = deserialize_map_or_any(&base_service.service_endpoint())?;
    let properties: Option<identity_iota::core::Object> = deserialize_map_or_any(&base_service.properties())?;

    Service::builder(properties.unwrap_or_default())
      .id(id)
      .types(types)
      .service_endpoint(service_endpoint)
      .build()
      .map(WasmCoreService)
      .wasm_result()
  }

  /// Returns a copy of the `CoreService` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmCoreDIDUrl {
    WasmCoreDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `CoreService` type.
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

  /// Returns a copy of the `CoreService` endpoint.
  #[wasm_bindgen(js_name = serviceEndpoint)]
  pub fn service_endpoint(&self) -> UServiceEndpoint {
    UServiceEndpoint::from(self.0.service_endpoint())
  }

  /// Returns a copy of the custom properties on the `CoreService`.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
  }
}

impl_wasm_json!(WasmCoreService, CoreService);
impl_wasm_clone!(WasmCoreService, CoreService);

impl From<Service> for WasmCoreService {
  fn from(service: Service) -> Self {
    Self(service)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ICoreService", extends = IService)]
  pub type ICoreService;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &ICoreService) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const I_CORE_SERVICE: &'static str = r#"
/**
 * Holds options to create a new `CoreService`.
 */
interface ICoreService extends IService {
    /**
     * Identifier of the service.
     *
     * Must be a valid DIDUrl with a fragment.
     */
    readonly id: CoreDIDUrl | string;
}"#;
