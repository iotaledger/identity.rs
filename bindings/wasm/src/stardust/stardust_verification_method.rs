// Copyright 2020-2022  Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::MapStringAny;
use identity_iota::crypto::PublicKey;
use identity_stardust::StardustVerificationMethod;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyType;
use crate::did::WasmMethodData;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDID;
use crate::stardust::WasmStardustDIDUrl;

#[wasm_bindgen(js_name = StardustVerificationMethod, inspectable)]
pub struct WasmStardustVerificationMethod(pub(crate) StardustVerificationMethod);

#[wasm_bindgen(js_class = StardustVerificationMethod)]
impl WasmStardustVerificationMethod {
  /// Creates a new `StardustVerificationMethod` from the given `did` and public key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &WasmStardustDID,
    keyType: WasmKeyType,
    publicKey: Vec<u8>,
    fragment: String,
  ) -> Result<WasmStardustVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(publicKey);
    StardustVerificationMethod::new(did.0.clone(), keyType.into(), &public_key, &fragment)
      .map(Self)
      .wasm_result()
  }

  /// Returns a copy of the `StardustVerificationMethod` id.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmStardustDIDUrl {
    WasmStardustDIDUrl::from(self.0.id().clone())
  }

  /// Sets the id of the `StardustVerificationMethod`.
  #[wasm_bindgen(js_name = setId)]
  pub fn set_id(&mut self, id: &WasmStardustDIDUrl) -> Result<()> {
    self.0.set_id(id.0.clone()).wasm_result()?;
    Ok(())
  }

  /// Returns a copy of the `controller` `DID` of the `StardustVerificationMethod`.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmStardustDID {
    WasmStardustDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `StardustVerificationMethod`.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, did: &WasmStardustDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Returns a copy of the `StardustVerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmMethodType {
    WasmMethodType::from(self.0.type_())
  }

  /// Sets the `StardustVerificationMethod` type.
  #[wasm_bindgen(js_name = setType)]
  pub fn set_type(&mut self, type_: &WasmMethodType) {
    *self.0.type_mut() = type_.0.clone();
  }

  /// Returns a copy of the `StardustVerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }

  /// Sets `StardustVerificationMethod` public key data.
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

impl_wasm_json!(WasmStardustVerificationMethod, StardustVerificationMethod);
impl_wasm_clone!(WasmStardustVerificationMethod, StardustVerificationMethod);

impl From<StardustVerificationMethod> for WasmStardustVerificationMethod {
  fn from(method: StardustVerificationMethod) -> Self {
    Self(method)
  }
}
