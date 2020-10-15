use identity_core::{
    common::{AsJson as _, Object, Timestamp},
    utils::{decode_b64, encode_b64},
};
use identity_crypto::{Ed25519, KeyPair, Proof, Secp256k1};
use identity_proof::signature::{
    EcdsaSecp256k1Signature2019, Ed25519Signature2018, JcsEd25519Signature2020, RsaSignature2018, SignatureOptions,
    SignatureProof, SignatureSuite,
};
use serde_json::json;
use std::convert::TryFrom;

macro_rules! keypair {
    ($suite:expr) => {
        KeyPair::generate(&$suite).unwrap()
    };
}

macro_rules! badpair {
    ($suite:expr) => {
        KeyPair::new(Vec::new().into(), KeyPair::generate(&$suite).unwrap().secret().clone())
    };
}

// Generated with:
//
// openssl genrsa 2048 | openssl pkcs8 -topk8 -nocrypt -outform DER > rsa-seckey.pk8
// openssl rsa -pubout -in rsa-seckey.pk8 -inform DER -RSAPublicKey_out -outform DER -out rsa-pubkey.pk8
const RSA_PUBKEY: &[u8] = include_bytes!("fixtures/keys/rsa-pubkey.pk8");
const RSA_SECKEY: &[u8] = include_bytes!("fixtures/keys/rsa-seckey.pk8");

fn rsa_keypair() -> KeyPair {
    KeyPair::new(RSA_PUBKEY.to_vec().into(), RSA_SECKEY.to_vec().into())
}

fn rsa_badpair() -> KeyPair {
    KeyPair::new(Vec::new().into(), rsa_keypair().secret().clone())
}

fn test_sign_and_verify(suite: impl SignatureSuite, keypair: &KeyPair, property: &str) {
    let created = Timestamp::try_from("2020-01-01T00:00:00Z").unwrap();
    let input = json!({"foo": "hello"});

    let options = SignatureOptions {
        created: created.into(),
        purpose: Some("authentication".into()),
        ..Default::default()
    };

    let suite = SignatureProof::with_options(suite, options);
    let proof = suite.create(&input, keypair.secret()).unwrap();
    let json = proof.to_json().unwrap();
    let object = Object::from_json(&json).unwrap();
    let signature = object[property].as_str().unwrap();

    assert_eq!(object["created"], created.to_string());
    assert_eq!(object["proofPurpose"], "authentication");
    assert!(!signature.is_empty());

    if property == "jws" {
        let parts = signature.split('.').collect::<Vec<_>>();

        assert!(parts.len() == 3);
        assert!(parts[1].is_empty());
        assert!(decode_b64(parts[0]).is_ok());
        assert!(decode_b64(parts[2]).is_ok());
    }

    assert!(suite.verify(&input, &object, keypair.public()).unwrap());
}

fn test_modified_input(suite: impl SignatureSuite, keypair: &KeyPair) {
    let input1 = json!({"foo": "hello"});
    let input2 = json!({"foo": "helloo"});
    let suite = SignatureProof::new(suite);
    let proof = suite.create(&input1, keypair.secret()).unwrap();
    let json = proof.to_json().unwrap();
    let object = Object::from_json(&json).unwrap();

    assert!(!suite.verify(&input2, &object, keypair.public()).unwrap());
}

fn test_modified_signature(suite: impl SignatureSuite, keypair: &KeyPair, property: &str) {
    let input = json!({"foo": "hello"});
    let suite = SignatureProof::new(suite);
    let proof = suite.create(&input, keypair.secret()).unwrap();
    let json = proof.to_json().unwrap();
    let mut object = Object::from_json(&json).unwrap();

    let current = object[property].as_str().unwrap();

    if property == "jws" {
        let parts = current.split('.').collect::<Vec<_>>();
        let mut bytes = decode_b64(&parts[2]).unwrap();

        bytes.reverse();

        let update = format!("{}..{}", &parts[0], encode_b64(&bytes));

        object.insert(property.into(), update.into());
    } else {
        let update = format!("{}1", &current[..current.len() - 1]);

        object.insert(property.into(), update.into());
    }

    assert!(!suite.verify(&input, &object, keypair.public()).unwrap());
}

