use identity_core::{
    common::OneOrMany,
    did::{DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};

use crate::{did::TangleDID, error::Result};

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: &[u8]) -> Result<DIDDocument> {
    //create comnet id
    let did: DID = TangleDID::with_network(auth_key, "com")?.into();
    let key: DID = format!("{}#key-1", did).parse()?;

    let public_key: PublicKey = PublicKeyBuilder::default()
        .id(key.clone())
        .controller(did.clone())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(encode_b58(auth_key)))
        .build()
        .unwrap();

    let doc: DIDDocument = DIDDocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did)
        .public_keys(vec![public_key])
        .auth(vec![key.into()])
        .build()
        .unwrap();

    Ok(doc)
}
