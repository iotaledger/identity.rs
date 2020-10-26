use crate::utils::{create_address_from_trits, utf8_to_trytes};
use crate::{did::IotaDID, error::Result};
use bs58::encode;
use identity_core::{
    common::OneOrMany,
    did::{DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};
use iota::transaction::bundled::Address;
use multihash::Blake2b256;

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: &[u8]) -> Result<DIDDocument> {
    //create comnet id
    let did: DID = IotaDID::with_network(auth_key, "com")?.into();
    let key: DID = format!("{}#key-1", did).parse()?;

    let public_key: PublicKey = PublicKeyBuilder::default()
        .id(key.clone())
        .controller(did.clone())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(encode_b58(auth_key)))
        .build()
        .unwrap();

    let mut doc: DIDDocument = DIDDocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did)
        .public_keys(vec![public_key])
        .auth(vec![key.into()])
        .build()
        .unwrap();

    doc.init_timestamps();

    Ok(doc)
}

/// Creates an 81 Trytes IOTA address from public key bytes for a diff
pub fn create_diff_address_hash(public_key: &[u8]) -> Result<String> {
    let hash = &Blake2b256::digest(public_key);
    let second_hash = &Blake2b256::digest(hash.digest());
    let encoded: String = encode(second_hash.digest()).into_string();
    let mut trytes: String = utf8_to_trytes(&encoded);

    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

    Ok(trytes)
}
pub fn create_diff_address(public_key: &[u8]) -> Result<Address> {
    create_diff_address_hash(public_key).and_then(create_address_from_trits)
}
