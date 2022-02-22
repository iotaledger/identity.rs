// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity::account::ChainState;
use identity::account::EncryptionKey;
use identity::account::Error as AccountError;
use identity::account::Generation;
use identity::account::IdentityState;
use identity::account::KeyLocation;
use identity::account::Result as AccountResult;
use identity::account::Signature;
use identity::account::Storage;
use identity::core::encode_b58;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::iota::IotaDID;
use js_sys::Object;
use js_sys::Promise;
use js_sys::Reflect;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::account::identity::WasmChainState;
use crate::account::identity::WasmIdentityState;
use crate::account::types::WasmGeneration;
use crate::account::types::WasmKeyLocation;
use crate::did::WasmDID;
use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseUnit;
  #[wasm_bindgen(typescript_type = "Promise<DIDLease>")]
  pub type PromiseDIDLease;
  #[wasm_bindgen(typescript_type = "Promise<string>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<Signature>")]
  pub type PromiseSignature;
  #[wasm_bindgen(typescript_type = "Promise<boolean>")]
  pub type PromiseBool;
  #[wasm_bindgen(typescript_type = "Promise<Generation>")]
  pub type PromiseOptionGeneration;
  #[wasm_bindgen(typescript_type = "Promise<ChainState>")]
  pub type PromiseOptionChainState;
  #[wasm_bindgen(typescript_type = "Promise<IdentityState>")]
  pub type PromiseOptionIdentityState;
}

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;

  #[wasm_bindgen(method)]
  pub fn set_password(this: &WasmStorage, password: Vec<u8>) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn flush_changes(this: &WasmStorage) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn lease_did(this: &WasmStorage, did: WasmDID) -> PromiseDIDLease;
  #[wasm_bindgen(method)]
  pub fn key_new(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method)]
  pub fn key_insert(
    this: &WasmStorage,
    did: WasmDID,
    location: WasmKeyLocation,
    private_key: String,
  ) -> PromisePublicKey;
  #[wasm_bindgen(method)]
  pub fn key_get(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method)]
  pub fn key_del(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn key_sign(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation, data: Vec<u8>) -> PromiseSignature;
  #[wasm_bindgen(method)]
  pub fn key_exists(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method)]
  pub fn published_generation(this: &WasmStorage, did: WasmDID) -> PromiseOptionGeneration;
  #[wasm_bindgen(method)]
  pub fn set_published_generation(this: &WasmStorage, did: WasmDID, index: WasmGeneration) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn chain_state(this: &WasmStorage, did: WasmDID) -> PromiseOptionChainState;
  #[wasm_bindgen(method)]
  pub fn set_chain_state(this: &WasmStorage, did: WasmDID, chain_state: WasmChainState) -> PromiseUnit;
  #[wasm_bindgen(method)]
  pub fn state(this: &WasmStorage, did: WasmDID) -> PromiseOptionIdentityState;
  #[wasm_bindgen(method)]
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
  /// Sets the account password.
  async fn set_password(&self, password: EncryptionKey) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.set_password(password.to_vec()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Write any unsaved changes to disk.
  async fn flush_changes(&self) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.flush_changes());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Creates a new keypair at the specified `location`, and returns its `PublicKey`.
  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> AccountResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_new(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Inserts a private key at the specified `location`, and returns its `PublicKey`.
  async fn key_insert(
    &self,
    did: &IotaDID,
    location: &KeyLocation,
    private_key: PrivateKey,
  ) -> AccountResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_insert(
      did.clone().into(),
      location.clone().into(),
      encode_b58(private_key.as_ref()),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> AccountResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_get(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.key_del(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> AccountResult<Signature> {
    let promise: Promise = Promise::resolve(&self.key_sign(did.clone().into(), location.clone().into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> AccountResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_exists(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the last generation that has been published to the tangle for the given `did`.
  async fn published_generation(&self, did: &IotaDID) -> AccountResult<Option<Generation>> {
    let promise: Promise = Promise::resolve(&self.published_generation(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Sets the last generation that has been published to the tangle for the given `did`.
  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.set_published_generation(did.clone().into(), index.into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, did: &IotaDID) -> AccountResult<Option<ChainState>> {
    let promise: Promise = Promise::resolve(&self.chain_state(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Set the chain state of the identity specified by `did`.
  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.set_chain_state(did.clone().into(), chain_state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Returns the state of the identity specified by `did`.
  async fn state(&self, did: &IotaDID) -> AccountResult<Option<IdentityState>> {
    let promise: Promise = Promise::resolve(&self.state(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Sets a new state for the identity specified by `did`.
  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.set_state(did.clone().into(), state.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> AccountResult<()> {
    let promise: Promise = Promise::resolve(&self.purge(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }
}

#[wasm_bindgen(typescript_custom_section)]
const STORAGE: &'static str = r#"
/** All methods an object must implement to be used as an account storage. */
interface Storage {
  /** Sets the account password.*/
  setPassword: (encryptionKey: Uint8Array) => Promise<void>;

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
  chainState: (did: DID) => Promise<ChainState>;

  /** Set the chain state of the identity specified by `did`.*/
  setChainState: (did: DID, chainState: ChainState) => Promise<void>;

  /** Returns the state of the identity specified by `did`.*/
  state: (did: DID) => Promise<IdentityState>;

  /** Sets a new state for the identity specified by `did`.*/
  setState: (did: DID, identityState: IdentityState) => Promise<void>;

  /** Removes the keys and any state for the identity specified by `did`.*/
  purge: (did: DID) => Promise<void>;

  /** Returns the last generation that has been published to the tangle for the given `did`.*/
  publishedGeneration: (did: DID) => Promise<Generation>;

  /** Sets the last generation that has been published to the tangle for the given `did`.*/
  setPublishedGeneration: (did: DID, generation: Generation) => Promise<void>;
}"#;

// from https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-656293288
#[allow(dead_code)]
pub fn downcast_js_value<T: FromWasmAbi<Abi = u32>>(js_value: JsValue, classname: &str) -> Result<T, AccountError> {
  let constructor_name: js_sys::JsString = Object::get_prototype_of(&js_value).constructor().name();
  if constructor_name == classname {
    let ptr: JsValue = Reflect::get(&js_value, &JsValue::from_str("ptr"))
      .map_err(|_| AccountError::JsError("unable to retrieve `ptr` property".to_string()))?;
    let ptr_u32: u32 = ptr
      .as_f64()
      .ok_or_else(|| AccountError::JsError("unable to read `ptr` property of `JsValue` as f64".to_string()))?
      as u32;
    unsafe { Ok(T::from_abi(ptr_u32)) }
  } else {
    Err(AccountError::SerializationError(format!(
      "Expected: {} - Found: {}",
      classname, constructor_name
    )))
  }
}
