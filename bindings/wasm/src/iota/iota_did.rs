// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DIDError;
use identity_iota::did::DID;
use identity_iota::iota_core::IotaDID;
use identity_iota::iota_core::NetworkName;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::WasmIotaDIDUrl;

/// A DID conforming to the IOTA UTXO DID method specification.
///
/// @typicalname did
#[wasm_bindgen(js_name = IotaDID, inspectable)]
pub struct WasmIotaDID(pub(crate) IotaDID);

#[wasm_bindgen(js_class = IotaDID)]
impl WasmIotaDID {
  /// The IOTA UTXO DID method name (`"iota"`).
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

  /// Constructs a new `IotaDID` from a byte representation of the tag and the given
  /// network name.
  ///
  /// See also {@link IotaDID.placeholder}.
  #[wasm_bindgen(constructor)]
  pub fn new(bytes: &[u8], network: String) -> Result<WasmIotaDID> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    let tag_bytes: &[u8; 32] = bytes
      .try_into()
      .map_err(|_| DIDError::Other("invalid bytes length for IotaDID tag, expected 32"))
      .wasm_result()?;
    Ok(Self::from(IotaDID::new(tag_bytes, &network_name)))
  }

  /// Creates a new placeholder [`IotaDID`] with the given network name.
  ///
  /// E.g. `did:iota:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.
  #[wasm_bindgen]
  pub fn placeholder(network: String) -> Result<WasmIotaDID> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    Ok(Self::from(IotaDID::placeholder(&network_name)))
  }

  /// Parses a `IotaDID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmIotaDID> {
    IotaDID::parse(input).map(Self).wasm_result()
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

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
  pub fn join(&self, segment: &str) -> Result<WasmIotaDIDUrl> {
    self.0.clone().join(segment).wasm_result().map(WasmIotaDIDUrl)
  }

  /// Clones the `DID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = toUrl)]
  pub fn to_url(&self) -> WasmIotaDIDUrl {
    WasmIotaDIDUrl::from(self.0.to_url())
  }

  /// Converts the `DID` into a `DIDUrl`, consuming it.
  #[wasm_bindgen(js_name = intoUrl)]
  pub fn into_url(self) -> WasmIotaDIDUrl {
    WasmIotaDIDUrl::from(self.0.into_url())
  }

  /// Returns the `DID` as a string.
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
