// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::convert::TryInto;

use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use lazy_static::lazy_static;
use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_did::did::BaseDIDUrl;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use super::segments::Segments;

pub type Result<T> = std::result::Result<T, DIDError>;

/// A DID URL conforming to the IOTA Stardust DID method specification.
///
/// See [`DIDUrl`].
pub type IotaDIDUrl = DIDUrl<StardustDID>;

/// A DID conforming to the IOTA UTXO DID method specification.
///
/// This is a thin wrapper around the [`DID`][`CoreDID`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
pub struct StardustDID(CoreDID);

// Optimization to avoid validating the placeholder string every time a new DID is created with `StardustDID::new()`.
const INITIAL_ALIAS_ID: &'static str = "0x0000000000000000000000000000000000000000000000000000000000000000";
lazy_static! {
  // StardustDID's have 64-byte tags, matching the hex-encoding of the Alias ID. This value reflects the initial AliasID
  // which is required to be zeroed out.
  static ref PLACHEHOLDER_DID_STR: String = format!("did:stardust:{}:{}", StardustDID::DEFAULT_NETWORK, INITIAL_ALIAS_ID);
  static ref PLACEHOLDER_DID: StardustDID = StardustDID(CoreDID::parse(PLACHEHOLDER_DID_STR.as_str()).unwrap());
}

impl StardustDID {
  /// The URL scheme for Decentralized Identifiers.
  pub const SCHEME: &'static str = CoreDID::SCHEME;

  /// The IOTA DID method name (`"stardust"`).
  // TODO: This will be changed to `iota` before IOTA Identity 0.7 is released.
  pub const METHOD: &'static str = "stardust";

  /// The default Tangle network (`"main"`).
  // TODO: Currently we only have the  Shimmer testnet "rms", once stardust becomes available on main that should perhaps be the default?, 
  pub const DEFAULT_NETWORK: &'static str = "rms";

  /// Converts an owned [`CoreDID`] to a [`StardustDID`].
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn try_from_core(did: CoreDID) -> Result<Self> {
    Self::check_validity(&did)?;

    Ok(Self(did))
  }

  /// Parses an [`StardustDID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn parse(input: impl AsRef<str>) -> Result<Self> {
    CoreDID::parse(input).map_err(Into::into).and_then(Self::try_from_core)
  }

  /// Creates a new [`StardustDID`] with the default network. See [`Self::new_with_network`](Self::new_with_network) if
  /// another network is desired.
  pub fn new() -> Self {
    PLACEHOLDER_DID.clone()
  }

  /// Creates a new [`StardustDID`] with the given network.
  ///
  /// # Errors
  /// `Err` is returned if the network name does not satisfy the requirements of the [`StardustDID`] method
  /// specification.
  // TODO: consider refactoring to use `NetworkName` once that gets ported along with the `Client`.
  pub fn new_with_network(network: &str) -> Result<Self> {
    Self::check_network_str(network)?;
    CoreDID::parse(&format!("did:stardust:{}:{}", network, INITIAL_ALIAS_ID)).map(Self)
  }

  // Check whether the network satisfies the requirements of the [`StardustDID`] method specification.
  // TODO: Consider removing this code once `NetworkName` is ported together with the `Client`. 
  fn check_network_str(network: &str) -> Result<()> {
    // TODO: move this logic to a dedicated network (name) struct once work on the stardust `Client` starts.
    const MAX_NETWORK_CHARACTERS: usize = 6;

    (!network.is_empty())
      .then_some(())
      .ok_or(DIDError::Other("invalid network name: no network was provided"))?;
    (network.len() <= MAX_NETWORK_CHARACTERS)
      .then_some(())
      .ok_or(DIDError::Other(
        "invalid network name: maximum of six characters exceeded",
      ))?;
    network
      .chars()
      .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
      .then_some(())
      .ok_or(DIDError::Other(
        "invalid network name: name does not exclusively consist of lower case ascii characters and digits",
      ))?;

    Ok(())
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] network name.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid network name according to the [`StardustDID`] method specification. 
  pub fn check_network<D: DID>(did: &D) -> Result<()> {
    let network_name = Segments(did.method_id()).network();
    dbg!(network_name);
    Self::check_network_str(network_name)
  }

  /// Checks if the given `DID` is syntactically valid according to the [`StardustDID`] method specification.
  ///
  ///
  ///  This function does NOT check whether the `did-method-specific-id` corresponds to a Blake2b-256 hash of an
  ///  `OutputID`, despite this being a requirement for the [`StardustDID`] method specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a syntactically valid [`StardustDID`].
  pub fn check_validity<D: DID>(did: &D) -> Result<()> {
    todo!();
  }

  /// Checks if the given `DID` has a valid IOTA DID `method` (i.e. `"iota"`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_method<D: DID>(did: &D) -> Result<()> {
    (did.method() == Self::METHOD)
      .then_some(())
      .ok_or(DIDError::InvalidMethodName)
  }

}

