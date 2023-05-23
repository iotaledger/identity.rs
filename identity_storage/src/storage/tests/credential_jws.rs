// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;

use identity_credential::validator::vc_jwt_validation::CredentialValidationOptions;
use identity_document::document::CoreDocument;
use identity_document::verifiable::JwsVerificationOptions;
use identity_verification::jose::jws::JwsAlgorithm;
use identity_verification::MethodScope;

use crate::key_id_storage::KeyIdMemstore;
use crate::key_storage::JwkMemStore;
use crate::storage::JwsOptions;

use crate::storage::JwkDocumentExt;
use crate::Storage;

type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

const MOCK_DOCUMENT_JSON: &str = r#"
{
    "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
    "verificationMethod": [
      {
        "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
        "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "zHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
      }
    ]
}"#;

async fn setup() -> (CoreDocument, MemStorage, String, Credential) {
  let mut mock_document = CoreDocument::from_json(MOCK_DOCUMENT_JSON).unwrap();
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());

  // Generate a method with the kid as fragment
  let kid: Option<String> = mock_document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::assertion_method(),
    )
    .await
    .unwrap();

  let credential_json: &str = r#"
    {
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
      "issuanceDate": "2010-01-01T19:23:24Z",
      "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science in Mechanical Engineering"
        }
      }
    }"#;

  let credential: Credential = Credential::from_json(credential_json).unwrap();

  (mock_document, storage, kid.unwrap(), credential)
}

#[tokio::test]
async fn signing_credential_with_detached_option_fails() {
  let (document, storage, kid, credential) = setup().await;

  assert!(document
    .sign_credential(
      &credential,
      &storage,
      kid.as_ref(),
      &JwsOptions::default().detached_payload(true),
    )
    .await
    .is_err());
}

#[tokio::test]
async fn signing_credential_with_nonce_and_scope() {
  let (document, storage, kid, credential) = setup().await;
  let nonce: &str = "0xaabbccddeeff";

  let jws = document
    .sign_credential(
      &credential,
      &storage,
      kid.as_ref(),
      &JwsOptions::default().nonce(nonce.to_owned()),
    )
    .await
    .unwrap();

  let validator = identity_credential::validator::vc_jwt_validation::CredentialValidator::new();
  assert!(validator
    .validate::<_, Object>(
      &jws,
      &document,
      &CredentialValidationOptions::default().verification_options(
        JwsVerificationOptions::default()
          .nonce(nonce.to_owned())
          .method_scope(MethodScope::assertion_method())
      ),
      identity_credential::validator::FailFast::FirstError,
    )
    .is_ok());

  // Invalid: Nonce mismatch.
  assert!(validator
    .validate::<_, Object>(
      &jws,
      &document,
      &CredentialValidationOptions::default().verification_options(
        JwsVerificationOptions::default()
          .nonce("other-nonce".to_owned())
          .method_scope(MethodScope::assertion_method())
      ),
      identity_credential::validator::FailFast::FirstError,
    )
    .is_err());

  // Invalid: MethodScope mismatch.
  assert!(validator
    .validate::<_, Object>(
      &jws,
      &document,
      &CredentialValidationOptions::default().verification_options(
        JwsVerificationOptions::default()
          .nonce(nonce.to_owned())
          .method_scope(MethodScope::key_agreement())
      ),
      identity_credential::validator::FailFast::FirstError,
    )
    .is_err());
}

#[tokio::test]
async fn signing_credential_with_b64() {
  let (document, storage, kid, credential) = setup().await;

  let jws = document
    .sign_credential(&credential, &storage, kid.as_ref(), &JwsOptions::default().b64(true))
    .await
    .unwrap();

  let validator = identity_credential::validator::vc_jwt_validation::CredentialValidator::new();
  let decoded = validator
    .validate::<_, Object>(
      &jws,
      &document,
      &CredentialValidationOptions::default(),
      identity_credential::validator::FailFast::FirstError,
    )
    .unwrap();

  // Setting `options.b64 = true` should result in it not being added to the header as a parameter or as a crit,
  // as recommended in https://datatracker.ietf.org/doc/html/rfc7797#section-7
  assert!(decoded.header.b64().is_none());
  assert!(decoded.header.crit().is_none());

  // JWTs should not have `b64` set per https://datatracker.ietf.org/doc/html/rfc7797#section-7.
  assert!(document
    .sign_credential(&credential, &storage, kid.as_ref(), &JwsOptions::default().b64(false),)
    .await
    .is_err());
}
