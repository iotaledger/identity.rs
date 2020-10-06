use bs58::{decode, encode};
use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Authentication, Context, KeyData, PublicKey},
};
use identity_crypto::{Ed25519, Sign, Verify};

use crate::tangle_writer::{iota_network, Payload};
use iota_conversion::trytes_converter;
use multihash::Blake2b256;
use std::collections::HashMap;

/// Creates an 81 Trytes IOTA address from the DID
pub fn get_iota_address(did: &DID) -> crate::Result<String> {
    let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
    let hash = Blake2b256::digest(iota_specific_idstring.as_bytes());
    let bs58key = encode(&hash.digest()).into_string();
    let trytes = match trytes_converter::to_trytes(&bs58key) {
        Ok(trytes) => trytes,
        _ => return Err(crate::Error::TryteConversionError),
    };
    Ok(trytes[0..81].into())
}

pub fn verify_signature(message: &str, signature: &str, pub_key: &str) -> crate::Result<bool> {
    let pub_key = bs58::decode(pub_key).into_vec()?;
    Ok(Verify::verify(
        &Ed25519,
        &message.as_bytes(),
        &decode(signature).into_vec()?,
        &identity_crypto::PublicKey::from(pub_key),
    )?)
}

/// Verifies Ed25519 signatures for DID documents or diffs
pub fn has_valid_signature(payload: &Payload) -> crate::Result<bool> {
    //todo verify that did matches auth key (only before auth change, later verify signatures with previous auth key)
    let is_valid = match payload {
        Payload::DIDDocument(doc) => {
            if let Some(sig) = doc.metadata.get("proof") {
                let doc_without_metadata = doc.clone().supply_metadata(HashMap::new())?;
                if let Authentication::Key(pub_key) = &doc.auth[0].0 {
                    let pub_key = match &pub_key.key_data {
                        KeyData::Unknown(key) => key,
                        KeyData::Pem(key) => key,
                        KeyData::Jwk(key) => key,
                        KeyData::Hex(key) => key,
                        KeyData::Base64(key) => key,
                        KeyData::Base58(key) => key,
                        KeyData::Multibase(key) => key,
                        KeyData::IotaAddress(key) => key,
                        KeyData::EthereumAddress(key) => key,
                    };
                    verify_signature(&doc_without_metadata.to_string(), sig, pub_key)?
                } else {
                    false
                }
            } else {
                false
            }
        }
        Payload::DIDDocumentDifferences(diff) => verify_signature(&diff.diff, &diff.signature, &diff.auth_key)?,
    };

    Ok(is_valid)
}

pub fn sign(key: &identity_crypto::KeyPair, message: &str) -> crate::Result<String> {
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

/// Signs a DID document or diff with a Ed25519 Keypair
pub fn sign_payload(key: &identity_crypto::KeyPair, payload: Payload) -> crate::Result<Payload> {
    let signed_payload = match payload {
        Payload::DIDDocument(document) => {
            let mut metadata = HashMap::new();
            let signature = sign(&key, &document.to_string())?;
            metadata.insert("proof".into(), signature);
            let signed_doc = document.supply_metadata(metadata)?;
            Payload::DIDDocument(signed_doc)
        }
        Payload::DIDDocumentDifferences(mut diff) => {
            diff.signature = sign(&key, &diff.diff)?;
            Payload::DIDDocumentDifferences(diff)
        }
    };
    Ok(signed_payload)
}

/// Creates a DID document with an auth key and a DID
pub fn create_document(auth_key: String) -> crate::Result<DIDDocument> {
    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        ..Default::default()
    }
    .init();

    let key_data = KeyData::Base58(auth_key);
    let mut auth_key = PublicKey {
        key_type: "RsaVerificationKey2018".into(),
        key_data,
        ..Default::default()
    }
    .init();
    auth_key.create_own_id(iota_network::Comnet, None)?;
    let auth = Authentication::Key(auth_key);
    did_doc.update_auth(auth);

    did_doc.create_id()?;
    Ok(did_doc)
}
