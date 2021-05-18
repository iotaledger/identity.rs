// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str::FromStr;
use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use identity_core::utils::decode_b58;
use identity_core::utils::encode_b58;
use identity_did::did::Error as DIDError;
use identity_did::did::DID as CoreDID;

use crate::did::Segments;
use crate::error::Error;
use crate::error::Result;

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

/// A DID URL adhering to the IOTA DID method specification.
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

  /// Converts a borrowed `DID` to an `IotaDID`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn try_from_borrowed(did: &CoreDID) -> Result<&Self> {
    Self::check_validity(did)?;

    // SAFETY: we performed the necessary validation in `check_validity`.
    Ok(unsafe { Self::new_unchecked_ref(did) })
  }

  /// Converts an owned `DID` to an `IotaDID`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn try_from_owned(did: CoreDID) -> Result<Self> {
    Self::check_validity(&did)?;

    Ok(Self(Self::normalize(did)))
  }

  /// Converts a `DID` reference to an `IotaDID` reference without performing
  /// validation checks.
  ///
  /// # Safety
  ///
  /// This must be guaranteed safe by the caller.
  pub unsafe fn new_unchecked_ref(did: &CoreDID) -> &Self {
    // SAFETY: This is guaranteed safe by the caller.
    &*(did as *const CoreDID as *const IotaDID)
  }

  /// Parses an `IotaDID` from the given `input`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn parse(input: impl AsRef<str>) -> Result<Self> {
    CoreDID::parse(input).map_err(Into::into).and_then(Self::try_from_owned)
  }

  /// Creates a new `IotaDID` with a tag derived from the given `public` key.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not form a valid `IotaDID`.
  pub fn new(public: &[u8]) -> Result<Self> {
    try_did!(public)
  }

  /// Creates a new `IotaDID` from the given `public` key and `network`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not form a valid `IotaDID`.
  pub fn with_network(public: &[u8], network: &str) -> Result<Self> {
    try_did!(public, network)
  }

  /// Creates a new `IotaDID` from the given `public` key, `network`, and `shard`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input does not form a valid `IotaDID`.
  pub fn with_network_and_shard(public: &[u8], network: &str, shard: &str) -> Result<Self> {
    try_did!(public, network, shard)
  }

  #[doc(hidden)]
  pub fn from_components(public: &[u8], network: Option<&str>, shard: Option<&str>) -> Result<Self> {
    match (network, shard) {
      (Some(network), Some(shard)) => try_did!(public, network, shard),
      (Some(network), None) => try_did!(public, network),
      (None, Some(shard)) => try_did!(public, Self::DEFAULT_NETWORK, shard),
      (None, None) => try_did!(public),
    }
  }

  /// Creates a new `DID` by joining `self` with the relative DID `other`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if any base or relative DID segments are invalid.
  pub fn join(&self, other: impl AsRef<str>) -> Result<Self> {
    self.0.join(other).map_err(Into::into).and_then(Self::try_from_owned)
  }

  /// Change the method-specific-id of the [`IotaDID`].
  pub fn set_method_id(&mut self, value: impl AsRef<str>) {
    self.0.set_method_id(value);
  }

  /// Sets the `path` component of the DID Url.
  pub fn set_path(&mut self, value: impl AsRef<str>) {
    self.0.set_path(value);
  }

  /// Sets the `query` component of the DID Url.
  pub fn set_query(&mut self, value: Option<&str>) {
    self.0.set_query(value);
  }

  /// Sets the `fragment` component of the DID Url.
  pub fn set_fragment(&mut self, value: Option<&str>) {
    self.0.set_fragment(value);
  }

  /// Checks if the given `DID` has a valid IOTA DID `method` (i.e. `"iota"`).
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn check_method(did: &CoreDID) -> Result<()> {
    if did.method() != Self::METHOD {
      Err(Error::InvalidDID(DIDError::InvalidMethodName))
    } else {
      Ok(())
    }
  }

  /// Checks if the given `DID` has a valid `IotaDID` `method_id`.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn check_method_id(did: &CoreDID) -> Result<()> {
    let segments: Vec<&str> = did.method_id().split(':').collect();

    if segments.is_empty() || segments.len() > 3 {
      return Err(Error::InvalidDID(DIDError::InvalidMethodId));
    }

    // We checked if `id_segments` was empty so this should not panic
    let mid: &str = segments.last().unwrap();
    let len: usize = decode_b58(mid)?.len();

    if len == BLAKE2B_256_LEN {
      Ok(())
    } else {
      Err(Error::InvalidDID(DIDError::InvalidMethodId))
    }
  }

  /// Checks if the given `DID` is valid according to the `IotaDID` method
  /// specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid `IotaDID`.
  pub fn check_validity(did: &CoreDID) -> Result<()> {
    Self::check_method(did)?;
    Self::check_method_id(did)?;

    Ok(())
  }

  /// Returns a `bool` indicating if the given `DID` is valid according to the
  /// `IotaDID` method specification.
  pub fn is_valid(did: &CoreDID) -> bool {
    Self::check_validity(did).is_ok()
  }

  /// Returns the serialized [`IotaDID`].
  ///
  /// This is fast since the serialized value is stored in the [`DID`].
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  /// Consumes the [`IotaDID`] and returns the serialization.
  pub fn into_string(self) -> String {
    self.0.into_string()
  }

  /// Returns the [`IotaDID`] scheme. See [`DID::SCHEME`].
  pub fn scheme(&self) -> &'static str {
    self.0.scheme()
  }

  /// Returns the [`IotaDID`] authority.
  pub fn authority(&self) -> &str {
    self.0.authority()
  }

  /// Returns the [`IotaDID`] method name.
  pub fn method(&self) -> &str {
    self.0.method()
  }

  /// Returns the [`IotaDID`] method-specific ID.
  pub fn method_id(&self) -> &str {
    self.0.method_id()
  }

  /// Returns the [`IotaDID`] path.
  pub fn path(&self) -> &str {
    self.0.path()
  }

  /// Returns the [`IotaDID`] method query, if any.
  pub fn query(&self) -> Option<&str> {
    self.0.query()
  }

  /// Returns the [`IotaDID`] method fragment, if any.
  pub fn fragment(&self) -> Option<&str> {
    self.0.fragment()
  }

  /// Parses the [`IotaDID`] query and returns an iterator of (key, value) pairs.
  pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
    self.0.query_pairs()
  }

  /// Returns the Tangle `network` of the `DID`.
  pub fn network(&self) -> &str {
    self.segments().network()
  }

  /// Returns the Tangle network `shard` of the `DID`.
  pub fn shard(&self) -> Option<&str> {
    self.segments().shard()
  }

  /// Returns the unique Tangle tag of the `DID`.
  pub fn tag(&self) -> &str {
    self.segments().tag()
  }

  #[doc(hidden)]
  pub fn segments(&self) -> Segments<'_> {
    Segments(self.method_id())
  }

  pub(crate) fn normalize(mut did: CoreDID) -> CoreDID {
    let segments: Segments<'_> = Segments(did.method_id());

    if segments.count() == 2 && segments.network() == Self::DEFAULT_NETWORK {
      let method_id: String = segments.tag().to_string();
      did.set_method_id(method_id);
    }

    did
  }

  // Note: Must be `pub` for the `did` macro.
  #[doc(hidden)]
  pub fn encode_key(key: &[u8]) -> String {
    encode_b58(&Blake2b256::digest(key))
  }
}

