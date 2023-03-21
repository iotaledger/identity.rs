// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_document::document::CoreDocument;
use identity_jose::jwk::Jwk;
use identity_jose::jws::Decoder;
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

  // Decode and verify the JWS
  let public_key: &Jwk = document
    .resolve_method(kid.as_deref().unwrap(), Some(MethodScope::VerificationMethod))
    .unwrap()
    .data()
    .public_key_jwk()
    .unwrap();
  assert!(Decoder::new()
    .decode_compact_serialization(jws.as_bytes(), None)
    .and_then(|validation_item| validation_item.verify(&EdDSAJwsSignatureVerifier::default(), public_key))
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
