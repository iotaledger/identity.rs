// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity::iota::IotaDID;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use identity_wasm::crypto::Digest;
use identity_wasm::crypto::KeyCollection;
use identity_wasm::crypto::KeyPair;
use identity_wasm::crypto::KeyType;
use identity_wasm::did::WasmDID;
use identity_wasm::did::WasmDIDUrl;
use identity_wasm::did::WasmDocument;
use identity_wasm::did::WasmMethodScope;
use identity_wasm::did::WasmVerificationMethod;
use identity_wasm::error::WasmError;

#[wasm_bindgen_test]
fn test_keypair() {
  let key1 = KeyPair::new(KeyType::Ed25519).unwrap();
  let public_key = key1.public();
  let private_key = key1.private();
  let key2 = KeyPair::from_base58(KeyType::Ed25519, &public_key, &private_key).unwrap();

  let json1 = key1.to_json().unwrap();
  let json2 = key2.to_json().unwrap();

  let from1 = KeyPair::from_json(&json1).unwrap();
  let from2 = KeyPair::from_json(&json2).unwrap();

  assert_eq!(from1.public(), key1.public());
  assert_eq!(from1.private(), key1.private());

  assert_eq!(from2.public(), key2.public());
  assert_eq!(from2.private(), key2.private());
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
    assert_eq!(key.private(), keys.private(index).unwrap());

    assert!(keys.merkle_proof(Digest::Sha256, index).is_some());
  }

  assert!(keys.keypair(keys.length()).is_none());
  assert!(keys.merkle_proof(Digest::Sha256, keys.length()).is_none());

  let json = keys.to_json().unwrap();
  let from = KeyCollection::from_json(&json).unwrap();

  for index in 0..keys.length() {
    assert_eq!(keys.public(index).unwrap(), from.public(index).unwrap());
    assert_eq!(keys.private(index).unwrap(), from.private(index).unwrap());
  }
}

#[wasm_bindgen_test]
fn test_js_error_from_wasm_error() {
  let error = WasmError::new(Cow::Borrowed("Some name"), Cow::Owned("Error message".to_owned()));
  let js_error = js_sys::Error::from(error);
  assert_eq!(js_error.name(), "Some name");
  assert_eq!(js_error.message(), "Error message");
}

#[wasm_bindgen_test]
fn test_did() {
  let key = KeyPair::new(KeyType::Ed25519).unwrap();
  let did = WasmDID::new(&key, None).unwrap();

  assert_eq!(did.network_name(), "main");

  let parsed = WasmDID::parse(&did.to_string()).unwrap();

  assert_eq!(did.to_string(), parsed.to_string());

  let public = key.public();
  let base58 = WasmDID::from_base58(&public, Some("dev".to_owned())).unwrap();

  assert_eq!(base58.tag(), did.tag());
  assert_eq!(base58.network_name(), "dev");
}

#[wasm_bindgen_test]
fn test_did_url() {
  // Base DID Url
  let key = KeyPair::new(KeyType::Ed25519).unwrap();
  let did = WasmDID::new(&key, None).unwrap();
  let did_url = did.to_url();

  assert_eq!(did.to_string(), did_url.to_string());

  let parsed_from_did = WasmDIDUrl::parse(&did.to_string()).unwrap();
  let parsed_from_did_url = WasmDIDUrl::parse(&did_url.to_string()).unwrap();

  assert_eq!(did_url.to_string(), parsed_from_did.to_string());
  assert_eq!(did_url.to_string(), parsed_from_did_url.to_string());

  // DID Url segments
  let joined_did_url = did_url.join("/path?query#fragment").unwrap();
  assert_eq!(joined_did_url.path().unwrap(), "/path");
  assert_eq!(joined_did_url.query().unwrap(), "query");
  assert_eq!(joined_did_url.fragment().unwrap(), "fragment");
  assert_eq!(
    joined_did_url.to_string(),
    format!("{}{}", did.to_string(), "/path?query#fragment")
  );
}

#[wasm_bindgen_test]
fn test_document_new() {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
  assert_eq!(document.id().network_name(), "main");
  assert!(document.default_signing_method().is_ok());
}

#[wasm_bindgen_test]
fn test_document_sign_self() {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();

  // Sign with DIDUrl method query.
  {
    let mut document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
    document
      .sign_self(
        &keypair,
        &JsValue::from(document.default_signing_method().unwrap().id()).unchecked_into(),
      )
      .unwrap();
    assert!(WasmDocument::verify_document(&document, &document).is_ok());
  }

  // Sign with string method query.
  {
    let mut document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
    document
      .sign_self(
        &keypair,
        &JsValue::from_str(&document.default_signing_method().unwrap().id().to_string()).unchecked_into(),
      )
      .unwrap();
    assert!(WasmDocument::verify_document(&document, &document).is_ok());
  }
}

#[wasm_bindgen_test]
fn test_document_resolve_method() {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let mut document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
  let default_method: WasmVerificationMethod = document.default_signing_method().unwrap();
  let new_method: WasmVerificationMethod =
    WasmVerificationMethod::new(&KeyPair::new(KeyType::Ed25519).unwrap(), "new-key").unwrap();
  document
    .insert_method(&new_method, WasmMethodScope::authentication())
    .unwrap();

  // Resolve with DIDUrl method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from(default_method.id()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(&JsValue::from(new_method.id()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    new_method.id().to_string()
  );

  // Resolve with string method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&default_method.id().to_string()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&new_method.id().to_string()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    new_method.id().to_string()
  );

  // Resolve with string fragment method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&default_method.id().fragment().unwrap()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&new_method.id().fragment().unwrap()).unchecked_into())
      .unwrap()
      .id()
      .to_string(),
    new_method.id().to_string()
  );
}

#[wasm_bindgen_test]
fn test_document_network() {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let document: WasmDocument = WasmDocument::new(&keypair, Some("dev".to_owned()), None).unwrap();

  assert_eq!(document.id().network_name(), "dev");
}

#[wasm_bindgen_test]
fn test_did_serde() {
  // Ensure JSON deserialization of DID and strings match (for UWasmDID duck-typed parameters).
  let expected_str: &str = "did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV";
  let expected: IotaDID = IotaDID::parse(expected_str).unwrap();

  // Check WasmDID deserialization.
  {
    let wasm_did: WasmDID = WasmDID::from(expected.clone());
    let de: IotaDID = wasm_did.to_json().into_serde().unwrap();
    assert_eq!(de, expected);
  }

  // Check String JsValue deserialization.
  {
    let js_string: JsValue = JsValue::from_str(expected_str);
    let de: IotaDID = js_string.into_serde().unwrap();
    assert_eq!(de, expected);
  }
}
