// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use identity_core::common::KeyComparable;
use serde::Deserialize;
use serde::Serialize;

use identity_did::did::BaseDIDUrl;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use crate::NetworkName;

pub type Result<T> = std::result::Result<T, DIDError>;

// The length of an AliasID, which is a BLAKE2b-256 hash (32-bytes).
const TAG_BYTES_LEN: usize = 32;

/// A DID conforming to the IOTA UTXO DID method specification.
///
/// This is a thin wrapper around the [`DID`][`CoreDID`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
pub struct StardustDID(CoreDID);

impl StardustDID {
  /// The URL scheme for Decentralized Identifiers.
  pub const SCHEME: &'static str = CoreDID::SCHEME;

  /// The IOTA UTXO DID method name (`"stardust"`).
  // TODO: This will be changed to `iota` in the future.
  pub const METHOD: &'static str = "stardust";

  /// The default Tangle network (`"main"`).
  pub const DEFAULT_NETWORK: &'static str = "main";

  /// Converts an owned [`CoreDID`] to a [`StardustDID`].
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not conform to the [`StardustDID`] specification.
  pub fn try_from_core(did: CoreDID) -> Result<Self> {
    Self::check_validity(&did)?;

    Ok(Self(Self::normalize(did)))
  }

  /// Constructs a new [`StardustDID`] from a byte representation of the tag and the given network name.
  ///
  /// See also [`StardustDID::placeholder`].
  ///
  /// # Example
  ///
  /// ```
  /// # use identity_did::did::DID;
  /// # use identity_stardust::NetworkName;
  /// # use identity_stardust::StardustDID;
  /// #
  /// let did = StardustDID::new(&[1;32], &NetworkName::try_from("smr").unwrap());
  /// assert_eq!(did.as_str(), "did:stardust:smr:0x0101010101010101010101010101010101010101010101010101010101010101");
  pub fn new(bytes: &[u8; 32], network_name: &NetworkName) -> Self {
    let tag = prefix_hex::encode(bytes);
    let did: String = format!("did:{}:{}:{}", Self::METHOD, network_name, tag);

    Self::parse(did).expect("DIDs constructed with new should be valid")
  }

  /// Parses an [`StardustDID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not conform to the [`StardustDID`] specification.
  pub fn parse(input: impl AsRef<str>) -> Result<Self> {
    CoreDID::parse(input).and_then(Self::try_from_core)
  }

  /// Creates a new placeholder [`StardustDID`] with the given network name.
  ///
  /// # Example
  ///
  /// ```
  /// # use identity_did::did::DID;
  /// # use identity_stardust::NetworkName;
  /// # use identity_stardust::StardustDID;
  /// #
  /// let placeholder = StardustDID::placeholder(&NetworkName::try_from("smr").unwrap());
  /// assert_eq!(placeholder.as_str(), "did:stardust:smr:0x0000000000000000000000000000000000000000000000000000000000000000");
  pub fn placeholder(network_name: &NetworkName) -> Self {
    Self::new(&[0; 32], network_name)
  }

  // Check if the tag matches a potential alias_id
  fn check_tag_str(tag: &str) -> Result<()> {
    prefix_hex::decode::<[u8; TAG_BYTES_LEN]>(tag)
      .map_err(|_| DIDError::InvalidMethodId)
      .map(|_| ())
  }

  // Normalizes the DID `method_id` by removing the default network segment if present.
  //
  // E.g.
  // - `"did:stardust:main:123" -> "did:stardust:123"` is normalized
  // - `"did:stardust:dev:123" -> "did:stardust:dev:123"` is unchanged
  // TODO: Remove the lint once this bug in clippy has been fixed. Without to_owned a mutable reference will be aliased.
  #[allow(clippy::unnecessary_to_owned)]
  fn normalize(mut did: CoreDID) -> CoreDID {
    let method_id = did.method_id();
    let (network, tag) = Self::denormalized_components(method_id);
    if tag.len() == method_id.len() || network != Self::DEFAULT_NETWORK {
      did
    } else {
      did
        .set_method_id(tag.to_owned())
        .expect("normalizing a valid CoreDID should be Ok");
      did
    }
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] network name.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid network name according to the [`StardustDID`] method specification.
  pub fn check_network<D: DID>(did: &D) -> Result<()> {
    let network_name = Self::denormalized_components(did.method_id()).0;
    NetworkName::validate_network_name(network_name).map_err(|_| DIDError::Other("invalid network name"))
  }

