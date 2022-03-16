// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::json;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_did::did::DID;
use identity_iota_core::document::IotaDocument;

use crate::Result;

pub(super) fn generate_document_with_keys() -> (IotaDocument, KeyPair) {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let document: IotaDocument = IotaDocument::new(&keypair).unwrap();
  (document, keypair)
}

pub(super) fn generate_credential(
  issuer: &IotaDocument,
  subjects: &[IotaDocument],
  issuance_date: Timestamp,
  expiration_date: Timestamp,
) -> Credential {
  let credential_subjects: Result<Vec<Subject>> = subjects
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
      .map_err(Into::into)
    })
    .collect();

  // Build credential using subject above and issuer.
  CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732").unwrap())
    .issuer(Url::parse(issuer.id().as_str()).unwrap())
    .type_("UniversityDegreeCredential")
    .subjects(credential_subjects.unwrap())
    .issuance_date(issuance_date)
    .expiration_date(expiration_date)
    .build()
    .unwrap()
}

// generates a triple: issuer document, issuer's keys, unsigned credential issued by issuer
pub(super) fn credential_setup() -> (IotaDocument, KeyPair, Credential) {
  let (issuer_doc, issuer_key) = generate_document_with_keys();
  let (subject_doc, _) = generate_document_with_keys();
  let issuance_date = Timestamp::parse("2020-01-01T00:00:00Z").unwrap();
  let expiration_date = Timestamp::parse("2023-01-01T00:00:00Z").unwrap();
  let credential = generate_credential(&issuer_doc, &[subject_doc], issuance_date, expiration_date);
  (issuer_doc, issuer_key, credential)
}
