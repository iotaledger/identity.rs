use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    str::FromStr,
};
use identity_core::{did::DID, utils::encode_b58};
use iota::transaction::bundled::Address;
use multihash::Blake2b256;

use crate::{
    error::{Error, Result},
    utils::{create_address_from_trits, utf8_to_trytes},
};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IotaDID(DID);

impl IotaDID {
    pub const METHOD: &'static str = "iota";

    pub fn try_from_did(did: DID) -> Result<Self> {
        if did.method_name != Self::METHOD {
            return Err(Error::InvalidMethod);
        }

        if did.id_segments.is_empty() || did.id_segments.len() > 3 {
            return Err(Error::InvalidMethodId);
        }

        Ok(Self(did))
    }

    pub fn parse(string: impl AsRef<str>) -> Result<Self> {
        DID::from_str(string.as_ref())
            .map_err(Into::into)
            .and_then(Self::try_from_did)
    }

    pub fn new(public: &[u8]) -> Result<Self> {
        let key: String = Self::encode_key(public);
        let did: String = format!("did:{}:{}", Self::METHOD, key);
        let did: DID = DID::parse_from_str(did)?;

        Ok(Self(did))
    }

    pub fn with_network(public: &[u8], network: &str) -> Result<Self> {
        let key: String = Self::encode_key(public);
        let did: String = format!("did:{}:{}:{}", Self::METHOD, network, key);
        let did: DID = DID::parse_from_str(did)?;

        Ok(Self(did))
    }

    pub fn with_shard(public: &[u8], shard: &str) -> Result<Self> {
        let key: String = Self::encode_key(public);
        let did: String = format!("did:{}:{}:{}", Self::METHOD, shard, key);
        let did: DID = DID::parse_from_str(did)?;

        Ok(Self(did))
    }

    pub fn with_network_and_shard(public: &[u8], network: &str, shard: &str) -> Result<Self> {
        let key: String = Self::encode_key(public);
        let did: String = format!("did:{}:{}:{}:{}", Self::METHOD, network, shard, key);
        let did: DID = DID::parse_from_str(did)?;

        Ok(Self(did))
    }

    pub fn network(&self) -> &str {
        match &*self.id_segments {
            [_] => "main",
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
            [_] => self.id_segments.insert(0, "main".into()),
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

    fn encode_key(key: &[u8]) -> String {
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
        &self.0
    }
}

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
    use bs58::encode;
    use identity_core::did::DID;
    use iota::transaction::bundled::Address;
    use multihash::Blake2b256;

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
        let digest: &[u8] = &Blake2b256::digest(method_id(did)?.as_bytes());
        let encoded: String = encode(digest).into_string();
        let mut trytes: String = utf8_to_trytes(&encoded);

        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

        create_address_from_trits(trytes)
    }
}
