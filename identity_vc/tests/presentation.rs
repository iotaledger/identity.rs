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
        .type_("PrescriptionCredential")
        .try_subject(object!(id: "did:iota:alice"))
        .unwrap()
        .issuance_date(issuance)
        .build()
        .unwrap();

    let verifiable = VerifiableCredential::new(credential, object!());

    let presentation = PresentationBuilder::new()
        .context("https://www.w3.org/2018/credentials/examples/v1")
        .id("did:example:id:123")
        .type_("PrescriptionCredential")
        .credential(verifiable.clone())
        .try_refresh_service(object!(id: "", type: "Refresh2020"))
        .unwrap()
        .try_terms_of_use(object!(type: "Policy2019"))
        .unwrap()
        .try_terms_of_use(object!(type: "Policy2020"))
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(presentation.context.len(), 2);
    assert_matches!(presentation.context.get(0).unwrap(), Context::Uri(ref uri) if uri == Credential::BASE_CONTEXT);
    assert_matches!(presentation.context.get(1).unwrap(), Context::Uri(ref uri) if uri == "https://www.w3.org/2018/credentials/examples/v1");

    assert_eq!(presentation.id, Some("did:example:id:123".into()));

    assert_eq!(presentation.types.len(), 2);
    assert_eq!(presentation.types.get(0).unwrap(), Presentation::BASE_TYPE);
    assert_eq!(presentation.types.get(1).unwrap(), "PrescriptionCredential");

    assert_eq!(presentation.verifiable_credential.len(), 1);
    assert_eq!(presentation.verifiable_credential.get(0).unwrap(), &verifiable);

    assert_eq!(presentation.refresh_service.unwrap().len(), 1);
    assert_eq!(presentation.terms_of_use.unwrap().len(), 2);
}

#[test]
#[should_panic = "Invalid URI for Presentation id"]
fn test_builder_invalid_id_fmt() {
    PresentationBuilder::new()
        .id("foo")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Invalid URI for Presentation holder"]
fn test_builder_invalid_holder_fmt() {
    PresentationBuilder::new()
        .id("did:iota:123")
        .holder("d00m")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Cannot convert `Object` to `RefreshService`"]
fn test_builder_invalid_refresh_service_missing_id() {
    PresentationBuilder::new()
        .id("did:iota:123")
        .try_refresh_service(object!(type: "RefreshServiceType"))
        .unwrap_or_else(|error| panic!("{}", error))
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Cannot convert `Object` to `RefreshService`"]
fn test_builder_invalid_refresh_service_missing_type() {
    PresentationBuilder::new()
        .id("did:iota:123")
        .try_refresh_service(object!(id: "did:iota:rsv:123"))
        .unwrap_or_else(|error| panic!("{}", error))
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Cannot convert `Object` to `TermsOfUse`"]
fn test_builder_invalid_terms_of_use_missing_type() {
    PresentationBuilder::new()
        .id("did:iota:123")
        .try_terms_of_use(object!(id: "did:iota:rsv:123"))
        .unwrap_or_else(|error| panic!("{}", error))
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}
