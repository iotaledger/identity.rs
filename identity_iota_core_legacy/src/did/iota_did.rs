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
use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::KeyComparable;
use identity_core::utils::BaseEncoding;
use identity_did::did::BaseDIDUrl;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;
use identity_did::did::DIDUrl;
use identity_did::did::DID;

use crate::did::Segments;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Network;
use crate::tangle::NetworkName;
use crate::try_construct_did;

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

/// A DID URL conforming to the IOTA DID method specification.
///
/// See [`DIDUrl`].
pub type IotaDIDUrl = DIDUrl<IotaDID>;

/// A DID conforming to the IOTA DID method specification.
///
/// This is a thin wrapper around the [`DID`][`CoreDID`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "CoreDID", try_from = "CoreDID")]
pub struct IotaDID(CoreDID);

impl IotaDID {
  /// The URL scheme for Decentralized Identifiers.
  pub const SCHEME: &'static str = CoreDID::SCHEME;

  /// The IOTA DID method name (`"iota"`).
  pub const METHOD: &'static str = "iota";

  /// The default Tangle network (`"main"`).
  pub const DEFAULT_NETWORK: &'static str = "main";

  /// Converts an owned [`CoreDID`] to an [`IotaDID`].
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn try_from_core(did: CoreDID) -> Result<Self> {
    Self::check_validity(&did)?;

    Ok(Self(Self::normalize(did)))
  }

  /// Parses an [`IotaDID`] from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn parse(input: impl AsRef<str>) -> Result<Self> {
    CoreDID::parse(input).map_err(Into::into).and_then(Self::try_from_core)
  }

  /// Creates a new [`IotaDID`] with a tag derived from the given `public` key.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not form a valid [`IotaDID`].
  pub fn new(public: &[u8]) -> Result<Self> {
    try_construct_did!(public).map_err(Into::into)
  }

  /// Creates a new [`IotaDID`] from the given `public` key and `network`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not form a valid [`IotaDID`] or the `network` is invalid.
  /// See [`NetworkName`] for validation requirements.
  pub fn new_with_network(public: &[u8], network: impl TryInto<NetworkName>) -> Result<Self> {
    let network_name = network.try_into().map_err(|_| Error::InvalidNetworkName)?;
    try_construct_did!(public, network_name.as_ref()).map_err(Into::into)
  }

  /// Checks if the given `DID` has a valid IOTA DID `method` (i.e. `"iota"`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_method<D: DID>(did: &D) -> Result<()> {
    if did.method() != Self::METHOD {
      Err(Error::InvalidDID(DIDError::InvalidMethodName))
    } else {
      Ok(())
    }
  }

  /// Checks if the given `DID` has a valid [`IotaDID`] `method_id`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_method_id<D: DID>(did: &D) -> Result<()> {
    let segments: Vec<&str> = did.method_id().split(':').collect();

    if segments.is_empty() || segments.len() > 2 {
      return Err(Error::InvalidDID(DIDError::InvalidMethodId));
    }

    // We checked if `id_segments` was empty so this should not panic
    let mid: &str = segments.last().unwrap();
    let len: usize = BaseEncoding::decode_base58(mid)
      .map_err(|_| Error::InvalidDID(DIDError::InvalidMethodId))?
      .len();

    if len == BLAKE2B_256_LEN {
      Ok(())
    } else {
      Err(Error::InvalidDID(DIDError::InvalidMethodId))
    }
  }

  /// Checks if the given `DID` has a valid [`IotaDID`] network name, e.g. "main", "dev".
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  /// See [`NetworkName`] for validation requirements.
  pub fn check_network<D: DID>(did: &D) -> Result<()> {
    let network_name = Segments(did.method_id()).network();
    NetworkName::validate_network_name(network_name)
  }

  /// Checks if the given `DID` is valid according to the [`IotaDID`] method specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid [`IotaDID`].
  pub fn check_validity<D: DID>(did: &D) -> Result<()> {
    Self::check_method(did)?;
    Self::check_method_id(did)?;
    Self::check_network(did)?;

    Ok(())
  }

  /// Returns a `bool` indicating if the given `DID` is valid according to the
  /// [`IotaDID`] method specification.
  pub fn is_valid(did: &CoreDID) -> bool {
    Self::check_validity(did).is_ok()
  }

  /// Returns the Tangle `network` of the `DID`, if it is valid.
  pub fn network(&self) -> Result<Network> {
    Network::try_from_name(self.network_str().to_owned())
  }

  /// Returns the Tangle `network` name of the `DID`.
  pub fn network_str(&self) -> &str {
    self.segments().network()
  }

  /// Returns the unique Tangle tag of the `DID`.
  pub fn tag(&self) -> &str {
    self.segments().tag()
  }

  #[doc(hidden)]
  pub fn segments(&self) -> Segments<'_> {
    Segments(self.method_id())
  }

  /// Normalizes the DID `method_id` by removing the default network segment if present.
  ///
  /// E.g.
  /// - `"did:iota:main:123" -> "did:iota:123"` is normalized
  /// - `"did:iota:dev:123" -> "did:iota:dev:123"` is unchanged
  fn normalize(mut did: CoreDID) -> CoreDID {
    let segments: Segments<'_> = Segments(did.method_id());

    if segments.count() == 2 && segments.network() == Self::DEFAULT_NETWORK {
      let method_id: String = segments.tag().to_string();
      did
        .set_method_id(method_id)
        .expect("this method_id is from a valid did");
    }

    did
  }

  // Note: Must be `pub` for the `did` macro.
  #[doc(hidden)]
  pub fn encode_key(key: &[u8]) -> String {
    BaseEncoding::encode_base58(&Blake2b256::digest(key))
  }
}