  /// Checks if the given `DID` is syntactically valid according to the [`StardustDID`] method specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a syntactically valid [`StardustDID`].
  pub fn check_validity<D: DID>(did: &D) -> Result<()> {
    Self::check_method(did).and_then(|_| Self::check_method_id(did))
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] `method` (i.e. `"stardust"`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input represents another method.
  // TODO: Change the naming in the docs once we remove the code for the current IOTA method.
  pub fn check_method<D: DID>(did: &D) -> Result<()> {
    (did.method() == Self::METHOD)
      .then_some(())
      .ok_or(DIDError::InvalidMethodName)
  }

  /// Checks if the given `DID` has a valid [`StardustDID`] `method_id`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not have a [`StardustDID`] compliant method id.
  pub fn check_method_id<D: DID>(did: &D) -> Result<()> {
    let (network, tag) = Self::denormalized_components(did.method_id());
    NetworkName::validate_network_name(network)
      .map_err(|_| DIDError::InvalidMethodId)
      .and_then(|_| Self::check_tag_str(tag))
  }

  /// Returns a `bool` indicating if the given `DID` is valid according to the
  /// [`StardustDID`] method specification.
  ///
  /// Equivalent to `Self::check_validity(did).is_ok()`.
  pub fn is_valid(did: &CoreDID) -> bool {
    Self::check_validity(did).is_ok()
  }

  /// Returns the IOTA `network` name of the `DID`.
  pub fn network_str(&self) -> &str {
    Self::denormalized_components(self.method_id()).0
  }

  /// foo:bar -> (foo,bar)
  /// foo:bar:baz -> (foo, bar:baz)
  /// foo -> (StardustDID::DEFAULT_NETWORK.as_ref(), foo)
  #[inline(always)]
  fn denormalized_components(input: &str) -> (&str, &str) {
    input
      .find(':')
      .map(|idx| input.split_at(idx))
      .map(|(network, tail)| (network, &tail[1..]))
      // Self::DEFAULT_NETWORK is built from a static reference so unwrapping is fine
      .unwrap_or((Self::DEFAULT_NETWORK, input))
  }

  /// Returns the unique tag of the `DID`.
  pub fn tag(&self) -> &str {
    Self::denormalized_components(self.method_id()).1
  }

  /// Replace the network name of this [`StardustDID`] leaving all other segments (did, method, tag) intact.  
  pub fn with_network_name(mut self, name: NetworkName) -> Self {
    let new_method_id: String = format!("{}:{}", name, self.tag());
    // unwrap is fine as we are only replacing the network
    self.0.set_method_id(new_method_id).unwrap();
    self
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

  // TODO: Link [`StardustDIDUrl`] after `document` has been refactored to use the types in this module.
  /// Creates a new DIDUrl by joining with a relative DID Url string.
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

impl TryFrom<&str> for StardustDID {
  type Error = DIDError;

  fn try_from(other: &str) -> std::result::Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl TryFrom<String> for StardustDID {
  type Error = DIDError;

  fn try_from(other: String) -> std::result::Result<Self, Self::Error> {
    Self::parse(other)
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

impl From<StardustDID> for String {
  fn from(did: StardustDID) -> Self {
    did.into_string()
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

impl KeyComparable for StardustDID {
  type Key = CoreDID;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.as_ref()
  }
}

#[cfg(test)]
mod tests {

  use iota_client::block::output::AliasId;
  use iota_client::block::output::OutputId;
  use iota_client::block::output::OUTPUT_INDEX_RANGE;
  use iota_client::block::payload::transaction::TransactionId;
  use once_cell::sync::Lazy;
  use proptest::strategy::Strategy;
  use proptest::*;

  use super::*;

  const INITIAL_ALIAS_ID_STR: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

  // ===========================================================================================================================
  // Reusable constants and statics
  // ===========================================================================================================================

  // obtained AliasID from a valid OutputID string
  // output_id copied from https://github.com/iotaledger/bee/blob/30cab4f02e9f5d72ffe137fd9eb09723b4f0fdb6/bee-block/tests/output_id.rs
  // value of AliasID computed from AliasId::from(OutputId).to_string()
  const VALID_ALIAS_ID_STR: &str = "0xf29dd16310c2100fd1bf568b345fb1cc14d71caa3bd9b5ad735d2bd6d455ca3b";

  const LEN_VALID_ALIAS_STR: usize = VALID_ALIAS_ID_STR.len();

  static VALID_STARDUST_DID_STRING: Lazy<String> =
    Lazy::new(|| format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STR));

  // Rules are: at least one character, at most six characters and may only contain digits and/or lowercase ascii
  // characters.
  const VALID_NETWORK_NAMES: [&str; 13] = [
    StardustDID::DEFAULT_NETWORK,
    "main",
    "dev",
    "smr",
    "rms",
    "test",
    "foo",
    "foobar",
    "123456",
    "0",
    "foo42",
    "bar123",
    "42foo",
  ];

  const INVALID_NETWORK_NAMES: [&str; 10] = [
    "Main", "fOo", "deV", "féta", "", "  ", "foo ", " foo", "1234567", "foobar0",
  ];

  static VALID_STARDUST_DID_STRINGS: Lazy<Vec<String>> = Lazy::new(|| {
    let network_tag_to_did = |network, tag| format!("did:{}:{}:{}", StardustDID::METHOD, network, tag);

    let valid_strings: Vec<String> = VALID_NETWORK_NAMES
      .iter()
      .flat_map(|network| {
        [VALID_ALIAS_ID_STR, INITIAL_ALIAS_ID_STR]
          .iter()
          .map(move |tag| network_tag_to_did(network, tag))
      })
      .collect();

    // in principle the previous binding is not necessary (we could have just returned the value),
    // but let's just ensure that it contains the expected number of elements first.
    assert_eq!(valid_strings.len(), 2 * VALID_NETWORK_NAMES.len());

    valid_strings
  });

  // ===========================================================================================================================
  // Test check_* methods
  // ===========================================================================================================================

  #[test]
  fn invalid_check_method() {
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

  #[test]
  fn valid_check_network() {
    let assert_check_network = |input: &str| {
      let did_core: CoreDID =
        CoreDID::parse(input).unwrap_or_else(|_| panic!("expected {} to parse to a valid CoreDID", input));
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
    // network names can be contained in a generic CoreDID.

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
      "StardustDID::check_network` should have been executed"
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
    let did_other_string: String = format!("did:method:{}", VALID_ALIAS_ID_STR);
    let did_other_with_network: String = format!("did:method:test:{}", VALID_ALIAS_ID_STR);
    let did_other_core: CoreDID = CoreDID::parse(&did_other_string).unwrap();
    let did_other_with_network_core: CoreDID = CoreDID::parse(&did_other_with_network).unwrap();

    assert!(StardustDID::check_method_id(&did_other_core).is_ok());
    assert!(StardustDID::check_method_id(&did_other_with_network_core).is_ok());
  }

  #[test]
  fn invalid_check_method_id() {
    let invalid_method_id_strings = [
      // Invalid network name
      format!("did:method:1234567:{}", VALID_ALIAS_ID_STR),
      // Too many segments
      format!("did:method:main:test:{}", VALID_ALIAS_ID_STR),
      // Tag is not prefixed
      format!("did:method:{}", &VALID_ALIAS_ID_STR.strip_prefix("0x").unwrap()),
      // Tag is too long
      format!(
        "did:method:{}",
        &VALID_ALIAS_ID_STR.chars().chain("a".chars()).collect::<String>()
      ),
      // Tag is too short (omit last character)
      format!("did:method:main:{}", &VALID_ALIAS_ID_STR[..65]),
    ];

    for input in invalid_method_id_strings {
      let did_core: CoreDID = CoreDID::parse(input).unwrap();
      assert!(matches!(
        StardustDID::check_method_id(&did_core),
        Err(DIDError::InvalidMethodId)
      ));
    }
  }

  // ===========================================================================================================================
  // Test constructors
  // ===========================================================================================================================

  #[test]
  fn placeholder_produces_a_did_with_expected_string_representation() {
    assert_eq!(
      StardustDID::placeholder(&NetworkName::try_from(StardustDID::DEFAULT_NETWORK).unwrap()).as_str(),
      format!("did:{}:{}", StardustDID::METHOD, INITIAL_ALIAS_ID_STR)
    );

    for name in VALID_NETWORK_NAMES
      .iter()
      .filter(|name| *name != &StardustDID::DEFAULT_NETWORK)
    {
      let network_name: NetworkName = NetworkName::try_from(*name).unwrap();
      let did: StardustDID = StardustDID::placeholder(&network_name);
      assert_eq!(
        did.as_str(),
        format!("did:{}:{}:{}", StardustDID::METHOD, name, INITIAL_ALIAS_ID_STR)
      );
    }
  }

  #[test]
  fn normalization_in_constructors() {
    let did_with_default_network_string: String = format!(
      "did:{}:{}:{}",
      StardustDID::METHOD,
      StardustDID::DEFAULT_NETWORK,
      VALID_ALIAS_ID_STR
    );
    let expected_normalization_string_representation: String =
      format!("did:{}:{}", StardustDID::METHOD, VALID_ALIAS_ID_STR);

    assert_eq!(
      StardustDID::parse(did_with_default_network_string).unwrap().as_str(),
      expected_normalization_string_representation
    );
  }

  #[test]
  fn parse_valid() {
    for did_str in VALID_STARDUST_DID_STRINGS.iter() {
      assert!(StardustDID::parse(&did_str).is_ok());
    }
  }

  #[test]
  fn parse_invalid() {
    let execute_assertions = |valid_alias_id: &str| {
      assert!(matches!(
        StardustDID::parse(format!("dod:{}:{}", StardustDID::METHOD, valid_alias_id)),
        Err(DIDError::InvalidScheme)
      ));

      assert!(matches!(
        StardustDID::parse(format!("did:key:{}", valid_alias_id)),
        Err(DIDError::InvalidMethodName)
      ));

      // invalid network name (exceeded six characters)
      assert!(matches!(
        StardustDID::parse(format!("did:{}:1234567:{}", StardustDID::METHOD, valid_alias_id)),
        Err(DIDError::InvalidMethodId)
      ));

      // invalid network name (contains non ascii character é)
      assert!(matches!(
        StardustDID::parse(format!("did:{}:féta:{}", StardustDID::METHOD, valid_alias_id)),
        Err(DIDError::InvalidMethodId)
      ));

      // invalid tag
      assert!(matches!(
        StardustDID::parse(format!("did:{}:", StardustDID::METHOD)),
        Err(DIDError::InvalidMethodId)
      ));

      // too many segments in method_id
      assert!(matches!(
        StardustDID::parse(format!("did:{}:test:foo:{}", StardustDID::METHOD, valid_alias_id)),
        Err(DIDError::InvalidMethodId)
      ));
    };

    execute_assertions(INITIAL_ALIAS_ID_STR);
    execute_assertions(VALID_ALIAS_ID_STR);
  }

  // ===========================================================================================================================
  // Test constructors with randomly generated input
  // ===========================================================================================================================

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
      let did: String = format!("did:{}:{}",StardustDID::METHOD, alias_id);
      assert!(StardustDID::parse(&did).is_ok());
    }
  }

  proptest! {
    #[test]
    fn property_based_new(bytes in proptest::prelude::any::<[u8;32]>()) {
      for network_name in VALID_NETWORK_NAMES.iter().map(|name| NetworkName::try_from(*name).unwrap()) {
        // check that this does not panic
        StardustDID::new(&bytes, &network_name);
      }
    }
  }

  proptest! {
    #[test]
    fn property_based_alias_id_string_representation_roundtrip(alias_id in arbitrary_alias_id()) {
      for network_name in VALID_NETWORK_NAMES.iter().map(|name| NetworkName::try_from(*name).unwrap()) {
        assert_eq!(
          AliasId::from_str(StardustDID::new(&alias_id, &network_name).tag()).unwrap(),
          alias_id
        );
      }
    }
  }

  fn arbitrary_alias_id_string_replica() -> impl Strategy<Value = String> {
    proptest::string::string_regex(&format!("0x([a-f]|[0-9]){{{}}}", (LEN_VALID_ALIAS_STR - 2)))
      .expect("regex should be ok")
  }

  proptest! {
    #[test]
    fn valid_alias_id_string_replicas(tag in arbitrary_alias_id_string_replica()) {
      let did : String = format!("did:{}:{}", StardustDID::METHOD, tag);
      assert!(
        StardustDID::parse(did).is_ok()
      );
    }
  }

  fn arbitrary_invalid_tag() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[[:^alpha:]|[a-z]|[1-9]]*")
      .expect("regex should be ok")
      .prop_map(|arb_string| {
        if arb_string
          .chars()
          .all(|c| c.is_ascii_hexdigit() && c.is_ascii_lowercase())
          && arb_string.len() == LEN_VALID_ALIAS_STR
          && arb_string.starts_with("0x")
        {
          // this means we are in the rare case of generating a valid string hence we replace the last 0 with the non
          // ascii character é
          let mut counter = 0;
          arb_string
            .chars()
            .rev()
            .map(|value| {
              if value == '0' && counter == 0 {
                counter += 1;
                'é'
              } else {
                value
              }
            })
            .collect::<String>()
        } else {
          arb_string
        }
      })
  }

  proptest! {
    #[test]
    fn invalid_tag_property_based_parse(tag in arbitrary_invalid_tag()) {
      let did: String = format!("did:{}:{}", StardustDID::METHOD, tag);
      assert!(
        StardustDID::parse(did).is_err()
      );
    }
  }

  fn arbitrary_delimiter_mixed_in_prefix_hex() -> impl Strategy<Value = String> {
    proptest::string::string_regex("0x([a-f]|[:]|[0-9])*").expect("regex should be ok")
  }

  proptest! {
    #[test]
    fn invalid_hex_mixed_with_delimiter(tag in arbitrary_delimiter_mixed_in_prefix_hex()) {
      let did: String = format!("did:{}:{}", StardustDID::METHOD, tag);
      assert!(StardustDID::parse(did).is_err());
    }
  }

  // ===========================================================================================================================
  // Test getters
  // ===========================================================================================================================
  #[test]
  fn test_network() {
    let execute_assertions = |valid_alias_id: &str| {
      let did: StardustDID = format!("did:{}:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.network_str(), StardustDID::DEFAULT_NETWORK);

      let did: StardustDID = format!("did:{}:dev:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.network_str(), "dev");

      let did: StardustDID = format!("did:{}:test:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.network_str(), "test");

      let did: StardustDID = format!("did:{}:custom:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.network_str(), "custom");
    };

    execute_assertions(INITIAL_ALIAS_ID_STR);
    execute_assertions(VALID_ALIAS_ID_STR);
  }

  #[test]
  fn test_tag() {
    let execute_assertions = |valid_alias_id: &str| {
      let did: StardustDID = format!("did:{}:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.tag(), valid_alias_id);

      let did: StardustDID = format!(
        "did:{}:{}:{}",
        StardustDID::METHOD,
        StardustDID::DEFAULT_NETWORK,
        valid_alias_id
      )
      .parse()
      .unwrap();
      assert_eq!(did.tag(), valid_alias_id);

      let did: StardustDID = format!("did:{}:dev:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.tag(), valid_alias_id);

      let did: StardustDID = format!("did:{}:custom:{}", StardustDID::METHOD, valid_alias_id)
        .parse()
        .unwrap();
      assert_eq!(did.tag(), valid_alias_id);
    };
    execute_assertions(INITIAL_ALIAS_ID_STR);
    execute_assertions(VALID_ALIAS_ID_STR);
  }

  // ===========================================================================================================================
  // Test setters
  // ===========================================================================================================================

  #[test]
  fn replace_network_name() {
    for did in VALID_STARDUST_DID_STRINGS.iter() {
      let stardust_did: StardustDID = StardustDID::parse(did).unwrap();
      for name in VALID_NETWORK_NAMES {
        let old_tag: String = stardust_did.tag().to_string();
        let network_name: NetworkName = NetworkName::try_from(name).unwrap();
        let transfromed: StardustDID = stardust_did.clone().with_network_name(network_name.clone());
        assert_eq!(old_tag, transfromed.tag());
        assert_eq!(transfromed.network_str(), name);
      }
    }
  }

  // ===========================================================================================================================
  // Test DIDUrl
  // ===========================================================================================================================

  // TODO: Move `StardustDIDUrl` out of this test module once the `document` module gets refactored to use the types
  // from this module.
  /// A DID URL conforming to the IOTA Stardust UTXO DID method specification.
  ///
  /// See [`DIDUrl`].
  type StardustDIDUrl = DIDUrl<StardustDID>;
  #[test]
  fn test_parse_did_url_valid() {
    let execute_assertions = |valid_alias_id: &str| {
      assert!(StardustDIDUrl::parse(format!("did:{}:{}", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!("did:{}:{}#fragment", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:{}?somequery=somevalue",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:{}?somequery=somevalue#fragment",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());

      assert!(StardustDIDUrl::parse(format!("did:{}:main:{}", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!("did:{}:main:{}#fragment", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:main:{}?somequery=somevalue",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:main:{}?somequery=somevalue#fragment",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());

      assert!(StardustDIDUrl::parse(format!("did:{}:dev:{}", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!("did:{}:dev:{}#fragment", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:dev:{}?somequery=somevalue",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:dev:{}?somequery=somevalue#fragment",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());

      assert!(StardustDIDUrl::parse(format!("did:{}:custom:{}", StardustDID::METHOD, valid_alias_id)).is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:custom:{}#fragment",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:custom:{}?somequery=somevalue",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
      assert!(StardustDIDUrl::parse(format!(
        "did:{}:custom:{}?somequery=somevalue#fragment",
        StardustDID::METHOD,
        valid_alias_id
      ))
      .is_ok());
    };
    execute_assertions(INITIAL_ALIAS_ID_STR);
    execute_assertions(VALID_ALIAS_ID_STR);
  }

  #[test]
  fn valid_url_setters() {
    let execute_assertions = |valid_alias_id: &str| {
      let mut did_url: StardustDIDUrl = StardustDID::parse(format!("did:{}:{}", StardustDID::METHOD, valid_alias_id))
        .unwrap()
        .into_url();

      did_url.set_path(Some("/foo")).unwrap();
      did_url.set_query(Some("diff=true")).unwrap();
      did_url.set_fragment(Some("foo")).unwrap();

      assert_eq!(did_url.path(), Some("/foo"));
      assert_eq!(did_url.query(), Some("diff=true"));
      assert_eq!(did_url.fragment(), Some("foo"));
    };
    execute_assertions(INITIAL_ALIAS_ID_STR);
    execute_assertions(VALID_ALIAS_ID_STR);
  }
}
