use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Context, PublicKey, Service},
};

use std::str::FromStr;

use serde_diff::{Apply, Diff};

/// Test a full Did document from String.
#[test]
fn test_parse_document() {
    let json_str = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:iota:123456789abcdefghi",
        "publicKey": [{
            "id": "did:iota:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-2",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:iota:pqrstuvwxyz0987654321",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }, {
            "id": "did:iota:123456789abcdefghi#keys-3",
            "type": "EcdsaSecp256k1VerificationKey2019",
            "controller": "did:iota:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }]
    }
    "#;

    let doc = DIDDocument::from_str(json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let ctx = Context::new(vec![
        "https://w3id.org/did/v1".into(),
        "https://w3id.org/security/v1".into(),
    ]);
    let id = DID::new("iota".into(), vec!["123456789abcdefghi".into()], None, None, None, None).unwrap();

    assert_eq!(doc.context, ctx);
    assert_eq!(doc.id, id.into());
}

/// test doc creation via the `DIDDocument::new` method.
#[test]
fn test_doc_creation() {
    let mut did_doc = DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();
    did_doc.add_service(service.clone());
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    did_doc.add_key_pair(public_key.clone());

    let mut did_doc_2 =
        DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
    did_doc_2.add_service(service);
    did_doc_2.add_key_pair(public_key);

    let did_doc = did_doc.init_timestamps().unwrap();

    // did_doc has timestamps while did_doc_2 does not.
    assert_ne!(did_doc, did_doc_2);
}

/// test serde_diff on the did document structure.
#[test]
fn test_doc_diff() {
    // old doc
    let mut old = DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
    // new doc.
    let mut new = DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();
    new.add_service(service.clone());
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:123456789abcdefghi".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    new.add_key_pair(public_key.clone());

    // diff the two docs and create a json string of the diff.
    let json_diff = serde_json::to_string(&Diff::serializable(&old, &new)).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_diff);

    // apply the json string to the old document.
    Apply::apply(&mut deserializer, &mut old).unwrap();

    // check to see that the old and new docs cotain all of the same fields.
    assert_eq!(new.to_string(), old.to_string());
}
