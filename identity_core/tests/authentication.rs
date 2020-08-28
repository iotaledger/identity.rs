use identity_core::{
    document::DIDDocument,
    utils::{Authentication, Context, PublicKey, Subject},
};

use std::str::FromStr;

#[test]
fn test_auth() {
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "authentication": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

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
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "assertionMethod": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

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
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "verificationMethod": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

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
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "capabilityDelegation": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

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
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "capabilityInvocation": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

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
    let json_str = r#"
    {
        "@context": [
            "https://w3id.org/did/v1",
            "https://w3id.org/security/v1"
        ],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [
            {
                "id": "did:iota:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            },
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ],
        "keyAgreement": [
            "did:iota:123456789abcdefghi#keys-1",
            "did:iota:123456789abcdefghi#biometric-1",
            {
                "id": "did:iota:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:iota:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let doc_1 = DIDDocument::from_str(json_str).unwrap();

    let mut doc_2 = DIDDocument {
        context: Context::new(vec![
            "https://w3id.org/did/v1".into(),
            "https://w3id.org/security/v1".into(),
        ]),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    };

    let key1 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyPem".into(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
    )
    .unwrap();

    let key2 = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    doc_2.update_public_key(key1);
    doc_2.update_public_key(key2);

    let auth_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-2".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    let auth1 = Authentication::Method("did:iota:123456789abcdefghi#keys-1".into());
    let auth2 = Authentication::Method("did:iota:123456789abcdefghi#biometric-1".into());
    let auth3 = Authentication::Key(auth_key);

    doc_2.update_agreement(auth1);
    doc_2.update_agreement(auth2);
    doc_2.update_agreement(auth3);

    let doc_2 = doc_2.init();

    assert_eq!(doc_1, doc_2);
}
