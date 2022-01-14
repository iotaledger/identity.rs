use core::fmt::Debug;
use core::fmt::Formatter;

use wasm_bindgen::prelude::*;

use identity::account::ChainState;
use identity::account::DIDLease;
use identity::account::EncryptionKey;
use identity::account::Generation;
use identity::account::IdentityState;
use identity::account::KeyLocation;
use identity::account::Result as Result_;
use identity::account::Signature;
use identity::account::Storage;
use identity::core::encode_b58;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::iota::IotaDID;

use crate::account::WasmChainState;
use crate::account::WasmEncryptionKey;
use crate::account::WasmGeneration;
use crate::account::WasmIdentityState;
use crate::account::WasmKeyLocation;
use crate::did::WasmDID;
use crate::error::AccountResult;
use crate::error::Result;

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;
  #[wasm_bindgen(typescript_type = "Promise<DIDLease>")]
  pub type PromiseDIDLease;
  #[wasm_bindgen(typescript_type = "Promise<PublicKey>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<Signature>")]
  pub type PromiseSignature;
  #[wasm_bindgen(typescript_type = "Promise<Bool>")]
  pub type PromiseBool;
  #[wasm_bindgen(typescript_type = "Promise<Option<Generation>>")]
  pub type PromiseOptionGeneration;
  #[wasm_bindgen(typescript_type = "Promise<Option<ChainState>>")]
  pub type PromiseOptionChainState;
  #[wasm_bindgen(typescript_type = "Promise<Option<IdentityState>>")]
  pub type PromiseOptionIdentityState;

  #[wasm_bindgen(catch, method)]
  pub async fn set_password(this: &WasmStorage, password: WasmEncryptionKey) -> Result<()>;
  #[wasm_bindgen(catch, method)]
  pub async fn flush_changes(this: &WasmStorage) -> Result<()>;
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
  #[wasm_bindgen(catch, method)]
  pub async fn key_del(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> Result<()>;
  #[wasm_bindgen(method)]
  pub fn key_sign(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation, data: Vec<u8>) -> PromiseSignature;
  #[wasm_bindgen(method)]
  pub fn key_exists(this: &WasmStorage, did: WasmDID, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method)]
  pub fn published_generation(this: &WasmStorage, did: WasmDID) -> PromiseOptionGeneration;
  #[wasm_bindgen(catch, method)]
  pub async fn set_published_generation(this: &WasmStorage, did: WasmDID, index: WasmGeneration) -> Result<()>;
  #[wasm_bindgen(method)]
  pub fn chain_state(this: &WasmStorage, did: WasmDID) -> PromiseOptionChainState;
  #[wasm_bindgen(catch, method)]
  pub async fn set_chain_state(this: &WasmStorage, did: WasmDID, chain_state: WasmChainState) -> Result<()>;
  #[wasm_bindgen(method)]
  pub fn state(this: &WasmStorage, did: WasmDID) -> PromiseOptionIdentityState;
  #[wasm_bindgen(catch, method)]
  pub async fn set_state(this: &WasmStorage, did: WasmDID, state: WasmIdentityState) -> Result<()>;
  #[wasm_bindgen(catch, method)]
  pub async fn purge(this: &WasmStorage, did: WasmDID) -> Result<()>;
}

impl Debug for WasmStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmStorage")
  }
}

#[async_trait::async_trait(?Send)]
impl Storage for WasmStorage {
  async fn set_password(&self, password: EncryptionKey) -> Result_<()> {
    self.set_password(password.into()).await.account_result()
  }

  async fn flush_changes(&self) -> Result_<()> {
    self.flush_changes().await.account_result()
  }

  async fn lease_did(&self, did: &IotaDID) -> Result_<DIDLease> {
    let promise = js_sys::Promise::resolve(&self.lease_did(did.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Creates a new keypair at the specified `location`
  async fn key_new(&self, did: &IotaDID, location: &KeyLocation) -> Result_<PublicKey> {
    let promise = js_sys::Promise::resolve(&self.key_new(did.clone().into(), location.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Inserts a private key at the specified `location`.
  async fn key_insert(&self, did: &IotaDID, location: &KeyLocation, private_key: PrivateKey) -> Result_<PublicKey> {
    let promise = js_sys::Promise::resolve(&self.key_insert(
      did.clone().into(),
      location.clone().into(),
      encode_b58(private_key.as_ref()),
    ));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Retrieves the public key at the specified `location`.
  async fn key_get(&self, did: &IotaDID, location: &KeyLocation) -> Result_<PublicKey> {
    let promise = js_sys::Promise::resolve(&self.key_get(did.clone().into(), location.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Deletes the keypair specified by `location`.
  async fn key_del(&self, did: &IotaDID, location: &KeyLocation) -> Result_<()> {
    self
      .key_del(did.clone().into(), location.clone().into())
      .await
      .account_result()
  }

  /// Signs `data` with the private key at the specified `location`.
  async fn key_sign(&self, did: &IotaDID, location: &KeyLocation, data: Vec<u8>) -> Result_<Signature> {
    let promise = js_sys::Promise::resolve(&self.key_sign(did.clone().into(), location.clone().into(), data));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Returns `true` if a keypair exists at the specified `location`.
  async fn key_exists(&self, did: &IotaDID, location: &KeyLocation) -> Result_<bool> {
    let promise = js_sys::Promise::resolve(&self.key_exists(did.clone().into(), location.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Returns the last generation that has been published to the tangle for the given `did`.
  async fn published_generation(&self, did: &IotaDID) -> Result_<Option<Generation>> {
    let promise = js_sys::Promise::resolve(&self.published_generation(did.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Sets the last generation that has been published to the tangle for the given `did`.
  async fn set_published_generation(&self, did: &IotaDID, index: Generation) -> Result_<()> {
    self
      .set_published_generation(did.clone().into(), index.into())
      .await
      .account_result()
  }

  /// Returns the chain state of the identity specified by `did`.
  async fn chain_state(&self, did: &IotaDID) -> Result_<Option<ChainState>> {
    let promise = js_sys::Promise::resolve(&self.chain_state(did.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Set the chain state of the identity specified by `did`.
  async fn set_chain_state(&self, did: &IotaDID, chain_state: &ChainState) -> Result_<()> {
    self
      .set_chain_state(did.clone().into(), chain_state.clone().into())
      .await
      .account_result()
  }

  /// Returns the state of the identity specified by `did`.
  async fn state(&self, did: &IotaDID) -> Result_<Option<IdentityState>> {
    let promise = js_sys::Promise::resolve(&self.state(did.clone().into()));
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    result.map(|js_value| js_value.into_serde().unwrap()).account_result()
  }

  /// Sets a new state for the identity specified by `did`.
  async fn set_state(&self, did: &IotaDID, state: &IdentityState) -> Result_<()> {
    self
      .set_state(did.clone().into(), state.clone().into())
      .await
      .account_result()
  }

  /// Removes the keys and any state for the identity specified by `did`.
  async fn purge(&self, did: &IotaDID) -> Result_<()> {
    self.purge(did.clone().into()).await.account_result()
  }
}