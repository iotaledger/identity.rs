use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};
use identity_core::{
    crypto::KeyPair,
    did_url::{Error as DIDError, DID},
    proof::JcsEd25519Signature2020,
    utils::{decode_b58, encode_b58},
};
use multihash::Blake2b256;

use crate::{
    did::Segments,
    error::{Error, Result},
    utils::utf8_to_trytes,
};

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "DID", try_from = "DID")]
pub struct IotaDID(DID);

impl IotaDID {
    /// The DID method name.
    pub const METHOD: &'static str = "iota";

    /// The default Tangle network.
    pub const DEFAULT_NETWORK: &'static str = "main";

    /// Generates an `IotaDID` and `KeyPair` suitable for `ed25519` signatures.
    pub fn generate_ed25519<'b, 'c, T, U>(network: T, shard: U) -> Result<(Self, KeyPair)>
    where
        T: Into<Option<&'b str>>,
        U: Into<Option<&'c str>>,
    {
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();
        let public: &[u8] = keypair.public().as_ref();

        let did: Self = Self::with_network_and_shard(public, network, shard)?;

        Ok((did, keypair))
    }

    /// Converts a borrowed `DID` to an `IotaDID.`
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn try_from_borrowed(did: &DID) -> Result<&Self> {
        Self::check_validity(did)?;

        // SAFETY: we performed the necessary validation in `check_validity`.
        Ok(unsafe { Self::new_unchecked_ref(did) })
    }

    /// Converts an owned `DID` to an `IotaDID.`
    ///
    /// # Errors
    ///
    /// Returns `Err` if the input is not a valid `IotaDID`.
    pub fn try_from_owned(did: DID) -> Result<Self> {
        Self::check_validity(&did)?;

        Ok(Self(Self::normalize(did)))
    }

    /// Converts a `DID` reference to an `IotaDID` reference without performing
    /// validation checks.
    ///
    /// # Safety
    ///
    /// This must be guaranteed safe by the caller.
    pub unsafe fn new_unchecked_ref(did: &DID) -> &Self {
        // SAFETY: This is guaranteed safe by the caller.
        &*(did as *const DID as *const IotaDID)
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
            did.push_str(network);
            did.push(':');
        }

        if let Some(shard) = shard.into() {
            did.push_str(shard);
            did.push(':');
        }

        did.push_str(&Self::encode_key(public));

        Self::parse(did)
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
    pub fn check_method_id(did: &DID) -> Result<()> {
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

    pub fn segments(&self) -> Segments<'_> {
        Segments(self.method_id())
    }

    /// Returns the Tangle address of the DID auth chain.
    pub fn address(&self) -> String {
        let mut trytes: String = utf8_to_trytes(self.tag());
        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);
        trytes
    }

    pub(crate) fn normalize(mut did: DID) -> DID {
        let segments: Segments = Segments(did.method_id());

        if segments.count() == 2 && segments.network() == Self::DEFAULT_NETWORK {
            let method_id: String = segments.tag().to_string();
            did.set_method_id(method_id);
        }

        did
    }

    pub(crate) fn encode_key(key: &[u8]) -> String {
        encode_b58(Blake2b256::digest(key).digest())
    }
}

impl Display for IotaDID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Debug for IotaDID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Deref for IotaDID {
    type Target = DID;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<DID> for IotaDID {
    fn as_ref(&self) -> &DID {
        &self.0
    }
}

impl AsRef<str> for IotaDID {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<DID> for IotaDID {
    fn eq(&self, other: &DID) -> bool {
        self.0.eq(other)
    }
}

impl From<IotaDID> for DID {
    fn from(other: IotaDID) -> Self {
        other.0
    }
}

impl TryFrom<DID> for IotaDID {
    type Error = Error;

    fn try_from(other: DID) -> Result<Self, Self::Error> {
        Self::try_from_owned(other)
    }
}

impl<'a> TryFrom<&'a DID> for &'a IotaDID {
    type Error = Error;

    fn try_from(other: &'a DID) -> Result<Self, Self::Error> {
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
        assert_eq!(did.address(), ADDR_TRYTES);
    }

    #[test]
    fn test_new() {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let did: IotaDID = IotaDID::new(key.public().as_ref()).unwrap();
        let tag: String = IotaDID::encode_key(key.public().as_ref());

        assert_eq!(did.tag(), tag);
        assert_eq!(did.network(), IotaDID::DEFAULT_NETWORK);
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

    #[test]
    fn test_normalize() {
        let key: KeyPair = JcsEd25519Signature2020::new_keypair();
        let tag: String = IotaDID::encode_key(key.public().as_ref());

        // A DID with "main" as the network can be normalized ("main" removed)
        let did1: IotaDID = format!("did:iota:{}", tag).parse().unwrap();
        let did2: IotaDID = format!("did:iota:main:{}", tag).parse().unwrap();
        assert_eq!(did1, did2);

        // A DID with a shard cannot be normalized
        let did_str: String = format!("did:iota:main:shard:{}", tag);
        let did: IotaDID = did_str.parse().unwrap();

        assert_eq!(did.as_str(), did_str);
    }
}
