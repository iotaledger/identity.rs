// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::PublicKey;
use identity_iota::iota_core::IotaVerificationMethod;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyType;
use crate::did::WasmDIDUrl;
use crate::did::WasmIotaDID;
use crate::did::WasmMethodData;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = VerificationMethod, inspectable)]
pub struct WasmVerificationMethod(pub(crate) IotaVerificationMethod);

#[wasm_bindgen(js_class = VerificationMethod)]
impl WasmVerificationMethod {
  /// Creates a new `VerificationMethod` from the given `did` and public key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &WasmIotaDID,
    keyType: WasmKeyType,
    publicKey: Vec<u8>,
    fragment: String,
  ) -> Result<WasmVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(publicKey);
    IotaVerificationMethod::new(did.0.clone(), keyType.into(), &public_key, &fragment)
      .map(Self)
      .wasm_result()
  }

  /// Returns a copy of the `id` `DIDUrl` of the `VerificationMethod`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `controller` `DID` of the `VerificationMethod`.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmIotaDID {
    WasmIotaDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `VerificationMethod` object.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, did: &WasmIotaDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Returns a copy of the `VerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmMethodType {
    WasmMethodType::from(self.0.type_())
  }

  /// Returns a copy of the `VerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }
}

impl_wasm_json!(WasmVerificationMethod, VerificationMethod);
impl_wasm_clone!(WasmVerificationMethod, VerificationMethod);

impl From<IotaVerificationMethod> for WasmVerificationMethod {
  fn from(method: IotaVerificationMethod) -> Self {
    Self(method)
  }
}
