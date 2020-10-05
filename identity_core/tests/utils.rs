use identity_core::did::{Context, KeyData, PublicKey, Service, ServiceEndpoint, DID};

use std::str::FromStr;

const JSON_STR: &str = include_str!("utils.json");

fn setup_json(key: &str) -> String {
    let json_str = json::parse(JSON_STR).unwrap();

    json_str[key].to_string()
}

/// Test Context::new
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

/// Test building a Subject from a did string.
#[test]
fn test_subject_from_string() {
    let raw_str = r#""did:iota:123456789abcdefghi""#;
    let subject: DID = "did:iota:123456789abcdefghi".parse().unwrap();

    let string = serde_json::to_string(&subject).unwrap();

    assert_eq!(string, raw_str);
}

/// Test public key structure from String and with PublicKey::new
#[test]
fn test_public_key() {
    let raw_str = setup_json("public");

    let pk_t = PublicKey::from_str(&raw_str).unwrap();

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".parse().unwrap(),
        key_type: "Ed25519VerificationKey2018".into(),
        controller: "did:iota:pqrstuvwxyz0987654321".parse().unwrap(),
        key_data,
        ..Default::default()
    };

    assert_eq!(public_key, pk_t);

    let res = serde_json::to_string(&public_key).unwrap();

    assert_eq!(res, pk_t.to_string());
}

/// Test service without ServiceEndpoint body.
#[test]
fn test_service_with_no_endpoint_body() {
    let raw_str = setup_json("service");

    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service = Service {
        id: "did:into:123#edv".parse().unwrap(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    };

    let service_2: Service = Service::from_str(&raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_string());
}

/// Test Service with a ServiceEndpoint Body.
#[test]
fn test_service_with_body() {
    let raw_str = setup_json("endpoint");

    let endpoint = ServiceEndpoint {
        context: "https://schema.identity.foundation/hub".into(),
        endpoint_type: Some("UserHubEndpoint".into()),
        instances: Some(vec!["did:example:456".into(), "did:example:789".into()]),
    }
    .init();

    let service = Service {
        id: "did:example:123456789abcdefghi#hub".parse().unwrap(),
        service_type: "IdentityHub".into(),
        endpoint,
    };

    let service_2: Service = Service::from_str(&raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_string());
}
