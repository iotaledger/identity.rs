// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::convert::TryInto;

use once_cell::sync::Lazy;
use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_did::did::BaseDIDUrl;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

pub type Result<T> = std::result::Result<T, DIDError>;

/// A DID URL conforming to the IOTA Stardust DID method specification.
///
/// See [`DIDUrl`].
pub type StardustDIDUrl = DIDUrl<StardustDID>;

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

/// A DID conforming to the IOTA UTXO DID method specification.
///
/// This is a thin wrapper around the [`DID`][`CoreDID`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
pub struct StardustDID(CoreDID);

const INITIAL_ALIAS_ID: &'static str = "0x0000000000000000000000000000000000000000000000000000000000000000";

// StardustDID's have 64-byte tags, matching the hex-encoding of the Alias ID. This value reflects the initial AliasID
// which is required to be zeroed out.
// once_cell::sync::Lazy is utilized to avoid validating CoreDID::parse every time a
static PLACHEHOLDER_DID_STR: Lazy<String> =
  Lazy::new(|| format!("did:stardust:{}:{}", StardustDID::DEFAULT_NETWORK, INITIAL_ALIAS_ID));
static PLACEHOLDER_DID: Lazy<StardustDID> =
  Lazy::new(|| StardustDID(CoreDID::parse(PLACHEHOLDER_DID_STR.as_str()).unwrap()));

impl StardustDID {
  /// The URL scheme for Decentralized Identifiers.
  pub const SCHEME: &'static str = CoreDID::SCHEME;

  /// The IOTA DID method name (`"stardust"`).
  // TODO: This will be changed to `iota` before IOTA Identity 0.7 is released.
  pub const METHOD: &'static str = "stardust";

  /// The default Tangle network (`"main"`).
  // TODO: Currently we only have the  Shimmer testnet "rms", once stardust becomes available on main that should
  // perhaps be the default?,
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
    // TODO: Refactor this to use CoreDID::join
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

  // Check if the tag matches a potential alias_id
  fn check_tag_str(tag: &str) -> Result<()> {
    prefix_hex::decode::<[u8; BLAKE2B_256_LEN]>(tag)
      .map_err(|_| DIDError::InvalidMethodId)
      .map(|_| ())
  }

  // Normalizes the DID `method_id` by removing the default network segment if present.
  //
  // E.g.
  // - `"did:stardust:main:123" -> "did:stardust:123"` is normalized
  // - `"did:stardust:dev:123" -> "did:stardust:dev:123"` is unchanged
  fn normalize(mut did: CoreDID) -> CoreDID {
    let mut segments_iter = did.method_id().split(":");
    let normalized_id_string: Option<String> = match (segments_iter.next(), segments_iter.next()) {
      (Some(network), Some(tag)) => (network == Self::DEFAULT_NETWORK).then_some(tag.to_owned()),
      _ => None,
    };
    if let Some(tag) = normalized_id_string {
      did.set_method_id(tag);
    }
    did
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] network name.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid network name according to the [`StardustDID`] method specification.
  pub fn check_network<D: DID>(did: &D) -> Result<()> {
    let mut segment_iter = did.method_id().split(":");
    let network_name = match (segment_iter.next(), segment_iter.next()) {
      (Some(network), Some(tag)) => network,
      _ => Self::DEFAULT_NETWORK,
    };

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
  ///(
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_method<D: DID>(did: &D) -> Result<()> {
    (did.method() == Self::METHOD)
      .then_some(())
      .ok_or(DIDError::InvalidMethodName)
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] `method_id`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  // TODO: Is it correct to also validate the network here? The current IOTA DID method does NOT do that.
  pub fn check_method_id<D: DID>(did: &D) -> Result<()> {
    let id = did.method_id();
    let mut segments_iter = id.split(":");
    match (segments_iter.next(), segments_iter.next(), segments_iter.next()) {
      // OK if method_id = alias_id
      (Some(tag), None, None) => Self::check_tag_str(tag),
      // OK if method_id = network_id:alias_id
      (Some(network), Some(tag), None) => (Self::check_network_str(network).is_ok()
        && Self::check_tag_str(tag).is_ok())
      .then_some(())
      .ok_or(DIDError::InvalidMethodId),
      // Too many segments
      (_, _, Some(_)) => Err(DIDError::InvalidMethodId),
      // this last branch is actually unreachable, but needed to satisfy the compiler
      (None, _, _) => unreachable!("str::split should return at least one element"),
    }
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

  // obtain AliasID from a valid OutputID string
  // output_id copied from https://github.com/iotaledger/bee/blob/30cab4f02e9f5d72ffe137fd9eb09723b4f0fdb6/bee-block/tests/output_id.rs
  static VALID_ALIAS_ID_STRING: Lazy<String> = Lazy::new(|| {
    OutputId::from_str("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00")
      .map(AliasId::from)
      .map(|alias| alias.to_string())
      .unwrap()
  });

  static VALID_STARDUST_DID_STRING: Lazy<String> = Lazy::new(|| {
    "did:stardust:"
      .to_owned()
      .chars()
      .chain(VALID_ALIAS_ID_STRING.chars())
      .collect()
  });

  static VALID_STARDUST_DID_STRINGS: Lazy<Vec<String>> = Lazy::new(|| {
    let network_tag_to_did = |network, tag| format!("did:stardust:{}:{}", network, tag);

    let alias_id: &str = VALID_ALIAS_ID_STRING.as_str();
    vec![
      network_tag_to_did("main", alias_id),
      network_tag_to_did("dev", alias_id),
      network_tag_to_did("smr", alias_id),
      network_tag_to_did("rms", alias_id),
    ]
  });

  // Rules are: at least one character, at most six characters and may only contain digits and/or lowercase ascii
  // characters.
  const VALID_NETWORK_NAMES: [&str; 7] = ["foo", "foobar", "123456", "0", "foo42", "bar123", "42foo"];

  const INVALID_NETWORK_NAMES: [&str; 8] = ["Foo", "féta", "", "  ", "foo ", " foo", "1234567", "foobar0"];

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
      assert!(
        StardustDID::new_with_network(input).is_ok(),
        "test: valid_new_with_network: failed on input: {}",
        input,
      );
    }
  }

