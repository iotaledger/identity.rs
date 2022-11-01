// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::json;
use identity_core::utils::BaseEncoding;
use identity_did::did::CoreDID;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::VerificationMethod;

use crate::credential::Credential;
use crate::credential::CredentialBuilder;
use crate::credential::Subject;

pub(super) fn generate_document_with_keys() -> (CoreDocument, KeyPair) {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let did: CoreDID = CoreDID::parse(format!(
    "did:example:{}",
    BaseEncoding::encode_base58(keypair.public())
  ))
  .unwrap();
  let document: CoreDocument = CoreDocument::builder(Object::new())
    .id(did.clone())
    .verification_method(VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "#root").unwrap())
    .build()
    .unwrap();
  (document, keypair)
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

// generates a triple: issuer document, issuer's keys, unsigned credential issued by issuer
pub(super) fn credential_setup() -> (CoreDocument, KeyPair, Credential) {
  let (issuer_doc, issuer_key) = generate_document_with_keys();
  let (subject_doc, _) = generate_document_with_keys();
  let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
  let expiration_date = Timestamp::parse("2023-01-01T00:00:00Z").unwrap();
  let credential = generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);
  (issuer_doc, issuer_key, credential)
}
