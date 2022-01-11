use wasm_bindgen::prelude::*;

use crate::account::WasmChainState;
use crate::account::WasmEncryptionKey;
use crate::account::WasmGeneration;
use crate::account::WasmIdentityState;
use crate::account::WasmKeyLocation;
use crate::did::WasmDID;
use crate::error::Result;

#[wasm_bindgen]
extern "C" {
  pub type WasmStorage;

  pub type PromiseDIDLease;
  pub type PromisePublicKey;
  pub type PromiseSignature;
  pub type PromiseBool;
  pub type PromiseOptionGeneration;
  pub type PromiseOptionChainState;
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
