use identity_core::{
    common::OneOrMany,
    did::{DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use multihash::Blake2b256;

use crate::error::Result;

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: String) -> Result<DIDDocument> {
    //create comnet id
    let did: DID = create_method_id(&auth_key, Some("com"), None)?;
    let key: DID = format!("{}#key-1", did).parse()?;

    let public_key: PublicKey = PublicKeyBuilder::default()
        .id(key.clone())
        .controller(did.clone())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(auth_key))
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

pub fn create_method_id(pub_key: &str, network: Option<&str>, network_shard: Option<String>) -> Result<DID> {
    let hash = Blake2b256::digest(pub_key.as_bytes());
    let bs58key = bs58::encode(&hash.digest()).into_string();
    let network_string = match network {
        Some(network_str) => match network_str {
            "com" => "com:".to_string(),
            "dev" => "dev:".to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(), // default: "main" also can be written as ""
    };
    let shard_string = match &network_shard {
        Some(shard) => format!("{}:", shard),
        _ => String::new(),
    };
    let id_string = format!("did:iota:{}{}{}", network_string, shard_string, bs58key);
    Ok(DID::parse(id_string)?)
}
