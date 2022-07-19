// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::convert::TryInto;

use iota_client::bee_block::output::AliasId;
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
static PLACHEHOLDER_DID_STR: Lazy<String> = Lazy::new(|| format!("did:stardust:{}", INITIAL_ALIAS_ID));
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

    Ok(Self(Self::normalize(did)))
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
    CoreDID::parse(&format!("did:stardust:{}:{}", network, INITIAL_ALIAS_ID))
      .map(Self::normalize)
      .map(Self)
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
      did
        .set_method_id(tag)
        .expect("the extracted normalized tag should satisfy the DID Core specification");
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
    Self::check_method(did).and_then(|_| Self::check_method_id(did))
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

  /// Returns a `bool` indicating if the given `DID` is valid according to the
  /// [`StardustDID`] method specification.
  ///
  /// Equivalent to `Self::check_validity(did).is_ok()`.
  pub fn is_valid(did: &CoreDID) -> bool {
    Self::check_validity(did).is_ok()
  }

  /// Returns the Tangle `network` name of the `DID`.
  pub fn network_str(&self) -> &str {
    let mut segments_iter = self.method_id().split(":");
    match (segments_iter.next(), segments_iter.next()) {
      (Some(network), Some(tag)) => network,
      // DID network must have been truncated during normalization in the constructor in all other cases
      _ => Self::DEFAULT_NETWORK,
    }
  }

  /// Returns the unique Tangle tag of the `DID`.
  pub fn tag(&self) -> &str {
    let mut segments_iter = self.method_id().split(":");
    match (segments_iter.next(), segments_iter.next()) {
      (Some(_), Some(tag)) => tag,
      // guaranteed by constructors to be a valid tag
      (Some(tag), None) => tag,
      _ => unreachable!("a {} DID should have a tag", StardustDID::METHOD),
    }
  }

  /// Change the network name of this [`StardustDID`] leaving all other segments (did, method, tag) intact.  
  //
  // TODO: Either change this method to take a network (or network name) once that has been ported with the `Client` and
  // make this method public or remove in favour of another constructor `new_with_network_and_alias`.
  //
  // Also consider replacing this with a `setter` in order to be more similar to the JS API.
  fn with_network(network: &str) -> Self {
    todo!("implement this once the network/network name has been ported")
  }
}

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

impl FromStr for StardustDID {
  type Err = DIDError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Self::parse(s)
  }
}

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

impl From<AliasId> for StardustDID {
  /// Transforms an [`AliasId`] to a [`StardustDID`].
  ///
  /// # Network
  /// The [`StardustDID`] constructed from this method is assumed to be associated with the default network,
  /// whenever that is not the case one should follow this up with calling [`StardustDID::with_network`].  
  fn from(id: AliasId) -> Self {
    let did_str = format!("did:{}:{}", StardustDID::METHOD, id);
    Self::parse(did_str).expect(&format!(
      "transforming an AliasId to a {} DID should be infallible",
      StardustDID::METHOD
    ))
  }
}

impl From<StardustDID> for AliasId {
  fn from(did: StardustDID) -> Self {
    Self::from_str(did.tag()).expect(&format!(
      "the tag of a {} DID should always parse to an AliasId",
      StardustDID::METHOD
    ))
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

  use std::sync::atomic::AtomicUsize;
  use std::sync::atomic::Ordering;

  use iota_client::bee_block::output::AliasId;
  use iota_client::bee_block::output::OutputId;
  use iota_client::bee_block::output::OUTPUT_INDEX_RANGE;
  use iota_client::bee_block::payload::transaction::TransactionId;
  use proptest::strategy::Strategy;
  use proptest::*;

  use super::*;

  // obtained AliasID from a valid OutputID string
  // output_id copied from https://github.com/iotaledger/bee/blob/30cab4f02e9f5d72ffe137fd9eb09723b4f0fdb6/bee-block/tests/output_id.rs
  // value of AliasID computed from AliasId::from(OutputId)
  const VALID_ALIAS_ID_STRING: &str = "0xf29dd16310c2100fd1bf568b345fb1cc14d71caa3bd9b5ad735d2bd6d455ca3b";

  static VALID_STARDUST_DID_STRING: Lazy<String> =
    Lazy::new(|| format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING));

