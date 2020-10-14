use identity_core::{
    common::{AsJson as _, Context, OneOrMany, Url},
    did::{KeyData, KeyType, PublicKey, PublicKeyBuilder, Service, ServiceBuilder, ServiceEndpointBuilder, DID},
};

const JSON_STR: &str = include_str!("fixtures/did/utils.json");

fn setup_json(key: &str) -> String {
    let json_str = json::parse(JSON_STR).unwrap();

    json_str[key].to_string()
}

/// Test Context::new
#[test]
fn test_context() {
    let raw_str = r#"["https://w3id.org/did/v1","https://w3id.org/security/v1"]"#;
    let ctx: OneOrMany<Context> = vec!["https://w3id.org/did/v1".into(), "https://w3id.org/security/v1".into()].into();

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

    let pk_t: PublicKey = serde_json::from_str(&raw_str).unwrap();

    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:pqrstuvwxyz0987654321".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    assert_eq!(public_key, pk_t);
}

/// Test service without ServiceEndpoint body.
#[test]
fn test_service_with_no_endpoint_body() {
    let raw_str = setup_json("service");

    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(Url::parse("https://edv.example.com/").unwrap())
        .build()
        .unwrap();

    let service_2: Service = Service::from_json(&raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_json().unwrap());
}

/// Test Service with a ServiceEndpoint Body.
#[test]
fn test_service_with_body() {
    let raw_str = setup_json("endpoint");

    let endpoint = ServiceEndpointBuilder::default()
        .context("https://schema.identity.foundation/hub".parse().unwrap())
        .endpoint_type("UserHubEndpoint")
        .instances(vec!["did:example:456".into(), "did:example:789".into()])
        .build()
        .unwrap();

    let service = ServiceBuilder::default()
        .id("did:example:123456789abcdefghi#hub".parse().unwrap())
        .service_type("IdentityHub")
        .endpoint(endpoint)
        .build()
        .unwrap();

    let service_2: Service = Service::from_json(&raw_str).unwrap();

    assert_eq!(service, service_2);

    let res = serde_json::to_string(&service).unwrap();

    assert_eq!(res, service_2.to_json().unwrap());
}
