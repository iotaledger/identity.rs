#[macro_use]
mod macros;

#[macro_use]
extern crate identity_core;

use identity_core::{common::Context, vc::*};

#[test]
fn test_builder_valid() {
    let issuance = timestamp!("2010-01-01T00:00:00Z");

    let subject = CredentialSubjectBuilder::default()
        .id("did:iota:alice")
        .build()
        .unwrap();

    let credential = CredentialBuilder::new()
        .issuer("did:example:issuer")
        .context(vec![
            Context::from(Credential::BASE_CONTEXT),
            Context::from("https://www.w3.org/2018/credentials/examples/v1"),
        ])
        .types(vec![Credential::BASE_TYPE.into(), "PrescriptionCredential".into()])
        .subject(subject)
        .issuance_date(issuance)
        .build()
        .unwrap();

    let verifiable = VerifiableCredential::new(credential, object!());

    let refresh_service = RefreshServiceBuilder::default()
        .id("refresh-service")
        .types("Refresh2020".to_string())
        .build()
        .unwrap();

    let terms = vec![
        TermsOfUseBuilder::default()
            .types("Policy2019".to_string())
            .build()
            .unwrap(),
        TermsOfUseBuilder::default()
            .types("Policy2020".to_string())
            .build()
            .unwrap(),
    ];

    let presentation = PresentationBuilder::new()
        .context(vec![
            Context::from(Presentation::BASE_CONTEXT),
            Context::from("https://www.w3.org/2018/credentials/examples/v1"),
        ])
        .id("did:example:id:123")
        .types(vec![Presentation::BASE_TYPE.into(), "PrescriptionCredential".into()])
        .credential(verifiable.clone())
        .refresh_service(refresh_service)
        .terms_of_use(terms)
        .build()
        .unwrap();

    assert_eq!(presentation.context.len(), 2);
    assert_matches!(presentation.context.get(0).unwrap(), Context::Uri(ref uri) if uri == Presentation::BASE_CONTEXT);
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
#[should_panic = "Invalid ID"]
fn test_builder_invalid_id_fmt() {
    PresentationBuilder::new()
        .id("foo")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}

#[test]
#[should_panic = "Invalid Holder"]
fn test_builder_invalid_holder_fmt() {
    PresentationBuilder::new()
        .id("did:iota:123")
        .holder("d00m")
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
}
