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
pub fn create_address_hash(did: &DID) -> Result<String> {
    let hash = &Blake2b256::digest(method_id(did)?.as_bytes());
    let encoded: String = encode(hash.digest()).into_string();
    let mut trytes: String = utf8_to_trytes(&encoded);

    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

    Ok(trytes)
}

/// Creates an 81 Trytes IOTA address from public key bytes for a diff
pub fn create_diff_address_hash(public_key: &[u8]) -> Result<String> {
    let hash = &Blake2b256::digest(public_key);
    let encoded: String = encode(hash.digest()).into_string();
    let mut trytes: String = utf8_to_trytes(&encoded);

    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

    Ok(trytes)
}

pub fn create_address(did: &DID) -> Result<Address> {
    create_address_hash(did).and_then(create_address_from_trits)
}

pub fn create_diff_address(public_key: &[u8]) -> Result<Address> {
    create_diff_address_hash(public_key).and_then(create_address_from_trits)
}
