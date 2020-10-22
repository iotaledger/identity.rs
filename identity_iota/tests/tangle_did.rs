use identity_core::did::DID;
use identity_iota::did::TangleDID;

#[test]
fn test_parse_valid() {
    assert!(TangleDID::parse("did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
    assert!(TangleDID::parse("did:iota:main:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
    assert!(TangleDID::parse("did:iota:com:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
    assert!(TangleDID::parse("did:iota:dev:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
    assert!(TangleDID::parse("did:iota:rainbow:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
    assert!(TangleDID::parse("did:iota:rainbow:shard-1:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV").is_ok());
}

#[test]
fn test_parse_invalid() {
    assert!(TangleDID::parse("did:foo::").is_err());
    assert!(TangleDID::parse("did:::").is_err());
    assert!(TangleDID::parse("did:iota---::").is_err());
    assert!(TangleDID::parse("did:iota:").is_err());
}

#[test]
fn test_from_did() {
    let did: DID = "did:iota:123".parse().unwrap();
    assert!(TangleDID::try_from_did(did).is_ok());

    let did: DID = "did:web:123".parse().unwrap();
    assert!(TangleDID::try_from_did(did).is_err());
}

#[test]
fn test_network() {
    let did: TangleDID = "did:iota:dev:123".parse().unwrap();
    assert_eq!(did.network(), "dev");

    let did: TangleDID = "did:iota:123".parse().unwrap();
    assert_eq!(did.network(), "main");

    let did: TangleDID = "did:iota:rainbow:123".parse().unwrap();
    assert_eq!(did.network(), "rainbow");
}

#[test]
fn test_shard() {
    let did: TangleDID = "did:iota:dev:123".parse().unwrap();
    assert_eq!(did.shard(), None);

    let did: TangleDID = "did:iota:dev:shard:123".parse().unwrap();
    assert_eq!(did.shard(), Some("shard"));
}

#[test]
fn test_method_id() {
    let did: TangleDID = "did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".parse().unwrap();
    assert_eq!(did.method_id(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");

    let did: TangleDID = "did:iota:main:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        .parse()
        .unwrap();
    assert_eq!(did.method_id(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");

    let did: TangleDID = "did:iota:main:shard:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        .parse()
        .unwrap();
    assert_eq!(did.method_id(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");
}

#[test]
fn test_normalize() {
    let mut did1: TangleDID = "did:iota:123".parse().unwrap();
    let did2: TangleDID = "did:iota:main:123".parse().unwrap();

    assert!(did1 != did2);
    did1.normalize();
    assert!(did1 == did2);
}