impl DID for IotaDID {
  /// Returns the [`IotaDID`] scheme. See [`DID::SCHEME`].
  fn scheme(&self) -> &'static str {
    self.0.scheme()
  }

  /// Returns the [`IotaDID`] authority.
  fn authority(&self) -> &str {
    self.0.authority()
  }

  /// Returns the [`IotaDID`] method name.
  fn method(&self) -> &str {
    self.0.method()
  }

  /// Returns the [`IotaDID`] method-specific ID.
  fn method_id(&self) -> &str {
    self.0.method_id()
  }

  /// Returns the serialized [`IotaDID`].
  ///
  /// This is fast since the serialized value is stored in the [`DID`].
  fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// Consumes the [`IotaDID`] and returns the serialization.
  fn into_string(self) -> String {
    self.0.into_string()
  }

  /// Creates a new [`IotaDIDUrl`] by joining with a relative DID Url string.
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

impl Display for IotaDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Debug for IotaDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl AsRef<CoreDID> for IotaDID {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}

impl From<IotaDID> for CoreDID {
  fn from(other: IotaDID) -> Self {
    other.0
  }
}

impl TryFrom<BaseDIDUrl> for IotaDID {
  type Error = Error;

  fn try_from(other: BaseDIDUrl) -> Result<Self, Self::Error> {
    let core_did: CoreDID = CoreDID::try_from(other)?;
    Self::try_from(core_did)
  }
}

impl TryFrom<CoreDID> for IotaDID {
  type Error = Error;

  fn try_from(other: CoreDID) -> Result<Self, Self::Error> {
    Self::try_from_core(other)
  }
}

impl FromStr for IotaDID {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::parse(string)
  }
}

impl TryFrom<&str> for IotaDID {
  type Error = Error;

  fn try_from(other: &str) -> Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl TryFrom<String> for IotaDID {
  type Error = Error;

  fn try_from(other: String) -> Result<Self, Self::Error> {
    Self::parse(other)
  }
}

impl From<IotaDID> for String {
  fn from(did: IotaDID) -> Self {
    did.into_string()
  }
}

impl KeyComparable for IotaDID {
  type Key = CoreDID;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;

  use crate::did::IotaDID;
  use crate::did::IotaDIDUrl;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";

  #[test]
  fn test_parse_did_valid() {
    assert!(IotaDID::parse(format!("did:iota:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:main:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:dev:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:custom:{}", TAG)).is_ok());
  }

