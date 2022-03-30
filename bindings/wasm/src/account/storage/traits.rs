// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity::account_storage::ChainState;
use identity::account_storage::Error as AccountStorageError;
use identity::account_storage::KeyLocation;
use identity::account_storage::Result as AccountStorageResult;
use identity::account_storage::Signature;
use identity::account_storage::Storage;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::iota_core::IotaDID;
use identity::iota_core::NetworkName;
use identity::prelude::IotaDocument;
use identity::prelude::KeyType;
use js_sys::Array;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::account::identity::WasmChainState;
use crate::account::types::WasmKeyLocation;
use crate::crypto::WasmKeyType;
use crate::did::WasmDID;
use crate::did::WasmDocument;
use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseUnit;
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<Signature>")]
  pub type PromiseSignature;
  #[wasm_bindgen(typescript_type = "Promise<boolean>")]
  pub type PromiseBool;
  #[wasm_bindgen(typescript_type = "Promise<ChainState | undefined | null>")]
  pub type PromiseOptionChainState;
  #[wasm_bindgen(typescript_type = "Promise<Document | undefined | null>")]
  pub type PromiseOptionDocument;
  #[wasm_bindgen(typescript_type = "Promise<KeyLocation>")]
  pub type PromiseKeyLocation;
  #[wasm_bindgen(typescript_type = "Promise<Array<DID>>")]
  pub type PromiseArrayDID;
  #[wasm_bindgen(typescript_type = "Promise<[DID, KeyLocation]>")]
  pub type PromiseDIDKeyLocation;
}

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;

  #[wasm_bindgen(method, js_name = didCreate)]
  pub fn did_create(
    this: &WasmStorage,
    network: &str,
    fragment: &str,
    private_key: Option<Vec<u8>>,
  ) -> PromiseDIDKeyLocation;
  #[wasm_bindgen(method, js_name = didPurge)]
  pub fn did_purge(this: &WasmStorage, did: WasmDID) -> PromiseUnit;

  #[wasm_bindgen(method, js_name = didExists)]
  pub fn did_exists(this: &WasmStorage, did: WasmDID) -> PromiseBool;
  #[wasm_bindgen(method, js_name = didList)]
  pub fn did_list(this: &WasmStorage) -> PromiseArrayDID;

  #[wasm_bindgen(method, js_name = keyGenerate)]
  pub fn key_generate(this: &WasmStorage, did: WasmDID, key_type: WasmKeyType, fragment: String) -> PromiseKeyLocation;
  #[wasm_bindgen(method, js_name = keyInsert)]
  pub fn key_insert(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation, private_key: Vec<u8>) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keyPublic)]
  pub fn key_public(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyDelete)]
  pub fn key_delete(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keySign)]
  pub fn key_sign(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation, data: Vec<u8>) -> PromiseSignature;
  #[wasm_bindgen(method, js_name = keyExists)]
  pub fn key_exists(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method, js_name = chainStateGet)]
  pub fn chain_state_get(this: &WasmStorage, did: WasmDID) -> PromiseOptionChainState;
  #[wasm_bindgen(method, js_name = chainStateSet)]
  pub fn chain_state_set(this: &WasmStorage, did: WasmDID, chain_state: WasmChainState) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = documentGet)]
  pub fn document_get(this: &WasmStorage, did: WasmDID) -> PromiseOptionDocument;
  #[wasm_bindgen(method, js_name = documentSet)]
  pub fn document_set(this: &WasmStorage, did: WasmDID, document: WasmDocument) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = flushChanges)]
  pub fn flush_changes(this: &WasmStorage) -> PromiseUnit;
}

