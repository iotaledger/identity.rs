// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::WasmKeyType;
use crate::did::wasm_core_url::WasmCoreDIDUrl;
use crate::did::WasmCoreDID;
use crate::did::WasmMethodData;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::crypto::PublicKey;
use identity_iota::did::VerificationMethod;
use wasm_bindgen::prelude::*;
/// A DID Document Verification Method.
#[wasm_bindgen(js_name = CoreVerificationMethod, inspectable)]
pub struct WasmCoreVerificationMethod(pub(crate) VerificationMethod);

#[wasm_bindgen(js_class = CoreVerificationMethod)]
impl WasmCoreVerificationMethod {
  /// Creates a new `CoreVerificationMethod` from the given `did` and public key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &WasmCoreDID,
    keyType: WasmKeyType,
    publicKey: Vec<u8>,
    fragment: String,
  ) -> Result<WasmCoreVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(publicKey);
    VerificationMethod::new(did.0.clone(), keyType.into(), &public_key, &fragment)
      .map(Self)
      .wasm_result()
  }

  /// Returns a copy of the `CoreDIDUrl` of the `CoreVerificationMethod`'s `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmCoreDIDUrl {
    WasmCoreDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `controller` `DID` of the `CoreVerificationMethod`.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmCoreDID {
    WasmCoreDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `CoreVerificationMethod` object.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, did: &WasmCoreDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Returns a copy of the `CoreVerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> WasmMethodType {
    WasmMethodType::from(self.0.type_())
  }

  /// Returns a copy of the `CoreVerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }
}

impl_wasm_json!(WasmCoreVerificationMethod, CoreVerificationMethod);
impl_wasm_clone!(WasmCoreVerificationMethod, CoreVerificationMethod);

impl From<VerificationMethod> for WasmCoreVerificationMethod {
  fn from(method: VerificationMethod) -> Self {
    Self(method)
  }
}
