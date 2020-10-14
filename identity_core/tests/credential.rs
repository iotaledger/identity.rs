use identity_core::{
    common::{Context, Timestamp},
    object,
    vc::*,
};

#[test]
fn test_credential_builder_valid() {
    let issuance: Timestamp = "2010-01-01T00:00:00Z".parse().unwrap();

    let subjects = vec![
        CredentialSubjectBuilder::default()
            .id("did:iota:alice")
            .properties(object!(spouse: "did:iota:bob"))
            .build()
            .unwrap(),
        CredentialSubjectBuilder::default()
            .id("did:iota:bob")
            .properties(object!(spouse: "did:iota:alice"))
            .build()
            .unwrap(),
    ];

    let credential = CredentialBuilder::new()
        .issuer("did:example:issuer")
        .context(vec![
            Context::from(Credential::BASE_CONTEXT),
            Context::from("https://www.w3.org/2018/credentials/examples/v1"),
            Context::from(object!(id: "did:context:1234", type: "CustomContext2020")),
        ])
        .id("did:example:123")
        .types(vec![Credential::BASE_TYPE.into(), "RelationshipCredential".into()])
        .subject(subjects)
        .issuance_date(issuance)
        .build()
        .unwrap();

    assert_eq!(credential.context.len(), 3);
    assert!(matches!(credential.context.get(0).unwrap(), Context::Url(ref url) if url == Credential::BASE_CONTEXT));
    assert!(
        matches!(credential.context.get(1).unwrap(), Context::Url(ref url) if url == "https://www.w3.org/2018/credentials/examples/v1")
    );

    assert_eq!(credential.id, Some("did:example:123".into()));

    assert_eq!(credential.types.len(), 2);
    assert_eq!(credential.types.get(0).unwrap(), Credential::BASE_TYPE);
    assert_eq!(credential.types.get(1).unwrap(), "RelationshipCredential");

    assert_eq!(credential.credential_subject.len(), 2);
    assert_eq!(
        credential.credential_subject.get(0).unwrap().id,
        Some("did:iota:alice".into())
    );
    assert_eq!(
        credential.credential_subject.get(1).unwrap().id,
        Some("did:iota:bob".into())
    );

    assert_eq!(credential.issuer.url(), "did:example:issuer");

    assert_eq!(credential.issuance_date, issuance);
}

#[test]
#[should_panic = "Missing Subject"]
fn test_builder_missing_subjects() {
    CredentialBuilder::new()
        .issuer("did:issuer")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "`issuer` must be initialized"]
fn test_builder_missing_issuer() {
    CredentialBuilder::new()
        .subject(CredentialSubjectBuilder::default().id("did:sub").build().unwrap())
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "InvalidUrl"]
fn test_builder_invalid_issuer() {
    CredentialBuilder::new()
        .subject(CredentialSubjectBuilder::default().id("did:sub").build().unwrap())
        .issuer("foo")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}