  // TODO: Move test once a dedicated struct for network name gets ported along with the client.
  #[test]
  fn invalid_new_with_network() {
    for input in INVALID_NETWORK_NAMES {
      assert!(matches!(StardustDID::new_with_network(input), Err(DIDError::Other(_))));
    }
  }

  #[test]
  fn valid_check_network() {
    let assert_check_network = |input: &str| {
      let did_core: CoreDID = CoreDID::parse(input).expect(&format!("expected {} to parse to a valid CoreDID", input));
      assert!(
        StardustDID::check_network(&did_core).is_ok(),
        "test: valid_check_network failed with input {}",
        input,
      );
    };

    for network_name in VALID_NETWORK_NAMES {
      let did_string = format!(
        "did:method:{}:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
        network_name
      );
      assert_check_network(&did_string);
    }

    assert_check_network("did:method:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK");
  }

  #[test]
  fn invalid_check_network() {
    // Loop over list of network names known to be invalid, attempt to create a CoreDID containing the given network
    // name in the method_id sub-string and ensure that `StardustDID::check_network` fails. If the provided network
    // name is in conflict with the DID Core spec itself then proceed to the next network name.

    // Ensure that this test is robust to changes in the supplied list of network names, i.e. fail if none of the
    // network names can be contained in a generic DIDCore.

    let mut check_network_executed: bool = false;

    for network_name in INVALID_NETWORK_NAMES {
      let did_string: String = format!(
        "did:method:{}:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
        network_name
      );
      let did_core: CoreDID = {
        match CoreDID::parse(&did_string) {
          Ok(did_core) => did_core,
          Err(_) => continue,
        }
      };

      assert!(matches!(StardustDID::check_network(&did_core), Err(DIDError::Other(_))));
      check_network_executed = true;
    }
    assert!(
      check_network_executed,
      "test: invalid_check_network never executes `StardustDID::check_network`"
    );
  }

  #[test]
  fn test_normalize() {
    let did_with_default_network_string: String = format!(
      "did:stardust:{}:{}",
      StardustDID::DEFAULT_NETWORK,
      VALID_ALIAS_ID_STRING.as_str()
    );
    let core_did_not_normalized: CoreDID = CoreDID::parse(did_with_default_network_string).unwrap();

    let expected_normalization_string_representation: String =
      format!("did:stardust:{}", VALID_ALIAS_ID_STRING.as_str());

    assert_eq!(
      StardustDID::normalize(core_did_not_normalized).as_str(),
      &expected_normalization_string_representation
    );
  }

  #[test]
  fn valid_check_method_id() {
    for input in VALID_STARDUST_DID_STRINGS.iter() {
      let did_core: CoreDID = CoreDID::parse(input).unwrap();
      assert!(
        StardustDID::check_method_id(&did_core).is_ok(),
        "test: valid_check_method_id failed on input {}",
        input
      );
    }

    // Should also work for DID's of the form: did:<method_name>:<valid_stardust_network (or
    // nothing/normalized)>:<alias_id>
    let did_other_string: String = format!("did:method:{}", VALID_STARDUST_DID_STRING.as_str());
    let did_other_with_network: String = format!("did:method:test:{}", VALID_STARDUST_DID_STRING.as_str());
    let did_other_core: CoreDID = CoreDID::parse(&did_other_string).unwrap();
    let did_other_with_network_core: CoreDID = CoreDID::parse(&did_other_with_network).unwrap();

    assert!(StardustDID::check_method_id(&did_other_core).is_ok());
    assert!(StardustDID::check_method_id(&did_other_with_network_core).is_ok());
  }
  // TODO: Write test invalid_check_method_id
}
