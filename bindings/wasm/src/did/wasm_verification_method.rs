// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::MapStringAny;
use crate::crypto::WasmKeyType;
use crate::did::wasm_did_url::WasmDIDUrl;
use crate::did::WasmCoreDID;
use crate::did::WasmMethodData;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::crypto::PublicKey;
use identity_iota::verification::VerificationMethod;
use wasm_bindgen::prelude::*;

use super::wasm_core_did::ICoreDID;

/// A DID Document Verification Method.
#[wasm_bindgen(js_name = VerificationMethod, inspectable)]
pub struct WasmVerificationMethod(pub(crate) VerificationMethod);

#[wasm_bindgen(js_class = VerificationMethod)]
impl WasmVerificationMethod {
  /// Creates a new `VerificationMethod` from the given `did` and public key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &ICoreDID,
    keyType: WasmKeyType,
    publicKey: Vec<u8>,
    fragment: String,
  ) -> Result<WasmVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(publicKey);
    VerificationMethod::new(did.to_core_did().0, keyType.into(), &public_key, &fragment)
      .map(Self)
      .wasm_result()
  }

  /// Returns a copy of the `DIDUrl` of the `VerificationMethod`'s `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Sets the id of the `VerificationMethod`.
  #[wasm_bindgen(js_name = setId)]
  pub fn set_id(&mut self, id: &WasmDIDUrl) -> Result<()> {
    self.0.set_id(id.0.clone()).wasm_result()?;
    Ok(())
  }

  /// Returns a copy of the `controller` `DID` of the `VerificationMethod`.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmCoreDID {
    WasmCoreDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `VerificationMethod` object.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, did: &WasmCoreDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Returns a copy of the `VerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmMethodType {
    WasmMethodType::from(self.0.type_())
  }

  /// Sets the `VerificationMethod` type.
  #[wasm_bindgen(js_name = setType)]
  pub fn set_type(&mut self, type_: &WasmMethodType) {
    *self.0.type_mut() = type_.0;
  }

  /// Returns a copy of the `VerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }

  /// Sets `VerificationMethod` public key data.
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

impl_wasm_json!(WasmVerificationMethod, VerificationMethod);
impl_wasm_clone!(WasmVerificationMethod, VerificationMethod);

impl From<VerificationMethod> for WasmVerificationMethod {
  fn from(method: VerificationMethod) -> Self {
    Self(method)
  }
}
