// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::PublicKey;

use identity_iota::iota::IotaVerificationMethod;
use wasm_bindgen::prelude::*;

use crate::common::MapStringAny;
use crate::crypto::WasmKeyType;
use crate::did::WasmMethodData;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDIDUrl;

#[wasm_bindgen(js_name = IotaVerificationMethod, inspectable)]
pub struct WasmIotaVerificationMethod(pub(crate) IotaVerificationMethod);

#[wasm_bindgen(js_class = IotaVerificationMethod)]
impl WasmIotaVerificationMethod {
  /// Creates a new `IotaVerificationMethod` from the given `did` and public key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &WasmIotaDID,
    keyType: WasmKeyType,
    publicKey: Vec<u8>,
    fragment: String,
  ) -> Result<WasmIotaVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(publicKey);
    IotaVerificationMethod::new(did.0.clone(), keyType.into(), &public_key, &fragment)
      .map(Self)
      .wasm_result()
  }

  /// Returns a reference to the `IotaVerificationMethod` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmIotaDIDUrl {
    WasmIotaDIDUrl::from(self.0.id().clone())
  }

  /// Sets the id of the `IotaVerificationMethod`.
  #[wasm_bindgen(js_name = setId)]
  pub fn set_id(&mut self, id: &WasmIotaDIDUrl) -> Result<()> {
    self.0.set_id(id.0.clone()).wasm_result()?;
    Ok(())
  }

  /// Returns a copy of the `controller` `DID` of the `IotaVerificationMethod`.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmIotaDID {
    WasmIotaDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `IotaVerificationMethod`.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, did: &WasmIotaDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Sets the `IotaVerificationMethod` type.
  #[wasm_bindgen(js_name = setType)]
  pub fn set_type(&mut self, type_: &WasmMethodType) {
    *self.0.type_mut() = type_.0.clone();
  }

  /// Returns a copy of the `IotaVerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmMethodType {
    WasmMethodType::from(self.0.type_().clone())
  }

  /// Returns a copy of the `IotaVerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }

  /// Sets `IotaVerificationMethod` public key data.
  #[wasm_bindgen(js_name = setData)]
  pub fn set_data(&mut self, data: &WasmMethodData) {
    *self.0.data_mut() = data.0.clone();
  }

  /// Get custom properties of the Verification Method.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
  }

  /// Adds a custom property to the Verification Method.
  /// If the value is set to `null`, the custom property will be removed.
  ///
  /// ### WARNING
  /// This method can overwrite existing properties like `id` and result
  /// in an invalid Verification Method.
  #[wasm_bindgen(js_name = setPropertyUnchecked)]
  pub fn set_property_unchecked(&mut self, key: String, value: &JsValue) -> Result<()> {
    let value: Option<serde_json::Value> = value.into_serde().wasm_result()?;
    match value {
      Some(value) => {
        self.0.properties_mut().insert(key, value);
      }
      None => {
        self.0.properties_mut().remove(&key);
      }
    }
    Ok(())
  }
}

impl_wasm_json!(WasmIotaVerificationMethod, IotaVerificationMethod);
impl_wasm_clone!(WasmIotaVerificationMethod, IotaVerificationMethod);

impl From<IotaVerificationMethod> for WasmIotaVerificationMethod {
  fn from(method: IotaVerificationMethod) -> Self {
    Self(method)
  }
}
