// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DID;
use identity_iota::iota_core::IotaDID;
use wasm_bindgen::prelude::*;

use crate::did::wasm_did_url::WasmDIDUrl;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::WasmNetwork;

/// A DID conforming to the IOTA DID method specification.
#[wasm_bindgen(js_name = IotaDID, inspectable)]
pub struct WasmIotaDID(pub(crate) IotaDID);

#[wasm_bindgen(js_class = IotaDID)]
impl WasmIotaDID {
  /// The IOTA DID method name (`"iota"`).
  #[wasm_bindgen(getter = METHOD)]
  pub fn static_method() -> String {
    IotaDID::METHOD.to_owned()
  }

  /// The default Tangle network (`"main"`).
  #[wasm_bindgen(getter = DEFAULT_NETWORK)]
  pub fn static_default_network() -> String {
    IotaDID::DEFAULT_NETWORK.to_owned()
  }

  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Creates a new `DID` from a public key.
  #[wasm_bindgen(constructor)]
  pub fn new(public_key: &[u8], network: Option<String>) -> Result<WasmIotaDID> {
    Self::from_public_key(public_key, network)
  }

  /// Creates a new `IotaDID` from an arbitrary public key.
  fn from_public_key(public_key: &[u8], network: Option<String>) -> Result<WasmIotaDID> {
    let did = if let Some(network) = network {
      IotaDID::new_with_network(public_key, network)
    } else {
      IotaDID::new(public_key)
    };
    did.wasm_result().map(Self)
  }

  /// Parses a `IotaDID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmIotaDID> {
    IotaDID::parse(input).wasm_result().map(Self)
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the Tangle network of the `IotaDID`.
  #[wasm_bindgen]
  pub fn network(&self) -> Result<WasmNetwork> {
    self.0.network().map(Into::into).wasm_result()
  }

  /// Returns the Tangle network name of the `IotaDID`.
  #[wasm_bindgen(js_name = networkStr)]
  pub fn network_str(&self) -> String {
    self.0.network_str().to_owned()
  }

  /// Returns a copy of the unique tag of the `IotaDID`.
  #[wasm_bindgen]
  pub fn tag(&self) -> String {
    self.0.tag().to_owned()
  }

  // ===========================================================================
  // DID trait
  // ===========================================================================

  /// Returns the `DID` scheme.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "did"`
  /// - `"did:iota:main:12345678" -> "did"`
  #[wasm_bindgen]
  pub fn scheme(&self) -> String {
    self.0.scheme().to_owned()
  }

  /// Returns the `DID` authority: the method name and method-id.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "example:12345678"`
  /// - `"did:iota:main:12345678" -> "iota:main:12345678"`
  #[wasm_bindgen]
  pub fn authority(&self) -> String {
    self.0.authority().to_owned()
  }

  /// Returns the `DID` method name.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "example"`
  /// - `"did:iota:main:12345678" -> "iota"`
  #[wasm_bindgen]
  pub fn method(&self) -> String {
    self.0.method().to_owned()
  }

  /// Returns the `DID` method-specific ID.
  ///
  /// E.g.
  /// - `"did:example:12345678" -> "12345678"`
  /// - `"did:iota:main:12345678" -> "main:12345678"`
  #[wasm_bindgen(js_name = methodId)]
  pub fn method_id(&self) -> String {
    self.0.method_id().to_owned()
  }

  /// Construct a new `DIDUrl` by joining with a relative DID Url string.
  #[wasm_bindgen]
  pub fn join(&self, segment: &str) -> Result<WasmDIDUrl> {
    self.0.clone().join(segment).wasm_result().map(WasmDIDUrl)
  }

  /// Clones the `IotaDID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = toUrl)]
  pub fn to_url(&self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.to_url())
  }

  /// Converts the `IotaDID` into a `DIDUrl`, consuming it.
  #[wasm_bindgen(js_name = intoUrl)]
  pub fn into_url(self) -> WasmDIDUrl {
    WasmDIDUrl::from(self.0.into_url())
  }

  /// Returns the `IotaDID` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmIotaDID, IotaDID);
impl_wasm_clone!(WasmIotaDID, IotaDID);

impl From<IotaDID> for WasmIotaDID {
  fn from(did: IotaDID) -> Self {
    Self(did)
  }
}

/// Duck-typed union to pass either a string or WasmDID as a parameter.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaDID | string")]
  pub type UWasmIotaDID;
}

impl TryFrom<UWasmIotaDID> for IotaDID {
  type Error = JsValue;

  fn try_from(did: UWasmIotaDID) -> std::result::Result<Self, Self::Error> {
    // Parse rather than going through serde directly to return proper error types.
    let json: String = did.into_serde().wasm_result()?;
    IotaDID::parse(&json).wasm_result()
  }
}
