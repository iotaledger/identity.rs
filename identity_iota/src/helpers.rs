use crate::types::DIDDiff;
use anyhow::Result;
use bs58::{decode, encode};
use identity_core::{
    common::{one_or_many::OneOrMany, Context},
    did::{Authentication, DIDDocument, KeyData, PublicKey, DID},
};
use identity_crypto::{Ed25519, Sign, Verify};
use multihash::Blake2b256;
use std::collections::HashMap;

pub fn verify_signature(message: &str, signature: &str, pub_key: &str) -> Result<bool> {
    let pub_key = bs58::decode(pub_key).into_vec()?;
    Ok(Verify::verify(
        &Ed25519,
        &message.as_bytes(),
        &decode(signature).into_vec()?,
        &identity_crypto::PublicKey::from(pub_key),
    )?)
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
        let doc_without_metadata = doc.clone().supply_metadata(HashMap::new())?;
        if let Some(auth_key) = get_auth_key(&doc) {
            // Check did auth key correlation
            let did = doc.derive_did();
            let key = match &doc.auth[0] {
                Authentication::Key(key) => key.key_data.clone(),
                _ => return Ok(false),
            };
            let created_did = create_method_id(key, Some(&did.id_segments[0]), None)?;
            if did != &created_did {
                println!("DID doesn't match auth key");
                Ok(false)
            } else {
                Ok(verify_signature(&doc_without_metadata.to_string(), sig, auth_key)?)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn sign(key: &identity_crypto::KeyPair, message: &str) -> Result<String> {
    // "proof": {
    //     "type": "RsaSignature2018",
    //     "created": "2017-10-24T05:33:31Z",
    //     "creator": "https://example.com/jdoe/keys/1",
    //     "domain": "example.com",
    //     "signatureValue": "eyiOiJJ0eXAK...EjXkgFWFO"
    //   }
    let sig = Sign::sign(&Ed25519, message.as_bytes(), key.secret())?;
    let sig_bs58 = encode(sig).into_string();
    Ok(sig_bs58)
}

// /// Signs a DID document or diff with a Ed25519 Keypair
pub fn sign_document(key: &identity_crypto::KeyPair, document: DIDDocument) -> Result<DIDDocument> {
    let mut metadata = HashMap::new();
    let signature = sign(&key, &serde_json::to_string(&document)?)?;
    metadata.insert("proof".into(), signature);
    let signed_doc = document.supply_metadata(metadata)?;
    Ok(signed_doc)
}

pub fn sign_diff(key: &identity_crypto::KeyPair, mut diddiff: DIDDiff) -> Result<DIDDiff> {
    diddiff.signature = sign(&key, &diddiff.diff)?;
    Ok(diddiff)
}

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: String) -> Result<DIDDocument> {
    let mut did_doc = DIDDocument {
        context: OneOrMany::One(Context::from("https://w3id.org/did/v1")),
        ..Default::default()
    }
    .init();
    let key_data = KeyData::Base58(auth_key);
    //create comnet id
    // let did = create_method_id(key_data.clone())?;
    let did = create_method_id(key_data.clone(), Some("com"), None)?;

    let auth_key = PublicKey {
        key_type: "RsaVerificationKey2018".into(),
        key_data,
        id: did.clone(),
        controller: did.clone(),
        ..Default::default()
    }
    .init();
    let auth = Authentication::Key(auth_key);

    did_doc.update_auth(auth);
    did_doc.id = did;

    Ok(did_doc)
}

/// Get authentication key from a DIDDocument
pub fn get_auth_key(document: &DIDDocument) -> Option<&str> {
    if let Authentication::Key(pub_key) = &document.auth[0] {
        Some(pub_key.key_data.as_str())
    } else {
        None
    }
}

pub fn create_method_id(key_data: KeyData, network: Option<&str>, network_shard: Option<String>) -> Result<DID> {
    let pub_key = key_data.as_str();
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
    Ok(DID::parse_from_str(id_string).unwrap())
}