impl Display for IotaDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.0)
  }
}

impl Debug for IotaDID {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

impl TryFrom<CoreDID> for IotaDID {
  type Error = Error;

  fn try_from(other: CoreDID) -> Result<Self, Self::Error> {
    Self::try_from_owned(other)
  }
}

impl<'a> TryFrom<&'a CoreDID> for &'a IotaDID {
  type Error = Error;

  fn try_from(other: &'a CoreDID) -> Result<Self, Self::Error> {
    IotaDID::try_from_borrowed(other)
  }
}

impl FromStr for IotaDID {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::parse(string)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyPair;
  use identity_did::did::DID as CoreDID;

  use crate::did::IotaDID;

  const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";

  #[test]
  fn test_parse_valid() {
    assert!(IotaDID::parse(format!("did:iota:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:{}#fragment", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDID::parse(format!("did:iota:main:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:main:{}#fragment", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:main:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:main:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDID::parse(format!("did:iota:test:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:test:{}#fragment", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:test:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:test:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDID::parse(format!("did:iota:rainbow:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:{}#fragment", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:{}?somequery=somevalue#fragment", TAG)).is_ok());

    assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:{}", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:{}#fragment", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:{}?somequery=somevalue", TAG)).is_ok());
    assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:{}?somequery=somevalue#fragment", TAG)).is_ok());
  }

  #[test]
  fn test_parse_invalid() {
    // A non-"iota" DID method is invalid.
    assert!(IotaDID::parse("did:foo::").is_err());
    // An empty DID method is invalid.
    assert!(IotaDID::parse("did:::").is_err());
    assert!(IotaDID::parse(format!("did::rainbow:shard-1:{}", TAG)).is_err());
    // A non-"iota" DID method is invalid.
    assert!(IotaDID::parse("did:iota---::").is_err());
    // An empty `iota-specific-idstring` is invalid.
    assert!(IotaDID::parse("did:iota:").is_err());
    // Too many components is invalid.
    assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:random:{}", TAG)).is_err());
  }

  #[test]
  fn test_from_did() {
    let key: String = IotaDID::encode_key(b"123");

    let did: CoreDID = format!("did:iota:{}", key).parse().unwrap();
    let iota_did = IotaDID::try_from_owned(did).unwrap();
    assert_eq!(iota_did.network(), "main");
    assert_eq!(iota_did.shard(), None);
    assert_eq!(iota_did.tag(), key);
    assert_eq!(iota_did.path(), "");
    assert_eq!(iota_did.query(), None);

    let did: CoreDID = "did:iota:123".parse().unwrap();
    assert!(IotaDID::try_from_owned(did).is_err());

    let did: CoreDID = format!("did:web:{}", key).parse().unwrap();
    assert!(IotaDID::try_from_owned(did).is_err());
  }

  #[test]
  fn test_network() {
    let key: String = IotaDID::encode_key(b"123");

    let did: IotaDID = format!("did:iota:test:{}", key).parse().unwrap();
    assert_eq!(did.network(), "test");

    let did: IotaDID = format!("did:iota:{}", key).parse().unwrap();
    assert_eq!(did.network(), "main");

    let did: IotaDID = format!("did:iota:rainbow:{}", key).parse().unwrap();
    assert_eq!(did.network(), "rainbow");
  }

  #[test]
  fn test_shard() {
    let key: String = IotaDID::encode_key(b"123");

    let did: IotaDID = format!("did:iota:{}", key).parse().unwrap();
    assert_eq!(did.shard(), None);

    let did: IotaDID = format!("did:iota:dev:{}", key).parse().unwrap();
    assert_eq!(did.shard(), None);

    let did: IotaDID = format!("did:iota:dev:shard:{}", key).parse().unwrap();
    assert_eq!(did.shard(), Some("shard"));
  }

  #[test]
  fn test_tag() {
    let did: IotaDID = format!("did:iota:{}", TAG).parse().unwrap();
    assert_eq!(did.tag(), TAG);

    let did: IotaDID = format!("did:iota:main:{}", TAG).parse().unwrap();
    assert_eq!(did.tag(), TAG);

    let did: IotaDID = format!("did:iota:main:shard:{}", TAG).parse().unwrap();
    assert_eq!(did.tag(), TAG);
  }

  #[test]
  fn test_new() {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    assert_eq!(did.tag(), tag);
    assert_eq!(did.network(), IotaDID::DEFAULT_NETWORK);
    assert_eq!(did.shard(), None);

    let did = IotaDID::from_components(key.public().as_ref(), None, None).unwrap();
    assert_eq!(did.tag(), tag);
    assert_eq!(did.network(), IotaDID::DEFAULT_NETWORK);
    assert_eq!(did.shard(), None);

    let did = IotaDID::from_components(key.public().as_ref(), Some(IotaDID::DEFAULT_NETWORK), None).unwrap();
    assert_eq!(did.network(), IotaDID::DEFAULT_NETWORK);
  }

  #[test]
  fn test_with_network() {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let did: IotaDID = IotaDID::with_network(key.public().as_ref(), "foo").unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    assert_eq!(did.tag(), tag);
    assert_eq!(did.network(), "foo");
    assert_eq!(did.shard(), None);
  }

  #[test]
  fn test_with_network_and_shard() {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let did: IotaDID = IotaDID::with_network_and_shard(key.public().as_ref(), "foo", "shard-1").unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    assert_eq!(did.tag(), tag);
    assert_eq!(did.network(), "foo");
    assert_eq!(did.shard(), Some("shard-1"));
  }

  #[test]
  fn test_normalize() {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let tag: String = IotaDID::encode_key(key.public().as_ref());

    // A IotaDID with "main" as the network can be normalized ("main" removed)
    let did1: IotaDID = format!("did:iota:{}", tag).parse().unwrap();
    let did2: IotaDID = format!("did:iota:main:{}", tag).parse().unwrap();
    assert_eq!(did1, did2);

    // A IotaDID with a shard cannot be normalized
    let did_str: String = format!("did:iota:main:shard:{}", tag);
    let did: IotaDID = did_str.parse().unwrap();

    assert_eq!(did.as_str(), did_str);
  }

  #[test]
  fn test_setter() {
    let key: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();

    did.set_path("#foo");
    did.set_query(Some("diff=true"));
    did.set_fragment(Some("foo"));

    assert_eq!(did.path(), "#foo");
    assert_eq!(did.query(), Some("diff=true"));
    assert_eq!(did.fragment(), Some("foo"));
  }
}
