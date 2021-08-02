// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen_test::*;

use identity_wasm::crypto::Digest;
use identity_wasm::crypto::KeyCollection;
use identity_wasm::crypto::KeyPair;
use identity_wasm::crypto::KeyType;
use identity_wasm::error::WasmError;
use identity_wasm::wasm_did::WasmDID;
use identity_wasm::wasm_document::WasmDocument;
use std::borrow::Cow;

#[wasm_bindgen_test]
fn test_keypair() {
  let key1 = KeyPair::new(KeyType::Ed25519).unwrap();
  let pk = key1.public();
  let sk = key1.secret();
  let key2 = KeyPair::from_base58(KeyType::Ed25519, &pk, &sk).unwrap();

  let json1 = key1.to_json().unwrap();
  let json2 = key2.to_json().unwrap();

  let from1 = KeyPair::from_json(&json1).unwrap();
  let from2 = KeyPair::from_json(&json2).unwrap();

  assert_eq!(from1.public(), key1.public());
  assert_eq!(from1.secret(), key1.secret());

  assert_eq!(from2.public(), key2.public());
  assert_eq!(from2.secret(), key2.secret());
}

#[wasm_bindgen_test]
fn test_key_collection() {
  let size = 1 << 5;
  let keys = KeyCollection::new(KeyType::Ed25519, size).unwrap();

  assert_eq!(keys.length(), size);
  assert!(!keys.is_empty());

  for index in 0..keys.length() {
    let key = keys.keypair(index).unwrap();

    assert_eq!(key.public(), keys.public(index).unwrap());
    assert_eq!(key.secret(), keys.secret(index).unwrap());

    assert!(keys.merkle_proof(Digest::Sha256, index).is_some());
  }

  assert!(keys.keypair(keys.length()).is_none());
  assert!(keys.merkle_proof(Digest::Sha256, keys.length()).is_none());

  let json = keys.to_json().unwrap();
  let from = KeyCollection::from_json(&json).unwrap();

  for index in 0..keys.length() {
    assert_eq!(keys.public(index).unwrap(), from.public(index).unwrap());
    assert_eq!(keys.secret(index).unwrap(), from.secret(index).unwrap());
  }
}

#[wasm_bindgen_test]
fn test_js_error_from_wasm_error() {
  let error = WasmError::new(Cow::Borrowed("Some name"), Cow::Owned("Error message".to_owned()));
  let js_error = js_sys::Error::from(error);
  assert_eq!(js_error.name(), "Some name");
  assert_eq!(js_error.message(), "Error message");
}

#[test]
fn test_did() {
  let key = KeyPair::new(KeyType::Ed25519).unwrap();
  let did = WasmDID::new(&key, None).unwrap();

  assert_eq!(did.network_name(), "main");

  let parsed = WasmDID::parse(&did.to_string()).unwrap();

  assert_eq!(did.to_string(), parsed.to_string());

  let public = key.public();
  let base58 = WasmDID::from_base58(&public, Some("test".to_string())).unwrap();

  assert_eq!(base58.tag(), did.tag());
  assert_eq!(base58.network_name(), "test");
}

#[test]
fn test_document() {
  let output = WasmDocument::new(KeyType::Ed25519, None, None).unwrap();

  let mut document = output.doc();
  let keypair = output.key();

  document.sign(&keypair).unwrap();

  assert!(document.verify());
}

#[test]
fn test_document_network() {
  let output = WasmDocument::new(KeyType::Ed25519, Some("test".into()), None).unwrap();
  let document = output.doc();

  assert_eq!(document.id().network_name(), "test");
}
