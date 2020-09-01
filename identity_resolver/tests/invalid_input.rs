use identity_core::did::DID;
use identity_resolver::resolver::{NetworkNodes, ResolutionInputMetadata, Resolver};

#[smol_potat::test]
async fn invalid_method() {
    let resolver = Resolver::new(NetworkNodes::Com(vec!["http://localhost:14265"])).unwrap();
    let did = DID::parse_from_str("did:invalid:com:123456789abcdefghij").unwrap();
    assert!(resolver.resolve(did, ResolutionInputMetadata::default()).await.is_err());
}

#[smol_potat::test]
async fn no_node() {
    assert!(Resolver::new(NetworkNodes::Com(vec![])).is_err());
}
