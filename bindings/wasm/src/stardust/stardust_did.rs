// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::DIDError;
use identity_iota::did::DID;
use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDIDUrl;

/// A DID conforming to the IOTA UTXO DID method specification.
///
/// @typicalname did
#[wasm_bindgen(js_name = StardustDID, inspectable)]
pub struct WasmStardustDID(pub(crate) StardustDID);

#[wasm_bindgen(js_class = StardustDID)]
impl WasmStardustDID {
  /// The IOTA UTXO DID method name (`"stardust"`).
  // TODO: This will be changed to `iota` in the future.
  #[wasm_bindgen(getter = METHOD)]
  pub fn static_method() -> String {
    StardustDID::METHOD.to_owned()
  }

  /// The default Tangle network (`"main"`).
  #[wasm_bindgen(getter = DEFAULT_NETWORK)]
  pub fn static_default_network() -> String {
    StardustDID::DEFAULT_NETWORK.to_owned()
  }

  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs a new `StardustDID` from a byte representation of the tag and the given
  /// network name.
  ///
  /// See also {@link StardustDID.placeholder}.
  #[wasm_bindgen(constructor)]
  pub fn new(bytes: &[u8], network: String) -> Result<WasmStardustDID> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    let tag_bytes: &[u8; 32] = bytes
      .try_into()
      .map_err(|_| DIDError::Other("invalid bytes length for StardustDID tag, expected 32"))
      .wasm_result()?;
    Ok(Self::from(StardustDID::new(tag_bytes, &network_name)))
  }

  /// Creates a new placeholder [`StardustDID`] with the given network name.
  ///
  /// E.g. `did:stardust:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.
  #[wasm_bindgen]
  pub fn placeholder(network: String) -> Result<WasmStardustDID> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    Ok(Self::from(StardustDID::placeholder(&network_name)))
  }

  /// Parses a `StardustDID` from the input string.
  #[wasm_bindgen]
  pub fn parse(input: &str) -> Result<WasmStardustDID> {
    StardustDID::parse(input).map(Self).wasm_result()
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the Tangle network name of the `StardustDID`.
  #[wasm_bindgen(js_name = networkStr)]
  pub fn network_str(&self) -> String {
    self.0.network_str().to_owned()
  }

  /// Returns a copy of the unique tag of the `StardustDID`.
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
  pub fn join(self, segment: &str) -> Result<WasmStardustDIDUrl> {
    self.0.join(segment).wasm_result().map(WasmStardustDIDUrl)
  }

  /// Clones the `DID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = toUrl)]
  pub fn to_url(&self) -> WasmStardustDIDUrl {
    WasmStardustDIDUrl::from(self.0.to_url())
  }

  /// Converts the `DID` into a `DIDUrl`.
  #[wasm_bindgen(js_name = intoUrl)]
  pub fn into_url(self) -> WasmStardustDIDUrl {
    WasmStardustDIDUrl::from(self.0.into_url())
  }

  /// Returns the `DID` as a string.
  #[allow(clippy::inherent_to_string)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl_wasm_json!(WasmStardustDID, StardustDID);
impl_wasm_clone!(WasmStardustDID, StardustDID);

impl From<StardustDID> for WasmStardustDID {
  fn from(did: StardustDID) -> Self {
    Self(did)
  }
}