/*
impl DID for StardustDID {

  /// Returns the [`StardustDID`] scheme. See [`DID::SCHEME`].
  fn scheme(&self) -> &'static str {
    self.0.scheme()
  }

  /// Returns the [`StardustDID`] authority.
  fn authority(&self) -> &str {
    self.0.authority()
  }

  /// Returns the [`StardustDID`] method name.
  fn method(&self) -> &str {
    self.0.method()
  }

  /// Returns the [`StardustDID`] method-specific ID.
  fn method_id(&self) -> &str {
    self.0.method_id()
  }

  /// Returns the serialized [`StardustDID`].
  ///
  /// This is fast since the serialized value is stored in the [`DID`].
  fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// Consumes the [`StardustDID`] and returns the serialization.
  fn into_string(self) -> String {
    self.0.into_string()
  }

  /// Creates a new [`StardustDIDUrl`] by joining with a relative DID Url string.
  ///
  /// # Errors
  ///
  /// Returns `Err` if any base or relative DID segments are invalid.
  fn join(self, segment: impl AsRef<str>) -> std::result::Result<DIDUrl<Self>, DIDError> {
    self.into_url().join(segment)
  }

  fn to_url(&self) -> DIDUrl<Self> {
    DIDUrl::new(self.clone(), None)
  }

  fn into_url(self) -> DIDUrl<Self> {
    DIDUrl::new(self, None)
  }
}

*/

impl Display for StardustDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<StardustDID> for CoreDID {
  fn from(id: StardustDID) -> Self {
    id.0
  }
}

impl TryFrom<CoreDID> for StardustDID {
  type Error = DIDError;
  fn try_from(value: CoreDID) -> std::result::Result<Self, Self::Error> {
    Self::try_from_core(value)
  }
}

impl TryFrom<BaseDIDUrl> for StardustDID {
  type Error = DIDError;

  fn try_from(other: BaseDIDUrl) -> Result<Self> {
    let core_did: CoreDID = CoreDID::try_from(other)?;
    Self::try_from(core_did)
  }
}

impl AsRef<CoreDID> for StardustDID {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}

#[cfg(test)]
mod tests {

  use iota_client::bee_block::output::AliasId;
  use iota_client::bee_block::output::OutputId;

  use super::*;


  fn assert_new_with_network(input: &str, expected_ok: bool) {
    assert_eq!(
      StardustDID::new_with_network(input).is_ok(),
      expected_ok,
      "assert_new_with_network: failed on input: {} and expectation {}",
      input,
      expected_ok
    );
  }

  fn assert_check_network(input: &str, expected_ok: bool) {
    let did_core: CoreDID = CoreDID::parse(input).expect(&format!("input to `assert_check_network` should parse to a DID. This was not the case for {}",input)); 
    assert_eq!(
      StardustDID::check_network(&did_core).is_ok(),expected_ok, 
      "assert_check_network failed with input {} and expectation {}", 
      input,
      expected_ok
    );
  }



  fn assert_parse(input: &str, expected_ok: bool) {
    assert_eq!(
      StardustDID::parse(input).is_ok(),
      expected_ok,
      "assert_parse: failed on input: {} and expectation {}",
      input,
      expected_ok
    );
  }



  lazy_static! {
    // obtain AliasID from a valid OutputID string
    // output_id copied from https://github.com/iotaledger/bee/blob/30cab4f02e9f5d72ffe137fd9eb09723b4f0fdb6/bee-block/tests/output_id.rs
    static ref VALID_ALIAS_ID_STRING: String = OutputId::from_str("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00").map(
      AliasId::from
    ).map(|alias| alias.to_string()).unwrap();

    static ref VALID_STARDUST_DID_STRING: String = "did:stardust:".to_owned().chars().chain(VALID_ALIAS_ID_STRING.chars()).collect();
  }

  const VALID_NETWORK_NAMES: [&str; 7] = ["foo", "foobar", "123456", "0", "foo42", "bar123", "42foo"];

  const INVALID_NETWORK_NAMES: [&str; 7] = ["féta", "", "  ", "foo ", " foo", "1234567", "foobar0"];

  #[test]
  fn new_provides_placeholder() {
    assert_eq!(StardustDID::new().0.as_str(), PLACHEHOLDER_DID_STR.as_str());
  }

  #[test]
  fn invalid_method() {
    let did_key_core: CoreDID = CoreDID::parse("did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK").unwrap();
    assert!(matches!(
      StardustDID::check_method(&did_key_core),
      Err(DIDError::InvalidMethodName)
    ));
  }

  #[test]
  fn valid_check_method() {
    let did_stardust_core: CoreDID = CoreDID::parse(&*VALID_STARDUST_DID_STRING).unwrap();
    assert!(StardustDID::check_method(&did_stardust_core).is_ok());
  }

  // TODO: Move test once a dedicated struct for network name gets ported along with the client.
  #[test]
  fn valid_new_with_network() {
    for input in VALID_NETWORK_NAMES {
      assert_new_with_network(input, true);
    }
  }

  // TODO: Move test once a dedicated struct for network name gets ported along with the client.
  #[test]
  fn invalid_new_with_network() {
    for input in INVALID_NETWORK_NAMES {
      assert_new_with_network(input, false);
    }
  }


  #[test]
  fn valid_check_network() {
    for network_name in VALID_NETWORK_NAMES {
      let did_string = format!("did:method:{}:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK", network_name);
      assert_check_network(&did_string, true);
    }

    assert_check_network(
      "did:method:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK", 
      true
    );
  }

  //TODO NEXT: write test named invalid_check_network

  /*
  #[test]
  fn parse_valid_initialized_stardust_did() {
    check_parse(VALID_STARDUST_DID);
  }
  */
}
