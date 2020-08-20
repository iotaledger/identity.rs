use identity_core::{did::DID, document::DIDDocument, utils::Context};

use std::str::FromStr;

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
