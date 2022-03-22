// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity::account_storage::ChainState;
use identity::account_storage::Error as AccountStorageError;
use identity::account_storage::IdentityState;
use identity::account_storage::KeyLocation;
use identity::account_storage::Result as AccountStorageResult;
use identity::account_storage::Signature;
use identity::account_storage::Storage;
use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::iota_core::IotaDID;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::account::identity::WasmChainState;
use crate::account::identity::WasmIdentityState;
use crate::account::types::WasmKeyLocation;
use crate::did::WasmDID;
use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseUnit;
  #[wasm_bindgen(typescript_type = "Promise<string>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<Signature>")]
  pub type PromiseSignature;
  #[wasm_bindgen(typescript_type = "Promise<boolean>")]
  pub type PromiseBool;
  #[wasm_bindgen(typescript_type = "Promise<ChainState | undefined | null>")]
  pub type PromiseOptionChainState;
  #[wasm_bindgen(typescript_type = "Promise<IdentityState | undefined | null>")]
  pub type PromiseOptionIdentityState;
}

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;

  #[wasm_bindgen(method, js_name = flushChanges)]
  pub fn flush_changes(this: &WasmStorage) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keyNew)]
  pub fn key_new(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyInsert)]
  pub fn key_insert(
    this: &WasmStorage,
    did: WasmDID,
    location: WasmKeyLocation,
    private_key: String,
  ) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyGet)]
  pub fn key_get(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyDel)]
  pub fn key_del(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseUnit;
  #[wasm_bindgen(method, js_name = keySign)]
  pub fn key_sign(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation, data: Vec<u8>) -> PromiseSignature;
  #[wasm_bindgen(method, js_name = keyExists)]
  pub fn key_exists(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method, js_name = chainState)]
  pub fn chain_state(this: &WasmStorage, did: WasmDID) -> PromiseOptionChainState;
  #[wasm_bindgen(method, js_name = setChainState)]
  pub fn set_chain_state(this: &WasmStorage, did: WasmDID, chain_state: WasmChainState) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn state(this: &WasmStorage, did: WasmDID) -> PromiseOptionIdentityState;
  #[wasm_bindgen(method, js_name = setState)]
  pub fn set_state(this: &WasmStorage, did: WasmDID, state: WasmIdentityState) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn purge(this: &WasmStorage, did: WasmDID) -> PromiseUnit;
}

impl Debug for WasmStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmStorage")
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for WasmStorage {
  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.flush_changes());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Creates a new keypair at the specified `location`, and returns its `PublicKey`.
  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_new(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: String = result
      .account_err()?
      .as_string()
      .ok_or_else(|| AccountStorageError::SerializationError("Expected string".to_string()))?;
    let public_key: PublicKey = decode_b58(&public_key)?.into();
    Ok(public_key)
  }

  /// Inserts a private key at the specified `location`, and returns its `PublicKey`.
  async fn key_insert(
    &self,
    did: &IotaDID,
    location: &KeyLocation,
    private_key: PrivateKey,
  ) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_insert(
      did.clone().into(),
      location.clone().into(),
      encode_b58(private_key.as_ref()),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: String = result
      .account_err()?
      .as_string()
      .ok_or_else(|| AccountStorageError::SerializationError("Expected string".to_string()))?;
    let public_key: PublicKey = decode_b58(&public_key)?.into();
    Ok(public_key)
  }

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_get(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: String = result
      .account_err()?
      .as_string()
      .ok_or_else(|| AccountStorageError::SerializationError("Expected string".to_string()))?;
    let public_key: PublicKey = decode_b58(&public_key)?.into();
    Ok(public_key)
  }

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.key_del(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> AccountStorageResult<Signature> {
    let promise: Promise = Promise::resolve(&self.key_sign(did.clone().into(), location.clone().into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    let signature: Signature = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(signature)
  }

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_exists(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, did: &IotaDID) -> AccountStorageResult<Option<ChainState>> {
    let promise: Promise = Promise::resolve(&self.chain_state(did.clone().into()));
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
  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.set_chain_state(did.clone().into(), chain_state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the state of the identity specified by `did`.
  async fn state(&self, did: &IotaDID) -> AccountStorageResult<Option<IdentityState>> {
    let promise: Promise = Promise::resolve(&self.state(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.account_err()?;
    if js_value.is_null() || js_value.is_undefined() {
      return Ok(None);
    }
    let state: IdentityState = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(Some(state))
  }

  /// Sets a new state for the identity specified by `did`.
  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.set_state(did.clone().into(), state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.purge(did.clone().into()));
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
   * and returns its public key as a base58 string.*/
  keyNew: (did: DID, keyLocation: KeyLocation) => Promise<string>;

  /** Inserts a private key, encoded as a base58 string, at the specified `location`,
   * and returns its public key as a base58 string.*/
  keyInsert: (did: DID, keyLocation: KeyLocation, privateKey: string) => Promise<string>;

  /** Returns `true` if a keypair exists at the specified `location`.*/
  keyExists: (did: DID, keyLocation: KeyLocation) => Promise<boolean>;

  /** Retrieves the public key, encoded as a base58 string, from the specified `location`.*/
  keyGet: (did: DID, keyLocation: KeyLocation) => Promise<string>;

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
