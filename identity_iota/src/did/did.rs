use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    iter::once,
    ops::Deref,
    str::FromStr,
};
use identity_core::{
    crypto::KeyPair,
    did_url::{self, DID},
    proof::JcsEd25519Signature2020,
    utils::{decode_b58, encode_b58},
};
use iota::transaction::bundled::Address;
use multihash::Blake2b256;
use std::borrow::Cow;

use crate::{
    error::{Error, Result},
    utils::{create_address_from_trits, utf8_to_trytes},
};

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(into = "DID", try_from = "DID")]
pub struct IotaDID<'a>(Cow<'a, DID>);

impl<'a> IotaDID<'a> {
    /// The DID method name.
    pub const METHOD: &'static str = "iota";

    /// The default Tangle network.
    pub const NETWORK: &'static str = "main";

    /// Generates an `IotaDID` and `KeyPair` suitable for `ed25519` signatures.
    pub fn generate_ed25519<'b, 'c, T, U>(network: T, shard: U) -> Result<(Self, KeyPair)>
    where
        T: Into<Option<&'b str>>,
        U: Into<Option<&'c str>>,
    {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let did: Self = Self::with_network_and_shard(key.public().as_ref(), network, shard)?;

        Ok((did, key))
    }

    /// Converts a borrowed `DID` to an `IotaDID.`
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn try_from_borrowed(did: &'a DID) -> Result<Self> {
        Self::try_from_cow(Cow::Borrowed(did))
    }

    /// Converts an owned `DID` to an `IotaDID.`
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn try_from_owned(did: DID) -> Result<Self> {
        Self::try_from_cow(Cow::Owned(did))
    }

    /// Converts a clone-on-write `DID` to an `IotaDID`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn try_from_cow(did: Cow<'a, DID>) -> Result<Self> {
        Self::check_validity(&did)?;

