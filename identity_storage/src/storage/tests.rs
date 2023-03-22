// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_document::document::CoreDocument;
use identity_document::verifiable::JwsVerificationOptions;
use identity_jose::jws::EdDSAJwsSignatureVerifier;
use identity_jose::jws::JwsAlgorithm;
use identity_verification::MethodRelationship;
use identity_verification::MethodScope;

use crate::key_id_storage::KeyIdMemstore;
use crate::key_storage::JwkMemStore;
use crate::storage::JwkStorageDocumentSignatureOptions;

use super::JwkStorageDocumentExt;
use super::Storage;

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

fn setup() -> (CoreDocument, MemStorage) {
  let mock_document = CoreDocument::from_json(MOCK_DOCUMENT_JSON).unwrap();
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());
  (mock_document, storage)
}

#[tokio::test]
async fn generation() {
  let (mut document, storage) = setup();
  // Insert a method whose key material is backed by storage and check that it resolves.
  let fragment = "#key-1";
  let _kid = document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      Some(fragment),
      MethodScope::VerificationMethod,
    )
    .await
    .unwrap();
  assert!(document.resolve_method(fragment, None).is_some());

  // Insert a method backed by storage without passing a fragment
  let kid: Option<String> = document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod),
    )
    .await
    .unwrap();
  // Check that the method can be resolved by passing the `kid` as a fragment
  assert!(document
    .resolve_method(
      kid.as_deref().unwrap(),
      Some(MethodScope::VerificationRelationship(
        MethodRelationship::AssertionMethod
      ))
    )
    .is_some());
}

#[tokio::test]
async fn signing() {
  let (mut document, storage) = setup();
  // Generate a method with the kid as fragment
  let kid: Option<String> = document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await
    .unwrap();
  let payload = b"test";

  // TODO: Check with more Options
  let options = JwkStorageDocumentSignatureOptions::new();
  let jws = document
    .sign_bytes(&storage, kid.as_deref().unwrap(), payload, &options)
    .await
    .unwrap();

  assert!(document
    .verify_jws(
      &jws,
      &EdDSAJwsSignatureVerifier::default(),
      &JwsVerificationOptions::default()
    )
    .is_ok());
}

#[tokio::test]
async fn purging() {
  let (mut document, storage) = setup();
  let kid: Option<String> = document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await
    .unwrap();
  let method_id = document
    .resolve_method(kid.as_deref().unwrap(), None)
    .unwrap()
    .id()
    .to_owned();
  assert!(document.purge_method(&storage, &method_id).await.is_ok());
  // Check that the method is no longer contained in the document
  assert!(document.resolve_method(kid.as_deref().unwrap(), None).is_none());
  // there should now be no items left in the storage
  assert_eq!(storage.key_id_storage().count().await, 0);
  assert_eq!(storage.key_storage().count().await, 0);
}

#[cfg(feature = "iota-document")]
mod iota_document_tests {
  // Write a single test for the IotaDocument case just to check that it works
  use super::*;
  use identity_did::DIDUrl;
  use identity_did::DID;
  use identity_document::verifiable::JwsVerificationOptions;
  use identity_iota_core::IotaDocument;
  #[tokio::test]
  async fn iota_document_document_jwk_storage_extension() {
    // Construct IotaDocument from json
    const DOC_JSON: &str = r#"
    {
      "doc": {
        "id": "did:iota:rms:0x7591a0bc872e3a4ab66228d65773961a7a95d2299ec8464331c80fcd86b35f38",
        "controller": "did:iota:rms:0xfbaaa919b51112d51a8f18b1500d98f0b2e91d793bc5b27fd5ab04cb1b806343"
      },
      "meta": {
        "created": "2023-01-25T15:48:09Z",
        "updated": "2023-01-25T15:48:09Z",
        "governorAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu",
        "stateControllerAddress": "rms1pra642gek5g394g63uvtz5qdnrct96ga0yautvnl6k4sfjcmsp35xv6nagu"
      }
    }
    "#;
    let mut iota_document: IotaDocument = IotaDocument::from_json(DOC_JSON).unwrap();
    // Initialize storage
    let storage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
    let fragment = "#key-1";
    // Generate method
    let _kid = iota_document
      .generate_method(
        &storage,
        JwkMemStore::ED25519_KEY_TYPE,
        JwsAlgorithm::EdDSA,
        Some(fragment),
        MethodScope::VerificationMethod,
      )
      .await
      .unwrap();

    // Sign the test string
    let jws = iota_document
      .sign_bytes(&storage, fragment, b"test", &JwkStorageDocumentSignatureOptions::new())
      .await
      .unwrap();

    // Verify the JWS
    let result = iota_document.verify_jws(
      &jws,
      &EdDSAJwsSignatureVerifier::default(),
      &JwsVerificationOptions::default(),
    );
    assert!(result.is_ok());

    // Check that the claims contain the expected string
    assert!(std::str::from_utf8(&result.unwrap().claims).unwrap().contains("test"));

    // Remove the method from the document
    let method_id: DIDUrl = iota_document.id().to_url().join(fragment).unwrap();
    assert!(iota_document.purge_method(&storage, &method_id).await.is_ok());
    assert!(iota_document.resolve_method(method_id, None).is_none());
    // The storage should now be empty
    assert_eq!(storage.key_id_storage().count().await, 0);
    assert_eq!(storage.key_storage().count().await, 0);
  }
}