impl Debug for WasmStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmStorage")
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for WasmStorage {
  async fn did_create(
    &self,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> AccountStorageResult<(IotaDID, KeyLocation)> {
    let private_key: Option<Vec<u8>> = private_key.map(|key| {
      let key_bytes: Vec<u8> = key.as_ref().to_vec();
      core::mem::drop(key);
      key_bytes
    });

    let promise: Promise = Promise::resolve(&self.did_create(network.as_ref(), fragment, private_key));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let did_location_tuple: js_sys::Array = js_sys::Array::from(&result.account_err()?);
    let mut did_location_tuple: js_sys::ArrayIter = did_location_tuple.iter();

    let did: IotaDID = did_location_tuple
      .next()
      .ok_or_else(|| AccountStorageError::JsError("expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;

    let location: KeyLocation = did_location_tuple
      .next()
      .ok_or_else(|| AccountStorageError::JsError("expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;

    Ok((did, location))
  }

  async fn did_purge(&self, did: &IotaDID) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.did_purge(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn did_exists(&self, did: &IotaDID) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.did_exists(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn did_list(&self) -> AccountStorageResult<Vec<IotaDID>> {
    let promise: Promise = Promise::resolve(&self.did_list());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;

    js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))
  }

  async fn key_generate(&self, did: &IotaDID, key_type: KeyType, fragment: &str) -> AccountStorageResult<KeyLocation> {
    let promise: Promise =
      Promise::resolve(&self.key_generate(did.clone().into(), key_type.into(), fragment.to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let location: KeyLocation = result
      .account_err()?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;

    Ok(location)
  }

  async fn key_insert(
    &self,
    did: &IotaDID,
    location: &KeyLocation,
    private_key: PrivateKey,
  ) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.key_insert(
      did.clone().into(),
      location.clone().into(),
      private_key.as_ref().to_vec(),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn key_public(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_public(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: Vec<u8> = result.account_err().map(uint8array_to_bytes)??;
    Ok(public_key.into())
  }

  async fn key_delete(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_delete(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> AccountStorageResult<Signature> {
    let promise: Promise = Promise::resolve(&self.key_sign(did.clone().into(), location.clone().into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    let signature: Signature = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(signature)
  }

  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_exists(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn chain_state_get(&self, did: &IotaDID) -> AccountStorageResult<Option<ChainState>> {
    let promise: Promise = Promise::resolve(&self.chain_state_get(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    if js_value.is_null() || js_value.is_undefined() {
      return Ok(None);
    }
    let chain_state: ChainState = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(Some(chain_state))
  }

  async fn chain_state_set(&self, did: &IotaDID, chain_state: &ChainState) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.chain_state_set(did.clone().into(), chain_state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn document_get(&self, did: &IotaDID) -> AccountStorageResult<Option<IotaDocument>> {
    let promise: Promise = Promise::resolve(&self.document_get(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    if js_value.is_null() || js_value.is_undefined() {
      return Ok(None);
    }
    let document: IotaDocument = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(Some(document))
  }

  async fn document_set(&self, did: &IotaDID, document: &IotaDocument) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.document_set(did.clone().into(), document.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn flush_changes(&self) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.flush_changes());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }
}

#[wasm_bindgen(typescript_custom_section)]
const STORAGE: &'static str = r#"
/** All methods an object must implement to be used as an account storage. */
interface Storage {
  didCreate: (network: string, fragment: string, private_key: Uint8Array | undefined | null) => Promise<[DID, KeyLocation]>;

  didPurge: (did: DID) => Promise<boolean>;

  didExists: (did: DID) => Promise<boolean>;

  didList: () => Promise<Array<DID>>;

  keyGenerate: (did: DID, keyType: KeyType, fragment: string) => Promise<KeyLocation>;

  keyInsert: (did: DID, keyLocation: KeyLocation, privateKey: Uint8Array) => Promise<void>;

  keyPublic: (did: DID, keyLocation: KeyLocation) => Promise<Uint8Array>;

  keyDelete: (did: DID, keyLocation: KeyLocation) => Promise<boolean>;

  keySign: (did: DID, keyLocation: KeyLocation, data: Uint8Array) => Promise<Signature>;

  keyExists: (did: DID, keyLocation: KeyLocation) => Promise<boolean>;

  chainStateGet: (did: DID) => Promise<ChainState | undefined | null>;

  chainStateSet: (did: DID, chainState: ChainState) => Promise<void>;

  documentGet: (did: DID) => Promise<Document | undefined | null>;

  documentSet: (did: DID, document: Document) => Promise<void>;

  flushChanges: () => Promise<void>;
}"#;

fn uint8array_to_bytes(value: JsValue) -> AccountStorageResult<Vec<u8>> {
  if !JsCast::is_instance_of::<Uint8Array>(&value) {
    return Err(AccountStorageError::SerializationError(
      "expected Uint8Array".to_owned(),
    ));
  }
  let array_js_value = JsValue::from(Array::from(&value));
  array_js_value
    .into_serde()
    .map_err(|e| AccountStorageError::SerializationError(e.to_string()))
}
