// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_iota::core::Timestamp;
use identity_iota::iota_core::IotaDID;
use identity_wasm::account::types::WasmKeyLocation;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use identity_wasm::common::WasmTimestamp;
use identity_wasm::crypto::WasmKeyPair;
use identity_wasm::crypto::WasmKeyType;
use identity_wasm::did::WasmDID;
use identity_wasm::did::WasmDIDUrl;
use identity_wasm::did::WasmDocument;
use identity_wasm::did::WasmMethodScope;
use identity_wasm::did::WasmVerificationMethod;
use identity_wasm::error::WasmError;

#[wasm_bindgen_test]
fn test_keypair() {
  let key1 = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let key2 = WasmKeyPair::from_keys(WasmKeyType::Ed25519, key1.public(), key1.private()).unwrap();

  let json1 = key1.to_json().unwrap();
  let json2 = key2.to_json().unwrap();

  let from1 = WasmKeyPair::from_json(&json1).unwrap();
  let from2 = WasmKeyPair::from_json(&json2).unwrap();

  assert_eq!(from1.public(), key1.public());
  assert_eq!(from1.private(), key1.private());

  assert_eq!(from2.public(), key2.public());
  assert_eq!(from2.private(), key2.private());
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
  let key = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let did = WasmDID::new(&key.public(), None).unwrap();

  assert_eq!(did.network_name(), "main");

  let parsed = WasmDID::parse(&did.to_string()).unwrap();

  assert_eq!(did.to_string(), parsed.to_string());

  let base58 = WasmDID::new(&key.public(), Some("dev".to_owned())).unwrap();

  assert_eq!(base58.tag(), did.tag());
  assert_eq!(base58.network_name(), "dev");
}

#[wasm_bindgen_test]
fn test_did_url() {
  // Base DID Url
  let key = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let did = WasmDID::new(&key.public(), None).unwrap();
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
  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
  assert_eq!(document.id().network_name(), "main");
  assert!(document.default_signing_method().is_ok());
}

#[wasm_bindgen_test]
fn test_document_sign_self() {
  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();

  // Sign with DIDUrl method query.
  {
    let mut document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
    document
      .sign_self(
        &keypair,
        &JsValue::from(document.default_signing_method().unwrap().id()).unchecked_into(),
      )
      .unwrap();
    assert!(document.verify_document(&document).is_ok());
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
    assert!(document.verify_document(&document).is_ok());
  }
}

#[wasm_bindgen_test]
fn test_document_resolve_method() {
  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let mut document: WasmDocument = WasmDocument::new(&keypair, None, None).unwrap();
  let default_method: WasmVerificationMethod = document.default_signing_method().unwrap();

  let keypair_new: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let method_new: WasmVerificationMethod = WasmVerificationMethod::new(
    &document.id(),
    WasmKeyType::Ed25519,
    keypair_new.public(),
    "new-key".to_owned(),
  )
  .unwrap();
  document
    .insert_method(&method_new, &WasmMethodScope::authentication())
    .unwrap();

  // Resolve with DIDUrl method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from(default_method.id()).unchecked_into(), None)
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(&JsValue::from(method_new.id()).unchecked_into(), None)
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method_new.id().to_string()
  );

  // Resolve with string method query.
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from_str(&default_method.id().to_string()).unchecked_into(),
        None,
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&method_new.id().to_string()).unchecked_into(), None)
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method_new.id().to_string()
  );

  // Resolve with string fragment method query.
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from_str(&default_method.id().fragment().unwrap()).unchecked_into(),
        None,
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from_str(&method_new.id().fragment().unwrap()).unchecked_into(),
        None,
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method_new.id().to_string()
  );

  // Resolve with correct verification method relationship.
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from(default_method.id()).unchecked_into(),
        Some(JsValue::from(WasmMethodScope::capability_invocation()).unchecked_into()),
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    default_method.id().to_string()
  );
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from(method_new.id()).unchecked_into(),
        Some(JsValue::from(WasmMethodScope::authentication()).unchecked_into()),
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method_new.id().to_string()
  );

  // Resolve with wrong verification method relationship.
  assert!(document
    .resolve_method(
      &JsValue::from(default_method.id()).unchecked_into(),
      Some(JsValue::from(WasmMethodScope::key_agreement()).unchecked_into()),
    )
    .unwrap()
    .is_none());
  assert!(document
    .resolve_method(
      &JsValue::from(method_new.id()).unchecked_into(),
      Some(JsValue::from(WasmMethodScope::assertion_method()).unchecked_into()),
    )
    .unwrap()
    .is_none());
}

#[wasm_bindgen_test]
fn test_document_network() {
  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
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

#[wasm_bindgen_test]
fn test_timestamp_serde() {
  let expected_str: &str = "2022-12-25T13:58:03Z";
  let timestamp: Timestamp = Timestamp::parse(expected_str).unwrap();
  let wasm_timestamp: WasmTimestamp = WasmTimestamp::from(timestamp);

  let ser: String = wasm_timestamp.to_json().unwrap().into_serde().unwrap();
  assert_eq!(ser, expected_str);
  let de: Timestamp = JsValue::from_str(&ser).into_serde().unwrap();
  assert_eq!(de, timestamp);
  let wasm_de: WasmTimestamp = WasmTimestamp::from_json(&JsValue::from_str(&ser)).unwrap();
  assert_eq!(wasm_de.to_rfc3339(), expected_str);
}

#[wasm_bindgen_test]
fn test_sign_document() {
  let keypair1: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let document1: WasmDocument = WasmDocument::new(&keypair1, None, None).unwrap();

  // Replace the default signing method.
  let mut document2: WasmDocument = document1.clone();
  let keypair2: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let method: WasmVerificationMethod = WasmVerificationMethod::new(
    &document2.id(),
    keypair2.type_(),
    keypair2.public(),
    "#method-2".to_owned(),
  )
  .unwrap();
  document2
    .insert_method(&method, &WasmMethodScope::capability_invocation())
    .unwrap();
  document2
    .remove_method(&document1.default_signing_method().unwrap().id())
    .unwrap();

  // Sign update using original document.
  assert!(document1.verify_document(&document2).is_err());
  document1
    .sign_document(
      &mut document2,
      &keypair1,
      &JsValue::from(document1.default_signing_method().unwrap().id()).unchecked_into(),
    )
    .unwrap();
  document1.verify_document(&document2).unwrap();
}

// This test should be matched by a test with equivalent test vector in Rust
// to ensure hashes are consistent across architectures.
#[wasm_bindgen_test]
fn test_hash_is_consistent() {
  let test_vector_1: [u8; 32] = [
    187, 104, 26, 87, 133, 152, 0, 180, 17, 232, 218, 46, 190, 140, 102, 34, 42, 94, 9, 101, 87, 249, 167, 237, 194,
    182, 240, 2, 150, 78, 110, 218,
  ];

  let test_vector_2: [u8; 32] = [
    125, 153, 99, 21, 23, 190, 149, 109, 84, 120, 40, 91, 181, 57, 67, 254, 11, 25, 152, 214, 84, 46, 105, 186, 16, 39,
    141, 151, 100, 163, 138, 222,
  ];

  let location_1 = WasmKeyLocation::new(WasmKeyType::Ed25519, "".to_owned(), test_vector_1.to_vec());
  let location_2 = WasmKeyLocation::new(WasmKeyType::Ed25519, "".to_owned(), test_vector_2.to_vec());

  assert_eq!(location_1.to_string().split(':').last().unwrap(), "74874706796298672");
  assert_eq!(
    location_2.to_string().split(':').last().unwrap(),
    "10201576743536852223"
  );
}
