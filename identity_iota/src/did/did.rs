use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    iter::once,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use identity_core::{
    did::DID,
    utils::{decode_b58, encode_b58},
};
use identity_crypto::KeyPair;
use identity_proof::signature::jcsed25519signature2020;
use iota::transaction::bundled::Address;
use multihash::Blake2b256;

use crate::{
    error::{Error, Result},
    utils::{create_address_from_trits, utf8_to_trytes},
};

// The hash size of BLAKE2b-256 (32-bytes)
const BLAKE2B_256_LEN: usize = 32;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IotaDID(DID);

impl IotaDID {
    pub const METHOD: &'static str = "iota";
    pub const NETWORK: &'static str = "main";

    pub fn generate_ed25519<'a, T>(network: T) -> Result<(Self, KeyPair)>
    where
        T: Into<Option<&'a str>>,
    {
        let key: KeyPair = jcsed25519signature2020::new_keypair();
        let did: Self = Self::with_network(key.public().as_ref(), network)?;

        Ok((did, key))
    }

    pub fn try_from_did(did: DID) -> Result<Self> {
        Self::check_validity(&did).map(|_| Self(did))
    }

    pub fn parse(string: impl AsRef<str>) -> Result<Self> {
        Self::try_from_did(DID::from_str(string.as_ref())?)
    }

    pub fn new(public: &[u8]) -> Result<Self> {
        Self::with_network_and_shard(public, None, None)
    }

    pub fn with_network<'a, T>(public: &[u8], network: T) -> Result<Self>
    where
        T: Into<Option<&'a str>>,
    {
        Self::with_network_and_shard(public, network, None)
    }

    pub fn with_network_and_shard<'a, 'b, T, U>(public: &[u8], network: T, shard: U) -> Result<Self>
    where
        T: Into<Option<&'a str>>,
        U: Into<Option<&'b str>>,
    {
        let mut did: String = format!("did:{}:", Self::METHOD);

        if let Some(network) = network.into() {
            did.extend(network.chars().chain(once(':')));
        }

        if let Some(shard) = shard.into() {
            did.extend(shard.chars().chain(once(':')));
        }

        did.push_str(&Self::encode_key(public));

        Ok(Self(DID::parse(did)?))
    }

    pub fn network(&self) -> &str {
        match &*self.id_segments {
            [_] => Self::NETWORK,
            [network, _] => &*network,
            [network, _, _] => &*network,
            _ => unreachable!("IotaDID::network called for invalid DID"),
        }
    }

    pub fn shard(&self) -> Option<&str> {
        match &*self.id_segments {
            [_] => None,
            [_, _] => None,
            [_, shard, _] => Some(&*shard),
            _ => unreachable!("IotaDID::shard called for invalid DID"),
        }
    }

    pub fn method_id(&self) -> &str {
        match &*self.id_segments {
            [mid] => &*mid,
            [_, mid] => &*mid,
            [_, _, mid] => &*mid,
            _ => unreachable!("IotaDID::method_id called for invalid DID"),
        }
    }

    pub fn normalize(&mut self) {
        match &*self.id_segments {
            [_] => self.id_segments.insert(0, Self::NETWORK.into()),
            [_, _] | [_, _, _] => {}
            _ => unreachable!("IotaDID::normalize called for invalid DID"),
        }
    }

    pub fn normalized(&self) -> Self {
        let mut this: Self = self.clone();
        this.normalize();
        this
    }

    /// Creates an 81 Trytes IOTA address from the DID
    pub fn create_address_hash(&self) -> String {
        let mut trytes: String = utf8_to_trytes(self.method_id());
        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);
        trytes
    }

    pub fn create_address(&self) -> Result<Address> {
        create_address_from_trits(self.create_address_hash())
    }

    pub fn is_valid(did: &DID) -> bool {
        Self::check_validity(did).is_ok()
    }

    pub fn check_validity(did: &DID) -> Result<(), Error> {
        if did.method_name != Self::METHOD {
            return Err(Error::InvalidMethod);
        }

        if did.id_segments.is_empty() || did.id_segments.len() > 3 {
            return Err(Error::InvalidMethodId);
        }

        // We checked if `id_segments` was empty so this should not panic
        let mid: &str = did.id_segments.last().expect("infallible");
        let len: usize = decode_b58(mid)?.len();

        if len != BLAKE2B_256_LEN {
            return Err(Error::InvalidMethodId);
        }

        Ok(())
    }

    fn encode_key(key: &[u8]) -> String {
        encode_b58(Blake2b256::digest(key).digest())
    }
}

impl PartialEq<DID> for IotaDID {
    fn eq(&self, other: &DID) -> bool {
        self.0.eq(other)
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
        &self.0
    }
}

// TODO: Remove this
impl DerefMut for IotaDID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        Self::try_from_did(other)
    }
}

impl FromStr for IotaDID {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

pub mod deprecated {
    use identity_core::did::DID;
    use iota::transaction::bundled::Address;

    use crate::{
        error::{Error, Result},
        utils::{create_address_from_trits, utf8_to_trytes},
    };

    pub fn method_id(did: &DID) -> Result<&str> {
        did.id_segments
            .last()
            .map(|string| string.as_str())
            .ok_or_else(|| Error::InvalidMethodId)
    }

    /// Creates an 81 Trytes IOTA address from the DID
    pub fn create_address(did: &DID) -> Result<Address> {
        let mut trytes: String = utf8_to_trytes(method_id(did)?);

        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

        create_address_from_trits(trytes)
    }
}
