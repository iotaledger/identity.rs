// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DID;
use identity_iota::iota_core::IotaDID;
use wasm_bindgen::prelude::*;

use crate::did::wasm_did_url::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::WasmNetwork;

/// @typicalname did
#[wasm_bindgen(js_name = DID, inspectable)]
pub struct WasmDID(pub(crate) IotaDID);

#[wasm_bindgen(js_class = DID)]
impl WasmDID {
  /// Creates a new `DID` from a public key.
  #[wasm_bindgen(constructor)]
  pub fn new(public_key: &[u8], network: Option<String>) -> Result<WasmDID> {
    Self::from_public_key(public_key, network)
  }

  /// Creates a new `DID` from an arbitrary public key.
  fn from_public_key(public_key: &[u8], network: Option<String>) -> Result<WasmDID> {
    let did = if let Some(network) = network {
      IotaDID::new_with_network(public_key, network)
    } else {
      IotaDID::new(public_key)
    };
    did.wasm_result().map(Self)
  }

  /// Parses a `DID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmDID> {
    IotaDID::parse(input).wasm_result().map(Self)
  }

  /// Returns the IOTA tangle network of the `DID`.
  #[wasm_bindgen]
  pub fn network(&self) -> Result<WasmNetwork> {
    self.0.network().map(Into::into).wasm_result()
  }

  /// Returns the IOTA tangle network of the `DID`.
  #[wasm_bindgen(getter = networkName)]
  pub fn network_name(&self) -> String {
    self.0.network_str().into()
  }

  /// Returns a copy of the unique tag of the `DID`.
  #[wasm_bindgen]
  pub fn tag(&self) -> String {
    self.0.tag().into()
  }

  /// Construct a new `DIDUrl` by joining with a relative DID Url string.
  #[wasm_bindgen]
  pub fn join(self, segment: &str) -> Result<WasmDIDUrl> {
    self.0.join(segment).wasm_result().map(WasmDIDUrl)
  }

  /// Clones the `DID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = toUrl)]
  pub fn to_url(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.to_url())
  }

  /// Converts the `DID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = intoUrl)]
  pub fn into_url(self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.into_url())
  }

  /// Returns the `DID` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmDID, DID);
impl_wasm_clone!(WasmDID, DID);

impl From<IotaDID> for WasmDID {
  fn from(did: IotaDID) -> Self {
    Self(did)
  }
}

/// Duck-typed union to pass either a string or WasmDID as a parameter.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DID | string")]
  pub type UWasmDID;
}

impl TryFrom<UWasmDID> for IotaDID {
  type Error = JsValue;

  fn try_from(did: UWasmDID) -> std::result::Result<Self, Self::Error> {
    // Parse rather than going through serde directly to return proper error types.
    let json: String = did.into_serde().wasm_result()?;
    IotaDID::parse(&json).wasm_result()
  }
}
