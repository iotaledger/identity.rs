use identity_core::{
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};

use crate::{did::IotaDID, error::Result};

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