        // SAFETY: we performed the necessary validation in `check_validity`.
        Ok(unsafe { Self::from_cow_unchecked(did) })
    }

    /// Converts a clone-on-write `DID` to an `IotaDID` without validation.
    ///
    /// # Safety
    ///
    /// This must be guaranteed safe by the caller.
    #[allow(unused_unsafe)]
    pub unsafe fn from_cow_unchecked(did: Cow<'a, DID>) -> Self {
        // SAFETY: This is guaranteed safe by the caller.
        unsafe { Self(did) }
    }

    /// Parses an `IotaDID` from the given `input`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn parse(input: impl AsRef<str>) -> Result<Self> {
        DID::parse(input).map_err(Into::into).and_then(Self::try_from_owned)
    }

    /// Creates a new `IotaDID` with a tag derived from the given `public` key.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input does not form a valid `IotaDID`.
    pub fn new(public: &[u8]) -> Result<Self> {
        Self::with_network_and_shard(public, None, None)
    }

    /// Creates a new `IotaDID` for the given `public` key and `network`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input does not form a valid `IotaDID`.
    pub fn with_network<'b, T>(public: &[u8], network: T) -> Result<Self>
    where
        T: Into<Option<&'b str>>,
    {
        Self::with_network_and_shard(public, network, None)
    }

    /// Creates a new `IotaDID` for the given `public` key, `network`, and `shard`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input does not form a valid `IotaDID`.
    pub fn with_network_and_shard<'b, 'c, T, U>(public: &[u8], network: T, shard: U) -> Result<Self>
    where
        T: Into<Option<&'b str>>,
        U: Into<Option<&'c str>>,
    {
        let mut did: String = format!("{}:{}:", DID::SCHEME, Self::METHOD);

        if let Some(network) = network.into() {
            did.extend(network.chars().chain(once(':')));
        }

        if let Some(shard) = shard.into() {
            did.extend(shard.chars().chain(once(':')));
        }

        did.push_str(&Self::encode_key(public));

        did.parse().map(Cow::Owned).map(Self).map_err(Into::into)
    }

    /// Creates a new [`IotaDID`] by joining `self` with the relative IotaDID `other`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if any base or relative DID segments are invalid.
    pub fn join(&self, other: impl AsRef<str>) -> Result<Self> {
        self.0.join(other).map_err(Into::into).and_then(Self::try_from_owned)
    }

    /// Checks if the given `DID` has a valid `IotaDID` `method`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn check_method(did: &DID) -> Result<()> {
        if did.method() != Self::METHOD {
            Err(did_url::Error::InvalidMethodName.into())
        } else {
            Ok(())
        }
    }

    /// Checks if the given `DID` has a valid `IotaDID` `method_id`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn check_method_id(did: &DID) -> Result<()> {
        let segments: Vec<&str> = did.method_id().split(':').collect();

        if segments.is_empty() || segments.len() > 3 {
            return Err(did_url::Error::InvalidMethodId.into());
        }

        // We checked if `id_segments` was empty so this should not panic
        let mid: &str = segments.last().unwrap();
        let len: usize = decode_b58(mid)?.len();

        // TODO: Check if bytes are valid trytes

        if len == BLAKE2B_256_LEN {
            Ok(())
        } else {
            Err(did_url::Error::InvalidMethodId.into())
        }
    }

    /// Checks if the given `DID` is a valid `IotaDID`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn check_validity(did: &DID) -> Result<()> {
        Self::check_method(did)?;
        Self::check_method_id(did)?;

        Ok(())
    }

    /// Returns a `bool` indicating if the given `DID` is a valid `IotaDID`.
    pub fn is_valid(did: &DID) -> bool {
        Self::check_validity(did).is_ok()
    }

    /// Returns the Tangle `network` of the `IotaDID`.
    pub fn network(&self) -> &str {
        self.segments().network()
    }

    /// Returns the Tangle network `shard` of the `IotaDID`.
    pub fn shard(&self) -> Option<&str> {
        self.segments().shard()
    }

    /// Returns the unique Tangle tag of the `IotaDID`.
    pub fn tag(&self) -> &str {
        self.segments().tag()
    }

    /// Returns the Tangle address of the DID as a tryte-encoded String.
    pub fn address_hash(&self) -> String {
        let mut trytes: String = utf8_to_trytes(self.tag());
        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);
        trytes
    }

    /// Returns the Tangle address of the DID.
    pub fn address(&self) -> Result<Address> {
        create_address_from_trits(self.address_hash())
    }

    pub fn segments(&self) -> Segments {
        Segments(self.method_id())
    }

    fn encode_key(key: &[u8]) -> String {
        encode_b58(Blake2b256::digest(key).digest())
    }
}

impl Display for IotaDID<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Debug for IotaDID<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Deref for IotaDID<'_> {
    type Target = DID;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<DID> for IotaDID<'_> {
    fn as_ref(&self) -> &DID {
        self.0.as_ref()
    }
}

impl AsRef<str> for IotaDID<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<DID> for IotaDID<'_> {
    fn eq(&self, other: &DID) -> bool {
        self.0.as_ref() == other
    }
}

impl From<IotaDID<'_>> for DID {
    fn from(other: IotaDID<'_>) -> Self {
        other.0.into_owned()
    }
}

impl TryFrom<DID> for IotaDID<'_> {
    type Error = Error;

    fn try_from(other: DID) -> Result<Self, Self::Error> {
        Self::try_from_owned(other)
    }
}

impl<'a> TryFrom<&'a DID> for IotaDID<'a> {
    type Error = Error;

    fn try_from(other: &'a DID) -> Result<Self, Self::Error> {
        Self::try_from_borrowed(other)
    }
}

