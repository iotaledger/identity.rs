use identity_core::{
    did::DID,
    utils::{Context, PublicKey, Service, Subject},
};

use std::str::FromStr;

#[test]
fn test_context() {
    let raw_str = r#"["https://w3id.org/did/v1","https://w3id.org/security/v1"]"#;
    let ctx = Context::new(vec![
        "https://w3id.org/did/v1".into(),
        "https://w3id.org/security/v1".into(),
    ]);

    let string = serde_json::to_string(&ctx).unwrap();

    assert_eq!(string, raw_str);
}

#[test]
fn test_subject_from_string() {
    let raw_str = r#""did:iota:123456789abcdefghi""#;
    let subject = Subject::new("did:iota:123456789abcdefghi".into()).unwrap();

    let string = serde_json::to_string(&subject).unwrap();

    assert_eq!(string, raw_str);
}

#[test]
fn test_subject_from_did() {
    let did = DID::new(
        "iota".into(),
        vec!["123456".into(), "789011".into()],
        Some(vec![("name".into(), Some("value".into()))]),
        None,
        None,
        None,
    )
    .unwrap();

    let string = format!("\"{}\"", did.to_string());

    let subject = Subject::from_did(did).unwrap();
    let res = serde_json::to_string(&subject).unwrap();

    assert_eq!(res, string);
}

#[test]
fn test_subject_from() {
    let did = DID::new(
        "iota".into(),
        vec!["123456".into(), "789011".into()],
        Some(vec![("name".into(), Some("value".into()))]),
        None,
        None,
        None,
    )
    .unwrap();

    let string = format!("\"{}\"", did.to_string());

    let subject: Subject = did.into();

    let res = serde_json::to_string(&subject).unwrap();

    assert_eq!(res, string);
}

#[test]
fn test_public_key() {
    let raw_str = r#"
    {
        "id":"did:iota:123456789abcdefghi#keys-1",
        "type":"Ed25519VerificationKey2018","controller":
        "did:iota:pqrstuvwxyz0987654321","publicKeyBase58":
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
    }
    "#;
    let pk_t = PublicKey::from_str(raw_str).unwrap();
    let pk = PublicKey::new(
        "did:iota:123456789abcdefghi#keys-1".into(),
        "Ed25519VerificationKey2018".into(),
        "did:iota:pqrstuvwxyz0987654321".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();

    assert_eq!(pk, pk_t);

    let res = serde_json::to_string(&pk).unwrap();

    assert_eq!(res, pk_t.to_string());
}

#[test]
fn test_service_with_no_endpoint_body() {
    let raw_str = r#"{
        "id": "did:into:123#edv",
        "type": "EncryptedDataVault",
        "serviceEndpoint": "https://edv.example.com/"
    }"#;

    let service = Service::new(
        "did:into:123#edv".into(),
        "EncryptedDataVault".into(),
        "https://edv.example.com/".into(),
        None,
        None,
    )
    .unwrap();

    let service_2: Service = Service::from_str(raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_string());
}

#[test]
fn test_service_with_body() {
    let raw_str = r#"
    {
        "id": "did:example:123456789abcdefghi#hub",
        "type": "IdentityHub",
        "publicKey": "did:example:123456789abcdefghi#key-1",
        "serviceEndpoint": {
          "@context": "https://schema.identity.foundation/hub",
          "type": "UserHubEndpoint",
          "instances": ["did:example:456", "did:example:789"]
        }
    }
        "#;

    let service = Service::new(
        "did:example:123456789abcdefghi#hub".into(),
        "IdentityHub".into(),
        "https://schema.identity.foundation/hub".into(),
        Some("UserHubEndpoint".into()),
        Some(vec!["did:example:456".into(), "did:example:789".into()]),
    )
    .unwrap();

    let service_2: Service = Service::from_str(raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_string());
}