  #[test]
  fn test_parse_did_url_valid() {
    assert!(IotaDIDUrl::parse(format!("did:iota:{}", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:{}#fragment", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDIDUrl::parse(format!("did:iota:main:{}", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:main:{}#fragment", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:main:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:main:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDIDUrl::parse(format!("did:iota:dev:{}", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:dev:{}#fragment", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:dev:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:dev:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDIDUrl::parse(format!("did:iota:custom:{}", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:custom:{}#fragment", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:custom:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDIDUrl::parse(format!("did:iota:custom:{}?somequery=somevalue#fragment", TAG)).is_ok());
  }

  #[test]
  fn test_parse_did_invalid() {
    // A non-"iota" DID method is invalid.
    assert!(IotaDID::parse("did:foo::").is_err());
    // An empty DID method is invalid.
    assert!(IotaDID::parse("did:::").is_err());
    assert!(IotaDID::parse(format!("did::main:{}", TAG)).is_err());
    // A non-"iota" DID method is invalid.
    assert!(IotaDID::parse("did:iota---::").is_err());
    // An empty `iota-specific-idstring` is invalid.
    assert!(IotaDID::parse("did:iota:").is_err());
    // Too many components is invalid.
    assert!(IotaDID::parse(format!("did:iota:custom:shard-1:random:{}", TAG)).is_err());
    assert!(IotaDID::parse(format!("did:iota:custom:random:{}", TAG)).is_err());
    // Explicit empty network name is invalid (omitting it is still fine)
    assert!(IotaDID::parse(format!("did:iota::{}", TAG)).is_err());
    // Invalid network name is invalid.
    assert!(IotaDID::parse(format!("did:iota:Invalid-Network:{}", TAG)).is_err());
  }

  #[test]
  fn test_from_did() {
    let key: String = IotaDID::encode_key(b"123");

    let did: CoreDID = format!("did:iota:{}", key).parse().unwrap();
    let iota_did = IotaDID::try_from_core(did).unwrap();
    assert_eq!(iota_did.network_str(), "main");
    assert_eq!(iota_did.tag(), key);

    let did: CoreDID = "did:iota:123".parse().unwrap();
    assert!(IotaDID::try_from_core(did).is_err());

    let did: CoreDID = format!("did:web:{}", key).parse().unwrap();
    assert!(IotaDID::try_from_core(did).is_err());
  }

  #[test]
  fn test_network() {
    let key: String = IotaDID::encode_key(b"123");

    let did: IotaDID = format!("did:iota:{}", key).parse().unwrap();
    assert_eq!(did.network_str(), "main");

    let did: IotaDID = format!("did:iota:dev:{}", key).parse().unwrap();
    assert_eq!(did.network_str(), "dev");

    let did: IotaDID = format!("did:iota:test:{}", key).parse().unwrap();
    assert_eq!(did.network_str(), "test");

    let did: IotaDID = format!("did:iota:custom:{}", key).parse().unwrap();
    assert_eq!(did.network_str(), "custom");
  }

  #[test]
  fn test_tag() {
    let did: IotaDID = format!("did:iota:{}", TAG).parse().unwrap();
    assert_eq!(did.tag(), TAG);

    let did: IotaDID = format!("did:iota:main:{}", TAG).parse().unwrap();
    assert_eq!(did.tag(), TAG);
  }

  #[test]
  fn test_new() {
    let key: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    let did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();
    assert_eq!(did.tag(), tag);
    assert_eq!(did.network_str(), IotaDID::DEFAULT_NETWORK);
  }

  #[test]
  fn test_new_with_network() {
    let key: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let did: IotaDID = IotaDID::new_with_network(key.public().as_ref(), "foo").unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    assert_eq!(did.tag(), tag);
    assert_eq!(did.network_str(), "foo");
  }

  #[test]
  fn test_normalize() {
    let key: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    // An IotaDID with "main" as the network can be normalized ("main" removed)
    let did1: IotaDID = format!("did:iota:{}", tag).parse().unwrap();
    let did2: IotaDID = format!("did:iota:main:{}", tag).parse().unwrap();
    assert_eq!(did1, did2);
  }

  #[test]
  fn test_setter() {
    let key: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();
    let mut did_url: IotaDIDUrl = did.into_url();

    did_url.set_path(Some("/foo")).unwrap();
    did_url.set_query(Some("diff=true")).unwrap();
    did_url.set_fragment(Some("foo")).unwrap();

    assert_eq!(did_url.path(), Some("/foo"));
    assert_eq!(did_url.query(), Some("diff=true"));
    assert_eq!(did_url.fragment(), Some("foo"));
  }
}