  static VALID_STARDUST_DID_STRINGS: Lazy<Vec<String>> = Lazy::new(|| {
    let network_tag_to_did = |network, tag| format!("did:{}:{}:{}", StardustDID::METHOD, network, tag);

    vec![
      network_tag_to_did("main", VALID_ALIAS_ID_STRING),
      network_tag_to_did("dev", VALID_ALIAS_ID_STRING),
      network_tag_to_did("smr", VALID_ALIAS_ID_STRING),
      network_tag_to_did("rms", VALID_ALIAS_ID_STRING),
      network_tag_to_did("test", VALID_ALIAS_ID_STRING),
      format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING),
      network_tag_to_did("main", INITIAL_ALIAS_ID),
      network_tag_to_did("dev", INITIAL_ALIAS_ID),
      network_tag_to_did("smr", INITIAL_ALIAS_ID),
      network_tag_to_did("rms", INITIAL_ALIAS_ID),
      network_tag_to_did("test", INITIAL_ALIAS_ID),
      format!("did:{}:{}", StardustDID::METHOD, INITIAL_ALIAS_ID),
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
  fn normalization_in_constructors() {
    let did_with_default_network_string: String = format!(
      "did:{}:{}:{}",
      StardustDID::METHOD,
      StardustDID::DEFAULT_NETWORK,
      VALID_ALIAS_ID_STRING
    );
    let expected_normalization_string_representation: String =
      format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING);

    assert_eq!(
      StardustDID::parse(did_with_default_network_string).unwrap().as_str(),
      expected_normalization_string_representation
    );

    assert_eq!(
      StardustDID::new_with_network(StardustDID::DEFAULT_NETWORK)
        .unwrap()
        .as_str(),
      format!("did:stardust:{}", INITIAL_ALIAS_ID)
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
    let did_other_string: String = format!("did:method:{}", VALID_ALIAS_ID_STRING);
    let did_other_with_network: String = format!("did:method:test:{}", VALID_ALIAS_ID_STRING);
    let did_other_core: CoreDID = CoreDID::parse(&did_other_string).unwrap();
    let did_other_with_network_core: CoreDID = CoreDID::parse(&did_other_with_network).unwrap();

    assert!(StardustDID::check_method_id(&did_other_core).is_ok());
    assert!(StardustDID::check_method_id(&did_other_with_network_core).is_ok());
  }

  #[test]
  fn invalid_check_method_id() {
    let invalid_method_id_strings = [
      // Invalid network name
      format!("did:method:1234567:{}", VALID_ALIAS_ID_STRING),
      // Too many segments
      format!("did:method:main:test:{}", VALID_ALIAS_ID_STRING),
      // Tag is not prefixed
      format!("did:method:{}", &VALID_ALIAS_ID_STRING.strip_prefix("0x").unwrap()),
      // Tag is too long
      format!(
        "did:method:{}",
        &VALID_ALIAS_ID_STRING.chars().chain("a".chars()).collect::<String>()
      ),
      // Tag is too short (omit last character)
      format!("did:method:main:{}", &VALID_ALIAS_ID_STRING[..65]),
    ];

    for input in invalid_method_id_strings {
      let did_core: CoreDID = CoreDID::parse(input).unwrap();
      assert!(matches!(
        StardustDID::check_method_id(&did_core),
        Err(DIDError::InvalidMethodId)
      ));
    }
  }

  #[test]
  fn parse_valid() {
    for did_str in VALID_STARDUST_DID_STRINGS.iter() {
      assert!(StardustDID::parse(&did_str).is_ok());
    }
  }

  #[test]

  fn parse_invalid() {
    assert!(matches!(
      StardustDID::parse(format!("dod:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)),
      Err(DIDError::InvalidScheme)
    ));

    assert!(matches!(
      StardustDID::parse(format!("did:key:{}", VALID_ALIAS_ID_STRING)),
      Err(DIDError::InvalidMethodName)
    ));

    // invalid network name (exceeded six characters)
    assert!(matches!(
      StardustDID::parse(format!("did:{}:1234567:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)),
      Err(DIDError::InvalidMethodId)
    ));

    // invalid network name (contains non ascii character é)
    assert!(matches!(
      StardustDID::parse(format!("did:{}:féta:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)),
      Err(DIDError::InvalidMethodId)
    ));

    // invalid tag
    assert!(matches!(
      StardustDID::parse(format!("did:{}:", StardustDID::METHOD)),
      Err(DIDError::InvalidMethodId)
    ));

    // too many segments in method_id
    assert!(matches!(
      StardustDID::parse(format!(
        "did:{}:test:foo:{}",
        StardustDID::METHOD,
        VALID_ALIAS_ID_STRING
      )),
      Err(DIDError::InvalidMethodId)
    ));
  }

  fn arbitrary_alias_id() -> impl Strategy<Value = AliasId> {
    (proptest::prelude::any::<[u8; 32]>(), OUTPUT_INDEX_RANGE).prop_map(|(bytes, idx)| {
      let transaction_id: TransactionId = TransactionId::new(bytes);
      let output_id: OutputId = OutputId::new(transaction_id, idx).unwrap();
      AliasId::from(output_id)
    })
  }

  proptest! {
    #[test]
    fn property_based_valid_parse(alias_id in arbitrary_alias_id()) {
      let did: String = format!("did:{}:{}",StardustDID::METHOD, alias_id.to_string());
      assert!(StardustDID::parse(&did).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_alias_id_roundtrip(alias_id in arbitrary_alias_id()) {
      assert_eq!(
        AliasId::from(StardustDID::from(alias_id.clone())),
        alias_id
      );
    }
  }

  #[test]
  fn test_network() {
    let did: StardustDID = format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.network_str(), StardustDID::DEFAULT_NETWORK);

    let did: StardustDID = format!("did:{}:dev:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.network_str(), "dev");

    let did: StardustDID = format!("did:{}:test:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.network_str(), "test");

    let did: StardustDID = format!("did:{}:custom:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.network_str(), "custom");
  }

  #[test]
  fn test_tag() {
    let did: StardustDID = format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.tag(), VALID_ALIAS_ID_STRING);

    let did: StardustDID = format!("did:{}:main:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)
      .parse()
      .unwrap();
    assert_eq!(did.tag(), VALID_ALIAS_ID_STRING);
  }

  #[test]
  fn test_parse_did_url_valid() {
    assert!(StardustDIDUrl::parse(format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)).is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:{}#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:{}?somequery=somevalue",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:{}?somequery=somevalue#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());

    assert!(StardustDIDUrl::parse(format!("did:{}:main:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)).is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:main:{}#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:main:{}?somequery=somevalue",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:main:{}?somequery=somevalue#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());

    assert!(StardustDIDUrl::parse(format!("did:{}:dev:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)).is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:dev:{}#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:dev:{}?somequery=somevalue",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:dev:{}?somequery=somevalue#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());

    assert!(StardustDIDUrl::parse(format!("did:{}:custom:{}", StardustDID::METHOD, VALID_ALIAS_ID_STRING)).is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:custom:{}#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:custom:{}?somequery=somevalue",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
    assert!(StardustDIDUrl::parse(format!(
      "did:{}:custom:{}?somequery=somevalue#fragment",
      StardustDID::METHOD,
      VALID_ALIAS_ID_STRING
    ))
    .is_ok());
  }

  #[test]
  fn test_setter() {
    let mut did_url: StardustDIDUrl = StardustDID::parse(VALID_STARDUST_DID_STRING.as_str())
      .unwrap()
      .into_url();

    did_url.set_path(Some("/foo")).unwrap();
    did_url.set_query(Some("diff=true")).unwrap();
    did_url.set_fragment(Some("foo")).unwrap();

    assert_eq!(did_url.path(), Some("/foo"));
    assert_eq!(did_url.query(), Some("diff=true"));
    assert_eq!(did_url.fragment(), Some("foo"));
  }
}