fn test_empty_signature(suite: impl SignatureSuite, keypair: &KeyPair, property: &str) {
    let input = json!({"foo": "bar"});
    let suite = SignatureProof::new(suite);
    let proof = suite.create(&input, keypair.secret()).unwrap();
    let json = proof.to_json().unwrap();
    let mut object = Object::from_json(&json).unwrap();

    object.insert(property.into(), "".into());

    let result = suite.verify(&input, &object, keypair.public());

    assert!(matches!(result, Ok(false) | Err(_)));
}

fn test_invalid_public_key(suite: impl SignatureSuite, keypair: &KeyPair) {
    let input = json!({"foo": "bar"});
    let suite = SignatureProof::new(suite);
    let proof = suite.create(&input, keypair.secret()).unwrap();
    let json = proof.to_json().unwrap();
    let object = Object::from_json(&json).unwrap();
    let result = suite.verify(&input, &object, keypair.public());

    assert!(matches!(result, Ok(false) | Err(_)));
}

//
// EcdsaSecp256k1Signature2019
//

#[test]
fn test_ecdsasecp256k1signature2019_sign_and_verify() {
    test_sign_and_verify(EcdsaSecp256k1Signature2019, &keypair!(Secp256k1), "jws");
}

#[test]
fn test_ecdsasecp256k1signature2019_modified_input() {
    test_modified_input(EcdsaSecp256k1Signature2019, &keypair!(Secp256k1));
}

#[test]
fn test_ecdsasecp256k1signature2019_modified_signature() {
    test_modified_signature(EcdsaSecp256k1Signature2019, &keypair!(Secp256k1), "jws");
}

#[test]
fn test_ecdsasecp256k1signature2019_empty_signature() {
    test_empty_signature(EcdsaSecp256k1Signature2019, &keypair!(Secp256k1), "jws");
}

#[test]
fn test_ecdsasecp256k1signature2019_invalid_public_key() {
    test_invalid_public_key(EcdsaSecp256k1Signature2019, &badpair!(Secp256k1));
}

//
// Ed25519Signature2018
//

#[test]
fn test_ed25519signature2018_sign_and_verify() {
    test_sign_and_verify(Ed25519Signature2018, &keypair!(Ed25519), "jws");
}

#[test]
fn test_ed25519signature2018_modified_input() {
    test_modified_input(Ed25519Signature2018, &keypair!(Ed25519));
}

#[test]
fn test_ed25519signature2018_modified_signature() {
    test_modified_signature(Ed25519Signature2018, &keypair!(Ed25519), "jws");
}

#[test]
fn test_ed25519signature2018_empty_signature() {
    test_empty_signature(Ed25519Signature2018, &keypair!(Ed25519), "jws");
}

#[test]
fn test_ed25519signature2018_invalid_public_key() {
    test_invalid_public_key(Ed25519Signature2018, &badpair!(Ed25519));
}

//
// JcsEd25519Signature2020
//

#[test]
fn test_jcsed25519signature2020_sign_and_verify() {
    test_sign_and_verify(JcsEd25519Signature2020, &keypair!(Ed25519), "signatureValue");
}

#[test]
fn test_jcsed25519signature2020_modified_input() {
    test_modified_input(JcsEd25519Signature2020, &keypair!(Ed25519));
}

#[test]
fn test_jcsed25519signature2020_modified_signature() {
    test_modified_signature(JcsEd25519Signature2020, &keypair!(Ed25519), "signatureValue");
}

#[test]
fn test_jcsed25519signature2020_empty_signature() {
    test_empty_signature(JcsEd25519Signature2020, &keypair!(Ed25519), "signatureValue");
}

#[test]
fn test_jcsed25519signature2020_invalid_public_key() {
    test_invalid_public_key(JcsEd25519Signature2020, &badpair!(Ed25519));
}

//
// RsaSignature2018
//

#[test]
fn test_rsasignature2018_sign_and_verify() {
    test_sign_and_verify(RsaSignature2018, &rsa_keypair(), "jws");
}

#[test]
fn test_rsasignature2018_modified_input() {
    test_modified_input(RsaSignature2018, &rsa_keypair());
}

#[test]
fn test_rsasignature2018_modified_signature() {
    test_modified_signature(RsaSignature2018, &rsa_keypair(), "jws");
}

#[test]
fn test_rsasignature2018_empty_signature() {
    test_empty_signature(RsaSignature2018, &rsa_keypair(), "jws");
}

#[test]
fn test_rsasignature2018_invalid_public_key() {
    test_invalid_public_key(RsaSignature2018, &rsa_badpair());
}
