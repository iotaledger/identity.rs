use identity_core::{
    did::{Authentication, DIDDocument, DIDDocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKeyBuilder},
};
use json::JsonValue;
use std::str::FromStr;

const JSON_STR: &str = include_str!("fixtures/did/auth.json");

fn setup_json(index: usize) -> String {
    let json_str: JsonValue = json::parse(JSON_STR).unwrap();

    json_str[index].to_string()
}

#[test]
fn test_auth() {
    let json_str = setup_json(0);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_auth(auth1);
    doc_2.update_auth(auth2);
    doc_2.update_auth(auth3);

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_assertion() {
    let json_str = setup_json(1);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_assert(auth1);
    doc_2.update_assert(auth2);
    doc_2.update_assert(auth3);

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_verification() {
    let json_str = setup_json(2);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_verification(auth1);
    doc_2.update_verification(auth2);
    doc_2.update_verification(auth3);

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_delegation() {
    let json_str = setup_json(3);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_delegation(auth1);
    doc_2.update_delegation(auth2);
    doc_2.update_delegation(auth3);

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_invocation() {
    let json_str = setup_json(4);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_invocation(auth1);
    doc_2.update_invocation(auth2);
    doc_2.update_invocation(auth3);

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_agreement() {
    let json_str = setup_json(5);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let key_data_1 = KeyData::PublicKeyPem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::PublicKeyBase58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::RsaVerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(key_data_1)
        .build()
        .unwrap();

    let key2 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(key_data_2)
        .build()
        .unwrap();

    let auth_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(auth_key_data)
        .build()
        .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::DID("did:iota:123456789abcdefghi#keys-1".parse().unwrap());
    let auth2 = Authentication::DID("did:iota:123456789abcdefghi#biometric-1".parse().unwrap());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_agreement(auth1);
    doc_2.update_agreement(auth2);
    doc_2.update_agreement(auth3);

    assert_eq!(doc_1, doc_2);
}
