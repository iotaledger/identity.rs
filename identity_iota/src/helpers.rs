use identity_core::{
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};
use iota::transaction::bundled::Address;
use multihash::Blake2b256;

use crate::{
    did::IotaDID,
    error::Result,
    utils::{create_address_from_trits, utf8_to_trytes},
};

pub fn create_ed25519_key(did: &IotaDID, public_key: &[u8]) -> Result<PublicKey> {
    let public_key: PublicKey = PublicKeyBuilder::default()
        .id(format!("{}#key-1", did).parse()?)
        .controller(did.clone().into())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(encode_b58(public_key)))
        .build()
        .unwrap();

    Ok(public_key)
}

/// Creates an 81 Trytes IOTA address from public key bytes for a diff
pub fn create_diff_address_hash(public_key: &[u8]) -> Result<String> {
    let hash = &Blake2b256::digest(public_key);
    let second_hash = &Blake2b256::digest(hash.digest());
    let encoded: String = bs58::encode(second_hash.digest()).into_string();
    let mut trytes: String = utf8_to_trytes(&encoded);

    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

    Ok(trytes)
}

pub fn create_diff_address(public_key: &[u8]) -> Result<Address> {
    create_diff_address_hash(public_key).and_then(create_address_from_trits)
}
