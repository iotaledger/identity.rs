// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity::account_storage::AccountId;
use identity::account_storage::ChainState;
use identity::account_storage::Error as AccountStorageError;
use identity::account_storage::KeyLocation;
use identity::account_storage::Result as AccountStorageResult;
use identity::account_storage::Signature;
use identity::account_storage::Storage;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::iota_core::IotaDID;
use identity::prelude::IotaDocument;
use identity::prelude::KeyType;
use js_sys::Array;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::account::identity::WasmChainState;
use crate::account::types::WasmAccountId;
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
  #[wasm_bindgen(typescript_type = "Promise<AccountId | undefined | null>")]
  pub type PromiseOptionAccountId;
  #[wasm_bindgen(typescript_type = "Promise<Array<WasmDID>>")]
  pub type PromiseArrayDID;
}

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;

  #[wasm_bindgen(method, js_name = keyGenerate)]
  pub fn key_generate(
    this: &WasmStorage,
    account_id: WasmAccountId,
    key_type: WasmKeyType,
    fragment: String,
  ) -> PromiseKeyLocation;
  #[wasm_bindgen(method, js_name = keyInsert)]
  pub fn key_insert(
    this: &WasmStorage,
    account_id: WasmAccountId,
    location: WasmKeyLocation,
    private_key: Vec<u8>,
  ) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keyPublic)]
  pub fn key_public(this: &WasmStorage, account_id: WasmAccountId, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyDelete)]
  pub fn key_delete(this: &WasmStorage, account_id: WasmAccountId, location: WasmKeyLocation) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keySign)]
  pub fn key_sign(
    this: &WasmStorage,
    account_id: WasmAccountId,
    location: WasmKeyLocation,
    data: Vec<u8>,
  ) -> PromiseSignature;
  #[wasm_bindgen(method, js_name = keyExists)]
  pub fn key_exists(this: &WasmStorage, account_id: WasmAccountId, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method, js_name = chainStateGet)]
  pub fn chain_state_get(this: &WasmStorage, account_id: WasmAccountId) -> PromiseOptionChainState;
  #[wasm_bindgen(method, js_name = chainStateSet)]
  pub fn chain_state_set(this: &WasmStorage, account_id: WasmAccountId, chain_state: WasmChainState) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = documentGet)]
  pub fn document_get(this: &WasmStorage, account_id: WasmAccountId) -> PromiseOptionDocument;
  #[wasm_bindgen(method, js_name = documentSet)]
  pub fn document_set(this: &WasmStorage, account_id: WasmAccountId, document: WasmDocument) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn purge(this: &WasmStorage, did: WasmDID) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = flushChanges)]
  pub fn flush_changes(this: &WasmStorage) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = indexSet)]
  pub fn index_set(this: &WasmStorage, did: WasmDID, account_id: WasmAccountId) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = indexGet)]
  pub fn index_get(this: &WasmStorage, did: WasmDID) -> PromiseOptionAccountId;
  #[wasm_bindgen(method, js_name = indexKeys)]
  pub fn index_keys(this: &WasmStorage) -> PromiseArrayDID;
}

