// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_iota_core::IotaDocument;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::MethodScope;
use serde_json::json;

use crate::JwkDocumentExt;
use crate::JwkMemStore;
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

const ISSUER_IOTA_DOCUMENT_JSON: &str = r#"
{
  "doc": {
    "id": "did:iota:tst:0xdfda8bcfb959c3e6ef261343c3e1a8310e9c8294eeafee326a4e96d65dbeaca0"
  },
  "meta": {
    "created": "2023-05-12T15:09:50Z",
    "updated": "2023-05-12T15:09:50Z"
  }
}"#;

pub(super) struct Setup<T: JwkDocumentExt> {
  pub issuer_doc: T,
  pub subject_doc: CoreDocument,
  pub storage: MemStorage,
  pub kid: String,
}

pub(super) async fn setup_iotadocument(fragment: Option<&'static str>) -> Setup<IotaDocument> {
  let mut issuer_doc = IotaDocument::from_json(ISSUER_IOTA_DOCUMENT_JSON).unwrap();
  let subject_doc = CoreDocument::from_json(SUBJECT_DOCUMENT_JSON).unwrap();
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let kid: String = generate_method(&storage, &mut issuer_doc, fragment).await;

  Setup {
    issuer_doc,
    subject_doc,
    storage,
    kid,
  }
}

pub(super) async fn setup_coredocument(fragment: Option<&'static str>) -> Setup<CoreDocument> {
  let mut issuer_doc = CoreDocument::from_json(ISSUER_DOCUMENT_JSON).unwrap();
  let subject_doc = CoreDocument::from_json(SUBJECT_DOCUMENT_JSON).unwrap();
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let kid: String = generate_method(&storage, &mut issuer_doc, fragment).await;

  Setup {
    issuer_doc,
    subject_doc,
    storage,
    kid,
  }
}

async fn generate_method<T>(storage: &MemStorage, document: &mut T, fragment: Option<&'static str>) -> String
where
  T: JwkDocumentExt,
{
  document
    .generate_method(
      storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      fragment,
      MethodScope::assertion_method(),
    )
    .await
    .unwrap()
    .unwrap()
}

pub(super) struct CredentialSetup {
  pub credential: Credential,
  pub issuance_date: Timestamp,
  pub expiration_date: Timestamp,
}

pub(super) fn generate_credential<T: AsRef<CoreDocument>>(
  issuer: T,
  subjects: &[&CoreDocument],
  issuance_date: Option<Timestamp>,
  expiration_date: Option<Timestamp>,
) -> CredentialSetup {
  let issuance_date = issuance_date.unwrap_or_else(|| Timestamp::parse("2020-01-01T00:00:00Z").unwrap());
  let expiration_date = expiration_date.unwrap_or_else(|| Timestamp::parse("2024-01-01T00:00:00Z").unwrap());

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
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732").unwrap())
    .issuer(Url::parse(issuer.as_ref().id().as_str()).unwrap())
    .type_("UniversityDegreeCredential")
    .subjects(credential_subjects)
    .issuance_date(issuance_date)
    .expiration_date(expiration_date)
    .build()
    .unwrap();

  CredentialSetup {
    credential,
    issuance_date,
    expiration_date,
  }
}
