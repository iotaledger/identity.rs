use identity_core::{
    common::{SerdeInto as _, ToJson as _, Value},
    did::DIDDocument as Document,
    key::KeyRelation,
    utils::{decode_b58, encode_b58},
};
use identity_crypto::{
    sha2::{self, Digest as _},
    KeyPair,
};
use serde::Serialize;
use sodiumoxide::crypto::sign::ed25519;

use crate::{
    error::{Error, Result},
    signature::{LdSignature, SignatureOptions},
};

const SIGNATURE_TYPE: &str = "JcsEd25519Signature2020";

pub fn new_keypair() -> KeyPair {
    let (public, secret): (ed25519::PublicKey, ed25519::SecretKey) = ed25519::gen_keypair();

    KeyPair::new(public.as_ref().to_vec().into(), secret.as_ref().to_vec().into())
}

fn pubkey(slice: &[u8]) -> Result<ed25519::PublicKey> {
    ed25519::PublicKey::from_slice(slice).ok_or(Error::InvalidKeyFormat)
}

fn seckey(slice: &[u8]) -> Result<ed25519::SecretKey> {
    ed25519::SecretKey::from_slice(slice).ok_or(Error::InvalidKeyFormat)
}

// output = <SIGNATURE><MESSAGE>
fn ed25519_sign(message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
    seckey(secret).map(|secret| ed25519::sign(message, &secret))
}

// signature = <SIGNATURE><MESSAGE>
fn ed25519_verify(signature: &[u8], public: &[u8]) -> Result<Vec<u8>> {
    pubkey(public).and_then(|public| ed25519::verify(signature, &public).map_err(|_| Error::InvalidSignature))
}

pub fn jcs_sign<T>(document: &T, secret: &[u8]) -> Result<String>
where
    T: Serialize,
{
    serde_jcs::to_vec(document)
        .map_err(|_| Error::InvalidDocument)
        .map(|canon| sha2::Sha256::digest(&canon))
        .and_then(|digest| ed25519_sign(&digest, secret))
        .map(|signature| encode_b58(&signature))
}

pub fn jcs_verify<T>(document: &T, public: &[u8]) -> Result<()>
where
    T: Serialize,
{
    let mut json: Value = document.to_json_value()?;

    let proof: LdSignature = json
        .get("proof")
        .map(|data| data.serde_into())
        .transpose()?
        .ok_or(Error::InvalidSignature)?;

    if proof.type_ != SIGNATURE_TYPE {
        return Err(Error::InvalidSignature);
    }

    let signature: Vec<u8> = decode_b58(proof.proof())?;
    let verified: Vec<u8> = ed25519_verify(&signature, public)?;

    // Remove the signature from the JSON object
    let _ = json["proof"][proof.data.key()].take();

    let digest = serde_jcs::to_vec(&json)
        .map_err(|_| Error::InvalidDocument)
        .map(|canon| sha2::Sha256::digest(&canon))?;

    if digest[..] == verified[..] {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn sign_lds(document: &mut Document, options: SignatureOptions, secret: &[u8]) -> Result<()> {
    let fragment: &str = extract_verification(&options.verification_method)?;
    let keydata: Vec<u8> = resolve_key(document, fragment)?;

    // The verification method key data MUST be equal to the derived public key data.
    if pubkey(&keydata)? != seckey(secret)?.public_key() {
        return Err(Error::InvalidDocument);
    }

    // Create and serialize a proof with a blank signature
    let proof: Value = LdSignature::new(SIGNATURE_TYPE, options).to_json_value()?;

    // Add the proof to the DID document.
    document.metadata_mut().insert("proof".into(), proof);

    // Create an encoded signature
    let signature: String = jcs_sign(&document, secret)?;

    // Add the signature to the proof object within the DID document.
    document
        .metadata_mut()
        .get_mut("proof")
        .ok_or(Error::InvalidSignature)?
        .as_object_mut()
        .ok_or(Error::InvalidSignature)?
        .insert("signatureValue".into(), signature.into());

    Ok(())
}

pub fn verify_lds(document: &Document) -> Result<()> {
    // Extract the verification method from the proof
    let method: &str = document
        .metadata()
        .get("proof")
        .and_then(|proof| proof.as_object())
        .and_then(|proof| proof.get("verificationMethod"))
        .and_then(|method| method.as_str())
        .ok_or(Error::InvalidDocument)?;

    let fragment: &str = extract_verification(method)?;
    let keydata: Vec<u8> = resolve_key(document, fragment)?;

    jcs_verify(document, &keydata)
}

fn extract_verification(method: &str) -> Result<&str> {
    // "Parse" the verification method identifier.
    let fragment: &str = method.trim_start_matches('#');

    // The verification method identifier MUST NOT be empty.
    if fragment.is_empty() {
        Err(Error::InvalidDocument)
    } else {
        Ok(fragment)
    }
}

fn resolve_key(document: &Document, fragment: &str) -> Result<Vec<u8>> {
    // The DID document MUST have a verification method with the specified fragment.
    document
        .resolve_key(fragment, KeyRelation::Authentication)
        .ok_or(Error::InvalidDocument)?
        .key_data()
        .try_decode()
        .ok_or(Error::InvalidDocument)?
        .map_err(|_| Error::InvalidDocument)
}
