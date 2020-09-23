use identity_core::{
    document::DIDDocument,
    iota_network,
    utils::{Authentication, Context, KeyData, PublicKey, Subject},
};

use std::str::FromStr;

use json::JsonValue;

const JSON_STR: &str = include_str!("auth.json");

fn setup_json(index: usize) -> String {
    let json_str: JsonValue = json::parse(JSON_STR).unwrap();

    json_str[index].to_string()
}

#[test]
fn test_auth() {
    let json_str = setup_json(0);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_auth(auth1);
    doc_2.update_auth(auth2);
    doc_2.update_auth(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_assertion() {
    let json_str = setup_json(1);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_assert(auth1);
    doc_2.update_assert(auth2);
    doc_2.update_assert(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_verification() {
    let json_str = setup_json(2);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_verification(auth1);
    doc_2.update_verification(auth2);
    doc_2.update_verification(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_delegation() {
    let json_str = setup_json(3);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_delegation(auth1);
    doc_2.update_delegation(auth2);
    doc_2.update_delegation(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_invocation() {
    let json_str = setup_json(4);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_invocation(auth1);
    doc_2.update_invocation(auth2);
    doc_2.update_invocation(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

#[test]
fn test_agreement() {
    let json_str = setup_json(5);

    let doc_1 = DIDDocument::from_str(&json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key_data_2 = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

    let key2 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".into(),
        key_data: key_data_2,
        ..Default::default()
    }
    .init();

    let auth_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-2".into(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: auth_key_data,
        ..Default::default()
    };

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_agreement(auth1);
    doc_2.update_agreement(auth2);
    doc_2.update_agreement(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}

/// test DID creation.
#[test]
fn test_did_creation() {
    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let mut auth_key = PublicKey {
        key_type: "RsaVerificationKey2018".into(),
        key_data,
        ..Default::default()
    }
    .init();
    auth_key.0.create_own_id(iota_network::Comnet, None).unwrap();

    assert_eq!(
        auth_key.0.id.to_did().unwrap().to_string(),
        "did:iota:com:5X7Uq87P7x6P2kdJEiNYun6npfHa21DiozoCWhuJtwPg"
    );
}
