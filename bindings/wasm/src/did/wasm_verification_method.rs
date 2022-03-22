// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::merkle_key::Blake2b256;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::PublicKey;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaVerificationMethod;
use wasm_bindgen::prelude::*;

use crate::crypto::Digest;
use crate::crypto::WasmKeyCollection;
use crate::crypto::WasmKeyType;
use crate::did::wasm_did_url::WasmDIDUrl;
use crate::did::WasmDID;
use crate::did::WasmMethodData;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = VerificationMethod, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmVerificationMethod(pub(crate) IotaVerificationMethod);

#[wasm_bindgen(js_class = VerificationMethod)]
impl WasmVerificationMethod {
  /// Creates a new `VerificationMethod` object from the given `did` and public key.
  #[wasm_bindgen(constructor)]
  pub fn new(
    did: &WasmDID,
    key_type: WasmKeyType,
    public_key: Vec<u8>,
    fragment: String,
  ) -> Result<WasmVerificationMethod> {
    let public_key: PublicKey = PublicKey::from(public_key);
    IotaVerificationMethod::new(did.0.clone(), key_type.into(), &public_key, &fragment)
      .map(Self)
      .map_err(wasm_error)
  }

  /// Creates a new `MerkleKeyCollection2021` method from the given key collection.
  #[wasm_bindgen(js_name = newMerkleKey)]
  pub fn new_merkle_key(
    digest: Digest,
    did: &WasmDID,
    keys: &WasmKeyCollection,
    fragment: &str,
  ) -> Result<WasmVerificationMethod> {
    let did: IotaDID = did.0.clone();
    match digest {
      Digest::Sha256 => IotaVerificationMethod::new_merkle_key::<Sha256>(did, &keys.0, fragment)
        .map_err(wasm_error)
        .map(Self),
      Digest::Blake2b256 => IotaVerificationMethod::new_merkle_key::<Blake2b256>(did, &keys.0, fragment)
        .map_err(wasm_error)
        .map(Self),
    }
  }

  /// Returns a copy of the `id` `DIDUrl` of the `VerificationMethod` object.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.id().clone())
  }

  /// Returns a copy of the `controller` `DID` of the `VerificationMethod` object.
  #[wasm_bindgen]
  pub fn controller(&self) -> WasmDID {
    WasmDID::from(self.0.controller().clone())
  }

  /// Sets the `controller` `DID` of the `VerificationMethod` object.
  #[wasm_bindgen(js_name = SetController)]
  pub fn set_controller(&mut self, did: &WasmDID) {
    *self.0.controller_mut() = did.0.clone();
  }

  /// Returns a copy of the `VerificationMethod` type.
  #[wasm_bindgen(js_name = type)]
  pub fn type_(&self) -> String {
    self.0.type_().as_str().into()
  }

  /// Returns a copy of the `VerificationMethod` public key data.
  #[wasm_bindgen]
  pub fn data(&self) -> WasmMethodData {
    WasmMethodData::from(self.0.data().clone())
  }

  /// Serializes a `VerificationMethod` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `VerificationMethod` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<WasmVerificationMethod> {
    value.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmVerificationMethod, VerificationMethod);

impl From<IotaVerificationMethod> for WasmVerificationMethod {
  fn from(method: IotaVerificationMethod) -> Self {
    Self(method)
  }
}
