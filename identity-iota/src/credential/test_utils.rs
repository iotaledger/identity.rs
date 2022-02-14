use crate::Result;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::SignatureOptions;
use identity_core::json;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_credential::presentation::PresentationBuilder;
use identity_did::did::DID;
use iota_client::bee_message::MessageId;

use crate::credential::CredentialValidationOptions;
use crate::document::IotaDocument;
use crate::document::ResolvedIotaDocument;

pub(super) fn generate_document_with_keys() -> (IotaDocument, KeyPair) {
  // Generate a new Ed25519 public/private key pair.
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

  // Create a DID Document (an identity) from the generated key pair.
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

pub(super) fn mock_resolved_document(document: IotaDocument) -> ResolvedIotaDocument {
  ResolvedIotaDocument {
    document,
    integration_message_id: MessageId::null(), // not necessary for validation at least not at the moment
    diff_message_id: MessageId::null(),        // not necessary for validation at least not at the moment
  }
}