impl Debug for WasmStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmStorage")
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for WasmStorage {
  /// Creates a new keypair at the specified `location`, and returns its `PublicKey`.
  async fn key_generate(
    &self,
    account_id: &AccountId,
    key_type: KeyType,
    fragment: &str,
  ) -> AccountStorageResult<KeyLocation> {
    let promise: Promise =
      Promise::resolve(&self.key_generate((*account_id).into(), key_type.into(), fragment.to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let location: KeyLocation = result
      .account_err()?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;

    Ok(location)
  }

  /// Inserts a private key at the specified `location`, and returns its `PublicKey`.
  async fn key_insert(
    &self,
    account_id: &AccountId,
    location: &KeyLocation,
    private_key: PrivateKey,
  ) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.key_insert(
      (*account_id).into(),
      location.clone().into(),
      private_key.as_ref().to_vec(),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Retrieves the public key at the specified `location`.
  async fn key_public(&self, account_id: &AccountId, location: &KeyLocation) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_public((*account_id).into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: Vec<u8> = result.account_err().map(uint8array_to_bytes)??;
    Ok(public_key.into())
  }

  /// Deletes the keypair specified by `location`.
  async fn key_delete(&self, account_id: &AccountId, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_delete((*account_id).into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(
    &self,
    account_id: &AccountId,
    location: &KeyLocation,
    data: Vec<u8>,
  ) -> AccountStorageResult<Signature> {
    let promise: Promise = Promise::resolve(&self.key_sign((*account_id).into(), location.clone().into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    let signature: Signature = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(signature)
  }

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, account_id: &AccountId, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_exists((*account_id).into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state_get(&self, account_id: &AccountId) -> AccountStorageResult<Option<ChainState>> {
    let promise: Promise = Promise::resolve(&self.chain_state_get((*account_id).into()));
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

  /// Set the chain state of the identity specified by `did`.
  async fn chain_state_set(&self, account_id: &AccountId, chain_state: &ChainState) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.chain_state_set((*account_id).into(), chain_state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the state of the identity specified by `did`.
  async fn document_get(&self, account_id: &AccountId) -> AccountStorageResult<Option<IotaDocument>> {
    let promise: Promise = Promise::resolve(&self.document_get((*account_id).into()));
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

  /// Sets a new state for the identity specified by `did`.
  async fn document_set(&self, account_id: &AccountId, document: &IotaDocument) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.document_set((*account_id).into(), document.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.purge(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn index_set(&self, did: IotaDID, account_id: AccountId) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.index_set(did.clone().into(), account_id.into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn index_get(&self, did: &IotaDID) -> AccountStorageResult<Option<AccountId>> {
    let promise: Promise = Promise::resolve(&self.index_get(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;

    if js_value.is_null() || js_value.is_undefined() {
      return Ok(None);
    }

    js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))
  }

  async fn index_keys(&self) -> AccountStorageResult<Vec<IotaDID>> {
    let promise: Promise = Promise::resolve(&self.index_keys());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;

    js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))
  }

  /// Write any unsaved changes to disk.
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
  /** Write any unsaved changes to disk.*/
  flushChanges: () => Promise<void>;

  /** Creates a new keypair at the specified `location`,
   * and returns its public key.*/
  keyNew: (did: DID, keyLocation: KeyLocation) => Promise<Uint8Array>;

  /** Inserts a private key at the specified `location`,
   * and returns its public key.*/
  keyInsert: (did: DID, keyLocation: KeyLocation, privateKey: Uint8Array) => Promise<Uint8Array>;

  /** Returns `true` if a keypair exists at the specified `location`.*/
  keyExists: (did: DID, keyLocation: KeyLocation) => Promise<boolean>;

  /** Retrieves the public key from the specified `location`.*/
  keyGet: (did: DID, keyLocation: KeyLocation) => Promise<Uint8Array>;

  /** Deletes the keypair specified by the given `location`. Nothing happens if the key is not found.*/
  keyDel: (did: DID, keyLocation: KeyLocation) => Promise<void>;

  /** Signs `data` with the private key at the specified `location`.*/
  keySign: (did: DID, keyLocation: KeyLocation, data: Uint8Array) => Promise<Signature>;

  /** Returns the chain state of the identity specified by `did`.*/
  chainState: (did: DID) => Promise<ChainState | undefined | null>;

  /** Set the chain state of the identity specified by `did`.*/
  setChainState: (did: DID, chainState: ChainState) => Promise<void>;

  /** Returns the state of the identity specified by `did`.*/
  state: (did: DID) => Promise<IdentityState | undefined | null>;

  /** Sets a new state for the identity specified by `did`.*/
  setState: (did: DID, identityState: IdentityState) => Promise<void>;

  /** Removes the keys and any state for the identity specified by `did`.*/
  purge: (did: DID) => Promise<void>;
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
