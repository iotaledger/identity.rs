#[macro_use]
extern crate identity_core;

#[macro_use]
mod macros;

use identity_vc::prelude::*;

#[test]
fn test_builder_valid() {
    let issuance = timestamp!("2010-01-01T00:00:00Z");

    let credential = CredentialBuilder::new()
        .issuer("did:example:issuer")
        .context("https://www.w3.org/2018/credentials/examples/v1")
        .context(object!(id: "did:context:1234", type: "CustomContext2020"))
        .id("did:example:123")
        .type_("RelationshipCredential")
        .try_subject(object!(id: "did:iota:alice", spouse: "did:iota:bob"))
        .unwrap()
        .try_subject(object!(id: "did:iota:bob", spouse: "did:iota:alice"))
        .unwrap()
        .issuance_date(issuance)
        .build()
        .unwrap();

    assert_eq!(credential.context.len(), 3);
    assert_matches!(credential.context.get(0).unwrap(), Context::Uri(ref uri) if uri == Credential::BASE_CONTEXT);
    assert_matches!(credential.context.get(1).unwrap(), Context::Uri(ref uri) if uri == "https://www.w3.org/2018/credentials/examples/v1");

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

    assert_eq!(credential.issuer.uri(), "did:example:issuer");

    assert_eq!(credential.issuance_date, issuance);
}

#[test]
#[should_panic = "Missing `Credential` subject"]
fn test_builder_missing_subjects() {
    CredentialBuilder::new()
        .issuer("did:issuer")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Invalid `Credential` subject"]
fn test_builder_invalid_subjects() {
    CredentialBuilder::new()
        .issuer("did:issuer")
        .try_subject(object!())
        .unwrap_or_else(|error| panic!("{}", error))
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Missing `Credential` issuer"]
fn test_builder_missing_issuer() {
    CredentialBuilder::new()
        .try_subject(object!(id: "did:sub"))
        .unwrap_or_else(|error| panic!("{}", error))
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Invalid URI for Credential issuer"]
fn test_builder_invalid_issuer() {
    CredentialBuilder::new()
        .try_subject(object!(id: "did:sub"))
        .unwrap_or_else(|error| panic!("{}", error))
        .issuer("foo")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}
