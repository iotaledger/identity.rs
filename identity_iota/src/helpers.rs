use bs58::{decode, encode};
use identity_core::{
    common::{AsJson as _, OneOrMany},
    did::{Authentication, DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use identity_crypto::{Ed25519, KeyPair, Sign, Verify};
use multihash::Blake2b256;

use crate::{
    error::{Error, Result},
    types::DIDDiff,
};

pub fn verify_signature(message: &str, signature: &str, pub_key: &str) -> Result<bool> {
    let pub_key = decode(pub_key).into_vec().map_err(|_| Error::InvalidSignature)?;
    let signature = decode(signature).into_vec().map_err(|_| Error::InvalidSignature)?;

    Verify::verify(&Ed25519, message.as_bytes(), &signature, &pub_key.into()).map_err(Into::into)
}

/// Verifies Ed25519 signatures for diffs
pub fn diff_has_valid_signature(diff: DIDDiff, auth_key: &str) -> Result<bool> {
    // todo verify that did matches auth key (only before auth change, later verify signatures with previous auth key)
    Ok(verify_signature(&diff.diff, &diff.signature, auth_key)?)
}

/// Verifies Ed25519 signatures for DID documents
pub fn doc_has_valid_signature(doc: &DIDDocument) -> Result<bool> {
    // todo verify that did matches auth key (only before auth change, later verify signatures with previous auth key)
    if let Some(sig) = doc.metadata.get("proof") {
        let mut doc_without_metadata = doc.clone();
        doc_without_metadata.clear_metadata();

        if let Some(auth_key) = get_auth_key(&doc) {
            // Check did auth key correlation
            let did = doc.did();
            let key = match &doc.auth[0] {
                Authentication::Key(key) => key.key_data(),
                _ => return Ok(false),
            };
            let key = match key {
                KeyData::EthereumAddress(k) => k,
                KeyData::PublicKeyHex(k) => k,
                KeyData::PublicKeyBase58(k) => k,
                KeyData::PublicKeyPem(k) => k,
                _ => return Ok(false),
            };
            let created_did = create_method_id(key, Some(&did.id_segments[0]), None)?;
            if did != &created_did {
                println!("DID doesn't match auth key");
                Ok(false)
            } else {
                Ok(verify_signature(&doc_without_metadata.to_string(), sig, &auth_key)?)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn sign(key: &KeyPair, message: impl AsRef<[u8]>) -> Result<String> {
    let sig = Sign::sign(&Ed25519, message.as_ref(), key.secret())?;

    Ok(bs58::encode(sig).into_string())
}

/// Signs a DID document or diff with a Ed25519 Keypair
pub fn sign_document(document: &mut DIDDocument, key: &KeyPair) -> Result<()> {
    document.remove_metadata("proof");
    document.set_metadata("proof", sign(key, &document.to_json_vec()?)?);

    Ok(())
}

pub fn sign_diff(diff: &mut DIDDiff, key: &KeyPair) -> Result<()> {
    diff.signature = sign(key, diff.diff.as_bytes())?;

    Ok(())
}

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: String) -> Result<DIDDocument> {
    //create comnet id
    let did: DID = create_method_id(&auth_key, Some("com"), None)?;

    let key: PublicKey = PublicKeyBuilder::default()
        .id(did.clone())
        .controller(did.clone())
        .key_type(KeyType::RsaVerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(auth_key))
        .build()
        .expect("FIXME");

    let mut doc: DIDDocument = DIDDocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did)
        .auth(vec![key.into()])
        .build()
        .expect("FIXME");
    doc.init_timestamps();

    Ok(doc)
}

/// Get authentication key from a DIDDocument
pub fn get_auth_key(document: &DIDDocument) -> Option<String> {
    if let Authentication::Key(pub_key) = &document.auth[0] {
        match pub_key.key_data() {
            KeyData::EthereumAddress(k) => Some(k.to_string()),
            KeyData::PublicKeyHex(k) => Some(k.to_string()),
            KeyData::PublicKeyBase58(k) => Some(k.to_string()),
            KeyData::PublicKeyPem(k) => Some(k.to_string()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn create_method_id(pub_key: &str, network: Option<&str>, network_shard: Option<String>) -> Result<DID> {
    let hash = Blake2b256::digest(pub_key.as_bytes());
    let bs58key = encode(&hash.digest()).into_string();
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
