// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use std::rc::Rc;

// use identity_iota::account_storage::CekAlgorithm;
// use identity_iota::account_storage::DIDType;
// use identity_iota::account_storage::EncryptedData;
// use identity_iota::account_storage::EncryptionAlgorithm;
// use identity_iota::account_storage::Error as AccountStorageError;
// use identity_iota::account_storage::KeyLocation;
// use identity_iota::account_storage::Result as AccountStorageResult;
// use identity_iota::account_storage::Signature;
// use identity_iota::account_storage::Storage;
// use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::PublicKey;
// use identity_iota::did::CoreDID;
// use identity_iota::iota::NetworkName;
// use identity_iota::prelude::KeyType;
use identity_storage::KeyAlias;
use identity_storage::KeyStorage;
use identity_storage::Signature;
use identity_storage::StorageResult;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// use crate::account::types::WasmCekAlgorithm;
// use crate::account::types::WasmDIDType;
// use crate::account::types::WasmEncryptedData;
// use crate::account::types::WasmEncryptionAlgorithm;
// use crate::account::types::WasmKeyLocation;
// use crate::common::PromiseBool;
// use crate::common::PromiseVoid;
// use crate::crypto::WasmKeyType;
// use crate::did::WasmCoreDID;
use crate::error::JsValueResult;
use crate::key_alias::WasmKeyAlias;
use crate::util::uint8array_to_bytes;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<KeyAlias>")]
  pub type PromiseKeyAlias;

  // TODO: Change to signature type.
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromiseSignature;

  #[wasm_bindgen(typescript_type = "Promise<string>")]
  pub type PromiseString;

  // #[wasm_bindgen(typescript_type = "Promise<Array<CoreDID>>")]
  // pub type PromiseArrayDID;
  // #[wasm_bindgen(typescript_type = "Promise<[CoreDID, KeyLocation]>")]
  // pub type PromiseDIDKeyLocation;
  // #[wasm_bindgen(typescript_type = "Promise<EncryptedData>")]
  // pub type PromiseEncryptedData;
  // #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  // pub type PromiseData;
  // #[wasm_bindgen(typescript_type = "Promise<Uint8Array | undefined>")]
  // pub type PromiseOptionBytes;
}

pub type WasmNewKeyType = String;
pub type WasmSigningAlgorithm = String;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "KeyStorage")]
  pub type WasmKeyStorage;

  #[wasm_bindgen(method, js_name = generate)]
  pub fn generate(this: &WasmKeyStorage, key_type: WasmNewKeyType) -> PromiseKeyAlias;

  #[wasm_bindgen(method, js_name = public)]
  pub fn public(this: &WasmKeyStorage, alias: WasmKeyAlias) -> PromisePublicKey;

  #[wasm_bindgen(method, js_name = sign)]
  pub fn sign(
    this: &WasmKeyStorage,
    privateKey: WasmKeyAlias,
    signing_algorithm: WasmSigningAlgorithm,
    data: Vec<u8>,
  ) -> PromiseSignature;
}

impl Debug for WasmKeyStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmKeyStorage")
  }
}

#[async_trait::async_trait(?Send)]
impl KeyStorage for WasmKeyStorage {
  type KeyType = WasmNewKeyType;
  type SigningAlgorithm = WasmSigningAlgorithm;

  async fn generate(&self, key_type: Self::KeyType) -> StorageResult<KeyAlias> {
    let promise: Promise = Promise::resolve(&self.generate(key_type));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let alias: KeyAlias = result.0.expect("TODO").into_serde().expect("TODO");
    Ok(alias)
  }

  async fn public(&self, private_key: &KeyAlias) -> StorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.public(private_key.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: Vec<u8> = uint8array_to_bytes(result.0.expect("TODO"))?;
    Ok(public_key.into())
  }

  async fn sign<ST: Send + Into<Self::SigningAlgorithm>>(
    &self,
    private_key: &KeyAlias,
    signing_algorithm: ST,
    data: Vec<u8>,
  ) -> StorageResult<Signature> {
    let promise: Promise = Promise::resolve(&self.sign(private_key.clone().into(), signing_algorithm.into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.0.expect("TODO");
    let signature: Signature = js_value.into_serde().expect("TODO");
    Ok(signature)
  }
}

#[wasm_bindgen(typescript_custom_section)]
const STORAGE: &'static str = r#"
interface KeyStorage {
  // TODO: This can be made more type safe.
  generate: (keyType: string) => Promise<KeyAlias>;
  public: (privateKey: KeyAlias) => Promise<Uint8Array>;
  sign: (privateKey: KeyAlias, signing_algorithm: string, data: Uint8Array) => Promise<Uint8Array>;
}"#;
