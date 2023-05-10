use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::MethodScope;
use serde_json::json;

use crate::JwkMemStore;
use crate::JwkStorageDocumentExt;
use crate::KeyIdMemstore;
use crate::Storage;

type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

const ISSUER_DOCUMENT_JSON: &str = r#"
{
    "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
}"#;

const SUBJECT_DOCUMENT_JSON: &str = r#"
{
    "id": "did:foo:0xabcdef"
}"#;

pub struct Setup {
  pub issuer_doc: CoreDocument,
  pub subject_doc: CoreDocument,
  pub storage: MemStorage,
  pub kid: String,
}

pub async fn setup() -> Setup {
  let mut issuer_doc = CoreDocument::from_json(ISSUER_DOCUMENT_JSON).unwrap();
  let subject_doc = CoreDocument::from_json(SUBJECT_DOCUMENT_JSON).unwrap();
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());

  // Generate a method with the kid as fragment.
  let kid: String = issuer_doc
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::assertion_method(),
    )
    .await
    .unwrap()
    .unwrap();

  Setup {
    issuer_doc,
    subject_doc,
    storage,
    kid,
  }
}

pub(super) fn generate_credential(
  issuer: &CoreDocument,
  subjects: &[CoreDocument],
  issuance_date: Timestamp,
  expiration_date: Timestamp,
) -> Credential {
  let credential_subjects: Vec<Subject> = subjects
    .iter()
    .map(|subject| {
      Subject::from_json_value(json!({
        "id": subject.id().as_str(),
        "name": "Alice",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science and Arts",
        },
        "GPA": "4.0",
      }))
      .unwrap()
    })
    .collect();

  // Build credential using subject above and issuer.
  CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732").unwrap())
    .issuer(Url::parse(issuer.id().as_str()).unwrap())
    .type_("UniversityDegreeCredential")
    .subjects(credential_subjects)
    .issuance_date(issuance_date)
    .expiration_date(expiration_date)
    .build()
    .unwrap()
}
