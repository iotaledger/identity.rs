use identity_vc::prelude::*;
use serde_json::from_str;

macro_rules! object {
  () => {
    ::identity_vc::common::Object::default()
  };
  ($($key:ident : $value:expr),* $(,)*) => {
    {
      let mut object = ::std::collections::HashMap::new();

      $(
        object.insert(
          stringify!($key).to_string(),
          ::identity_vc::common::Value::from($value),
        );
      )*

      ::identity_vc::common::Object::from(object)
    }
  };
}

macro_rules! assert_matches {
  ($($tt:tt)*) => {
    assert!(matches!($($tt)*))
  };
}

fn try_credential(data: &(impl AsRef<str> + ?Sized)) {
  from_str::<VerifiableCredential>(data.as_ref())
    .unwrap()
    .validate()
    .unwrap()
}

fn try_presentation(data: &(impl AsRef<str> + ?Sized)) {
  from_str::<VerifiablePresentation>(data.as_ref())
    .unwrap()
    .validate()
    .unwrap()
}

#[test]
fn test_parse_credential() {
  try_credential(include_str!("input/example-01.json"));
  try_credential(include_str!("input/example-02.json"));
  try_credential(include_str!("input/example-03.json"));
  try_credential(include_str!("input/example-04.json"));
  try_credential(include_str!("input/example-05.json"));
  try_credential(include_str!("input/example-06.json"));
  try_credential(include_str!("input/example-07.json"));

  try_credential(include_str!("input/example-09.json"));
  try_credential(include_str!("input/example-10.json"));
  try_credential(include_str!("input/example-11.json"));
  try_credential(include_str!("input/example-12.json"));
  try_credential(include_str!("input/example-13.json"));
}

#[test]
fn test_parse_presentation() {
  try_presentation(include_str!("input/example-08.json"));
}

#[test]
fn test_credential_builder() {
  let credential: Credential = CredentialBuilder::new()
    .issuer("did:example:i55u3r")
    .context("https://www.w3.org/2018/credentials/examples/v1")
    .context(object!(id: "did:context:1234", type: "CustomContext2020"))
    .id("did:example:1234567890")
    .type_("RelationshipCredential")
    .subject(object!(id: "did:iota:alice", spouse: "did:iota:bob"))
    .subject(object!(id: "did:iota:bob", spouse: "did:iota:alice"))
    .issuance_date("2010-01-01T19:23:24Z")
    .expiration_date("2020-01-01T19:23:24Z")
    .build()
    .unwrap();

  assert_eq!(credential.issuer.uri(), "did:example:i55u3r");

  assert_eq!(credential.context.len(), 3);
  assert_matches!(credential.context.get(0).unwrap(), Context::URI(ref uri) if uri == Credential::BASE_CONTEXT);

  assert_eq!(credential.id, Some("did:example:1234567890".into()));

  assert_eq!(credential.types.len(), 2);
  assert_eq!(credential.types.get(0).unwrap(), Credential::BASE_TYPE);

  assert_eq!(credential.credential_subject.len(), 2);

  assert_eq!(
    credential.credential_subject.get(0).unwrap()["id"],
    "did:iota:alice".into()
  );

  assert_eq!(
    credential.credential_subject.get(1).unwrap()["id"],
    "did:iota:bob".into()
  );
}

#[test]
#[should_panic = "Not enough subjects"]
fn test_validate_credential_subject_none() {
  CredentialBuilder::new().issuer("did:issuer").build().unwrap();
}

#[test]
#[should_panic = "Invalid credential subject (empty)"]
fn test_validate_credential_subject_empty() {
  CredentialBuilder::new()
    .issuer("did:issuer")
    .subject(object!())
    .build()
    .unwrap();
}

#[test]
#[should_panic = "Missing issuer"]
fn test_validate_issuer_none() {
  CredentialBuilder::new()
    .subject(object!(id: "did:sub"))
    .build()
    .unwrap();
}

#[test]
#[should_panic = "Invalid URI `foo`"]
fn test_validate_issuer_bad_uri_1() {
  CredentialBuilder::new()
    .subject(object!(id: "did:sub"))
    .issuer("foo")
    .build()
    .unwrap();
}

#[test]
#[should_panic = "Invalid URI `did123`"]
fn test_validate_issuer_bad_uri_2() {
  CredentialBuilder::new()
    .subject(object!(id: "did:sub"))
    .issuer("did123")
    .build()
    .unwrap();
}

#[test]
#[should_panic = "Invalid issuance date (empty)"]
fn test_validate_issuance_date_empty() {
  CredentialBuilder::new()
    .issuer("did:issuer")
    .subject(object!(id: "did:sub"))
    .build()
    .unwrap();
}

#[test]
#[should_panic = "Invalid issuance date (premature end of input)"]
fn test_validate_issuance_date_bad_fmt() {
  CredentialBuilder::new()
    .issuer("did:issuer")
    .subject(object!(id: "did:sub"))
    .issuance_date("woo")
    .build()
    .unwrap();
}
