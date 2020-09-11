use identity_resolver::{
    resolver::{NetworkNodes, ResolutionInputMetadata, Resolver},
    Result,
};

#[smol_potat::test]
async fn not_supported_method() -> Result<()> {
    let resolver = Resolver::new(NetworkNodes::Com(vec!["http://localhost:14265"])).unwrap();
    let res = resolver
        .resolve(
            "did:test:com:123456789abcdefghij".into(),
            ResolutionInputMetadata::default(),
        )
        .await?;
    assert_eq!(res.metadata.error.unwrap(), "not-supported".to_string());
    Ok(())
}

#[smol_potat::test]
async fn no_node() {
    assert!(Resolver::new(NetworkNodes::Com(vec![])).is_err());
}
