// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_iota::core::Timestamp;
use identity_iota::iota_core::IotaDID;
use identity_wasm::crypto::WasmProofOptions;
use identity_wasm::did::WasmVerifierOptions;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use identity_wasm::common::WasmTimestamp;
use identity_wasm::crypto::WasmKeyPair;
use identity_wasm::crypto::WasmKeyType;
use identity_wasm::did::WasmMethodScope;
use identity_wasm::error::WasmError;
use identity_wasm::iota::WasmIotaDID;
use identity_wasm::iota::WasmIotaDIDUrl;
use identity_wasm::iota::WasmIotaDocument;
use identity_wasm::iota::WasmIotaVerificationMethod;

/// Generate a random byte array of the length of an Alias Id.
fn random_alias_id() -> [u8; 32] {
  rand::random()
}

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
  let alias_bytes = random_alias_id();
  let did = WasmIotaDID::new(&alias_bytes, IotaDID::DEFAULT_NETWORK.to_owned()).unwrap();

  assert_eq!(did.network_str(), WasmIotaDID::static_default_network());

  let parsed = WasmIotaDID::parse(&did.to_string()).unwrap();

  assert_eq!(did.to_string(), parsed.to_string());

  let smr_did = WasmIotaDID::new(&alias_bytes, "smr".to_owned()).unwrap();

  assert_eq!(smr_did.tag(), did.tag());
  assert_eq!(smr_did.network_str(), "smr");
}

#[wasm_bindgen_test]
fn test_did_methods() {
  let tag = "0xdfda8bcfb959c3e6ef261343c3e1a8310e9c8294eeafee326a4e96d65dbeaca0";
  let did_str = format!("did:iota:smr:{tag}");
  let did = WasmIotaDID::parse(&did_str).unwrap();

  assert_eq!(did.to_string(), did_str);
  assert_eq!(did.tag(), tag);
  assert_eq!(did.network_str(), "smr");
  assert_eq!(did.scheme(), "did");
  assert_eq!(did.method(), "iota");
  assert_eq!(did.method_id(), format!("smr:{tag}"));
  assert_eq!(did.authority(), format!("iota:smr:{tag}"));
}

#[wasm_bindgen_test]
fn test_did_url() {
  // Base DID Url
  let alias_bytes = random_alias_id();
  let did = WasmIotaDID::new(&alias_bytes, WasmIotaDID::static_default_network()).unwrap();
  let did_url = did.to_url();

  assert_eq!(did.to_string(), did_url.to_string());

  let parsed_from_did = WasmIotaDIDUrl::parse(&did.to_string()).unwrap();
  let parsed_from_did_url = WasmIotaDIDUrl::parse(&did_url.to_string()).unwrap();

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
  let document: WasmIotaDocument = WasmIotaDocument::new("atoi".to_owned()).unwrap();
  assert_eq!(
    document.id().tag(),
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );
  assert_eq!(document.id().network_str(), "atoi");
  assert!(document.metadata_created().is_some());
  assert!(document.metadata_updated().is_some());
}

#[wasm_bindgen_test]
fn test_document_sign() {
  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();

  // Sign with DIDUrl method query.
  {
    let mut document: WasmIotaDocument = WasmIotaDocument::new(WasmIotaDID::static_default_network()).unwrap();
    let method = WasmIotaVerificationMethod::new(
      &document.id(),
      WasmKeyType::Ed25519,
      keypair.public(),
      "#sign-0".to_owned(),
    )
    .unwrap();
    document
      .insert_method(&method, &WasmMethodScope::authentication())
      .unwrap();

    let data = JsValue::from_serde(&json!({
      "answer": 42,
      "nested": {
        "property": "string",
      }
    }))
    .unwrap();

    // Sign with DIDUrl.
    let signed = document
      .sign_data(
        &data,
        keypair.private(),
        &JsValue::from(method.id()).unchecked_into(),
        &WasmProofOptions::default(),
      )
      .unwrap();

    // Sign with string method query.
    let signed_str = document
      .sign_data(
        &data,
        keypair.private(),
        &JsValue::from_str("#sign-0").unchecked_into(),
        &WasmProofOptions::default(),
      )
      .unwrap();

    assert!(document.verify_data(&signed, &WasmVerifierOptions::default()).unwrap());
    assert_eq!(
      signed.into_serde::<serde_json::Value>().unwrap(),
      signed_str.into_serde::<serde_json::Value>().unwrap()
    );
  }
}