impl FromStr for IotaDID<'_> {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segments<'a>(&'a str);

impl<'a> Segments<'a> {
    pub fn network(&self) -> &'a str {
        match self.count() {
            1 => IotaDID::NETWORK,
            2 | 3 => &self.0[..self.head()],
            _ => unreachable!("Segments::network called for invalid IOTA DID"),
        }
    }

    pub fn shard(&self) -> Option<&'a str> {
        match self.count() {
            1 | 2 => None,
            3 => Some(&self.0[&self.head() + 1..self.tail()]),
            _ => unreachable!("Segments::shard called for invalid IOTA DID"),
        }
    }

    pub fn tag(&self) -> &'a str {
        match self.count() {
            1 => self.0,
            2 | 3 => &self.0[self.tail() + 1..],
            _ => unreachable!("Segments::tag called for invalid IOTA DID"),
        }
    }

    pub fn count(&self) -> usize {
        self.0.split(':').count()
    }

    fn head(&self) -> usize {
        self.0.find(':').unwrap()
    }

    fn tail(&self) -> usize {
        self.0.rfind(':').unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_core::{crypto::KeyPair, did_url::DID, proof::JcsEd25519Signature2020};

    const TAG: &str = "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";

    const ADDR_TAG: &str = "HbuRS48djS5PbLQciy6iE9BTdaDTBM3GxcbGdyuv3TWo";
    const ADDR_TRYTES: &str = "RBQCIDACBCYABBSCYCBCZAZBQCVB9CRCXCMD9BXCOBCBLBCCSCPCNBCCLBWBXAQBLDRCQCQBSCMDIDJDX";

    #[test]
    fn test_parse_valid() {
        assert!(IotaDID::parse(format!("did:iota:{}", TAG)).is_ok());
        assert!(IotaDID::parse(format!("did:iota:main:{}", TAG)).is_ok());
        assert!(IotaDID::parse(format!("did:iota:com:{}", TAG)).is_ok());
        assert!(IotaDID::parse(format!("did:iota:dev:{}", TAG)).is_ok());
        assert!(IotaDID::parse(format!("did:iota:rainbow:{}", TAG)).is_ok());
        assert!(IotaDID::parse(format!("did:iota:rainbow:shard-1:{}", TAG)).is_ok());
    }

    #[test]
    fn test_parse_invalid() {
        assert!(IotaDID::parse("did:foo::").is_err());
        assert!(IotaDID::parse("did:::").is_err());
        assert!(IotaDID::parse("did:iota---::").is_err());
        assert!(IotaDID::parse("did:iota:").is_err());
    }

    #[test]
    fn test_from_did() {
        let key: String = IotaDID::encode_key(b"123");

        let did: DID = format!("did:iota:{}", key).parse().unwrap();
        assert!(IotaDID::try_from_owned(did).is_ok());

        let did: DID = "did:iota:123".parse().unwrap();
        assert!(IotaDID::try_from_owned(did).is_err());

        let did: DID = format!("did:web:{}", key).parse().unwrap();
        assert!(IotaDID::try_from_owned(did).is_err());
    }

    #[test]
    fn test_network() {
        let key: String = IotaDID::encode_key(b"123");

        let did: IotaDID = format!("did:iota:dev:{}", key).parse().unwrap();
        assert_eq!(did.network(), "dev");

        let did: IotaDID = format!("did:iota:{}", key).parse().unwrap();
        assert_eq!(did.network(), "main");

        let did: IotaDID = format!("did:iota:rainbow:{}", key).parse().unwrap();
        assert_eq!(did.network(), "rainbow");
    }

    #[test]
    fn test_shard() {
        let key: String = IotaDID::encode_key(b"123");

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
    fn test_address() {
        let did: IotaDID = format!("did:iota:com:{}", ADDR_TAG).parse().unwrap();
        assert_eq!(did.address_hash(), ADDR_TRYTES);
    }

    #[test]
    fn test_new() {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();
        let tag: String = IotaDID::encode_key(key.public().as_ref());

        assert_eq!(did.tag(), tag);
        assert_eq!(did.network(), IotaDID::NETWORK);
        assert_eq!(did.shard(), None);
    }

    #[test]
    fn test_with_network() {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let did: IotaDID = IotaDID::with_network(key.public().as_ref(), "foo").unwrap();
        let tag: String = IotaDID::encode_key(key.public().as_ref());

        assert_eq!(did.tag(), tag);
        assert_eq!(did.network(), "foo");
        assert_eq!(did.shard(), None);
    }

    #[test]
    fn test_with_network_and_shard() {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let did: IotaDID = IotaDID::with_network_and_shard(key.public().as_ref(), "foo", "shard-1").unwrap();
        let tag: String = IotaDID::encode_key(key.public().as_ref());

        assert_eq!(did.tag(), tag);
        assert_eq!(did.network(), "foo");
        assert_eq!(did.shard(), Some("shard-1"));
    }
}
