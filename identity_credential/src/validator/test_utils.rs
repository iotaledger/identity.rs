// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::json;
use identity_core::utils::BaseEncoding;
use identity_did::CoreDID;
use identity_did::DID;
use identity_document::document::CoreDocument;
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsOkp;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::jwu;
use identity_verification::VerificationMethod;

use crate::credential::Credential;
use crate::credential::CredentialBuilder;
use crate::credential::Subject;

pub(crate) fn encode_public_ed25519_jwk(public_key: &PublicKey) -> Jwk {
  let x = jwu::encode_b64(public_key.as_ref());
  let mut params = JwkParamsOkp::new();
  params.x = x;
  params.d = None;
  params.crv = EdCurve::Ed25519.name().to_owned();
  let mut jwk = Jwk::from_params(params);
  jwk.set_alg(JwsAlgorithm::EdDSA.name());
  jwk
}

pub(crate) fn generate_jwk_document_with_keys() -> (CoreDocument, SecretKey, String) {
  let secret: SecretKey = SecretKey::generate().unwrap();
  let public: PublicKey = secret.public_key();
  let jwk: Jwk = encode_public_ed25519_jwk(&public);

  let did: CoreDID = CoreDID::parse(format!("did:example:{}", BaseEncoding::encode_base58(&public))).unwrap();
  let fragment: String = "#jwk".to_owned();
  let document: CoreDocument = CoreDocument::builder(Object::new())
    .id(did.clone())
    .verification_method(VerificationMethod::new_from_jwk(did, jwk, Some(&fragment)).unwrap())
    .build()
    .unwrap();
  (document, secret, fragment)
}

pub(crate) fn generate_document_with_keys() -> (CoreDocument, KeyPair) {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let did: CoreDID = CoreDID::parse(format!("did:example:{}", BaseEncoding::encode_base58(keypair.public()))).unwrap();
  let document: CoreDocument = CoreDocument::builder(Object::new())
    .id(did.clone())
    .verification_method(VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "#root").unwrap())
    .build()
    .unwrap();
  (document, keypair)
}

pub(crate) fn generate_credential(
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