#[wasm_bindgen_test]
fn test_document_resolve_method() {
  let mut document: WasmIotaDocument = WasmIotaDocument::new(WasmIotaDID::static_default_network()).unwrap();

  let keypair: WasmKeyPair = WasmKeyPair::new(WasmKeyType::Ed25519).unwrap();
  let method: WasmIotaVerificationMethod = WasmIotaVerificationMethod::new(
    &document.id(),
    WasmKeyType::Ed25519,
    keypair.public(),
    "new-key".to_owned(),
  )
  .unwrap();
  document
    .insert_method(&method, &WasmMethodScope::authentication())
    .unwrap();

  // Resolve with DIDUrl method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from(method.id()).unchecked_into(), None)
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method.id().to_string()
  );

  // Resolve with string method query.
  assert_eq!(
    document
      .resolve_method(&JsValue::from_str(&method.id().to_string()).unchecked_into(), None)
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method.id().to_string()
  );

  // Resolve with string fragment method query.
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from_str(&method.id().fragment().unwrap()).unchecked_into(),
        None,
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method.id().to_string()
  );

  // Resolve with correct verification method relationship.
  assert_eq!(
    document
      .resolve_method(
        &JsValue::from(method.id()).unchecked_into(),
        Some(JsValue::from(WasmMethodScope::authentication()).unchecked_into()),
      )
      .unwrap()
      .unwrap()
      .id()
      .to_string(),
    method.id().to_string()
  );

  // Resolve with wrong verification method relationship.
  assert!(document
    .resolve_method(
      &JsValue::from(method.id()).unchecked_into(),
      Some(JsValue::from(WasmMethodScope::assertion_method()).unchecked_into()),
    )
    .unwrap()
    .is_none());
}

#[wasm_bindgen_test]
fn test_did_serde() {
  // Ensure JSON deserialization of DID and strings match (for UWasmDID duck-typed parameters).
  let expected_str: &str = "did:iota:0xdfda8bcfb959c3e6ef261343c3e1a8310e9c8294eeafee326a4e96d65dbeaca0";
  let expected: IotaDID = IotaDID::parse(expected_str).unwrap();

  // Check WasmDID deserialization.
  {
    let wasm_did: WasmIotaDID = WasmIotaDID::from(expected.clone());
    let de: IotaDID = wasm_did.to_json().unwrap().into_serde().unwrap();
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

// TODO: Remove or reuse depending on what we do with the account. depending on what we do with the account.
// This test should be matched by a test with equivalent test vector in Rust
// to ensure hashes are consistent across architectures.
// #[wasm_bindgen_test]
// fn test_hash_is_consistent() {
//   let test_vector_1: [u8; 32] = [
//     187, 104, 26, 87, 133, 152, 0, 180, 17, 232, 218, 46, 190, 140, 102, 34, 42, 94, 9, 101, 87, 249, 167, 237, 194,
//     182, 240, 2, 150, 78, 110, 218,
//   ];

//   let test_vector_2: [u8; 32] = [
//     125, 153, 99, 21, 23, 190, 149, 109, 84, 120, 40, 91, 181, 57, 67, 254, 11, 25, 152, 214, 84, 46, 105, 186, 16,
// 39,     141, 151, 100, 163, 138, 222,
//   ];

//   let location_1 = WasmKeyLocation::new(WasmKeyType::Ed25519, "".to_owned(), test_vector_1.to_vec());
//   let location_2 = WasmKeyLocation::new(WasmKeyType::Ed25519, "".to_owned(), test_vector_2.to_vec());

//   assert_eq!(location_1.to_string().split(':').last().unwrap(), "74874706796298672");
//   assert_eq!(
//     location_2.to_string().split(':').last().unwrap(),
//     "10201576743536852223"
//   );
// }
