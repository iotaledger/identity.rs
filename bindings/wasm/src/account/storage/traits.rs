// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_iota::account_storage::CekAlgorithm;
use identity_iota::account_storage::DIDType;
use identity_iota::account_storage::EncryptedData;
use identity_iota::account_storage::EncryptionAlgorithm;
use identity_iota::account_storage::Error as AccountStorageError;
use identity_iota::account_storage::KeyLocation;
use identity_iota::account_storage::Result as AccountStorageResult;
use identity_iota::account_storage::Signature;
use identity_iota::account_storage::Storage;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::PublicKey;
use identity_iota::did::CoreDID;
use identity_iota::iota::NetworkName;
use identity_iota::prelude::KeyType;
use js_sys::Array;
use js_sys::Promise;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::account::types::WasmCekAlgorithm;
use crate::account::types::WasmDIDType;
use crate::account::types::WasmEncryptedData;
use crate::account::types::WasmEncryptionAlgorithm;
use crate::account::types::WasmKeyLocation;
use crate::common::PromiseBool;
use crate::common::PromiseVoid;
use crate::crypto::WasmKeyType;
use crate::did::WasmCoreDID;
use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromisePublicKey;
  #[wasm_bindgen(typescript_type = "Promise<Signature>")]
  pub type PromiseSignature;
  #[wasm_bindgen(typescript_type = "Promise<KeyLocation>")]
  pub type PromiseKeyLocation;
  #[wasm_bindgen(typescript_type = "Promise<Array<CoreDID>>")]
  pub type PromiseArrayDID;
  #[wasm_bindgen(typescript_type = "Promise<[CoreDID, KeyLocation]>")]
  pub type PromiseDIDKeyLocation;
  #[wasm_bindgen(typescript_type = "Promise<EncryptedData>")]
  pub type PromiseEncryptedData;
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromiseData;
  #[wasm_bindgen(typescript_type = "Promise<Uint8Array | undefined>")]
  pub type PromiseOptionBytes;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Storage")]
  pub type WasmStorage;

  #[wasm_bindgen(method, js_name = didCreate)]
  pub fn did_create(
    this: &WasmStorage,
    did_type: WasmDIDType,
    network: &str,
    fragment: &str,
    private_key: Option<Vec<u8>>,
  ) -> PromiseDIDKeyLocation;
  #[wasm_bindgen(method, js_name = didPurge)]
  pub fn did_purge(this: &WasmStorage, did: WasmCoreDID) -> PromiseVoid;

  #[wasm_bindgen(method, js_name = didExists)]
  pub fn did_exists(this: &WasmStorage, did: WasmCoreDID) -> PromiseBool;
  #[wasm_bindgen(method, js_name = didList)]
  pub fn did_list(this: &WasmStorage) -> PromiseArrayDID;

  #[wasm_bindgen(method, js_name = keyGenerate)]
  pub fn key_generate(
    this: &WasmStorage,
    did: WasmCoreDID,
    key_type: WasmKeyType,
    fragment: String,
  ) -> PromiseKeyLocation;
  #[wasm_bindgen(method, js_name = keyInsert)]
  pub fn key_insert(
    this: &WasmStorage,
    did: WasmCoreDID,
    location: WasmKeyLocation,
    private_key: Vec<u8>,
  ) -> PromiseVoid;
  #[wasm_bindgen(method, js_name = keyPublic)]
  pub fn key_public(this: &WasmStorage, did: WasmCoreDID, location: WasmKeyLocation) -> PromisePublicKey;
  #[wasm_bindgen(method, js_name = keyDelete)]
  pub fn key_delete(this: &WasmStorage, did: WasmCoreDID, location: WasmKeyLocation) -> PromiseVoid;
  #[wasm_bindgen(method, js_name = keySign)]
  pub fn key_sign(this: &WasmStorage, did: WasmCoreDID, location: WasmKeyLocation, data: Vec<u8>) -> PromiseSignature;
  #[wasm_bindgen(method, js_name = keyExists)]
  pub fn key_exists(this: &WasmStorage, did: WasmCoreDID, location: WasmKeyLocation) -> PromiseBool;
  #[wasm_bindgen(method, js_name = dataEncrypt)]
  pub fn data_encrypt(
    this: &WasmStorage,
    did: WasmCoreDID,
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
    encryption_algorithm: WasmEncryptionAlgorithm,
    cek_algorithm: WasmCekAlgorithm,
    public_key: Vec<u8>,
  ) -> PromiseEncryptedData;
  #[wasm_bindgen(method, js_name = dataDecrypt)]
  pub fn data_decrypt(
    this: &WasmStorage,
    did: WasmCoreDID,
    data: WasmEncryptedData,
    encryption_algorithm: WasmEncryptionAlgorithm,
    cek_algorithm: WasmCekAlgorithm,
    private_key: WasmKeyLocation,
  ) -> Uint8Array;
  #[wasm_bindgen(method, js_name = blobGet)]
  pub fn blob_get(this: &WasmStorage, did: WasmCoreDID) -> PromiseOptionBytes;
  #[wasm_bindgen(method, js_name = blobSet)]
  pub fn blob_set(this: &WasmStorage, did: WasmCoreDID, blob: Vec<u8>) -> PromiseVoid;
  #[wasm_bindgen(method, js_name = flushChanges)]
  pub fn flush_changes(this: &WasmStorage) -> PromiseVoid;
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
    did_type: DIDType,
    network: NetworkName,
    fragment: &str,
    private_key: Option<PrivateKey>,
  ) -> AccountStorageResult<(CoreDID, KeyLocation)> {
    let private_key: Option<Vec<u8>> = private_key.map(|key| {
      let key_bytes: Vec<u8> = key.as_ref().to_vec();
      core::mem::drop(key);
      key_bytes
    });

    let promise: Promise = Promise::resolve(&self.did_create(did_type.into(), network.as_ref(), fragment, private_key));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let did_location_tuple: js_sys::Array = js_sys::Array::from(&result.to_account_error()?);
    let mut did_location_tuple: js_sys::ArrayIter = did_location_tuple.iter();

    let did: CoreDID = did_location_tuple
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

  async fn did_purge(&self, did: &CoreDID) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.did_purge(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn did_exists(&self, did: &CoreDID) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.did_exists(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn did_list(&self) -> AccountStorageResult<Vec<CoreDID>> {
    let promise: Promise = Promise::resolve(&self.did_list());
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.to_account_error()?;

    js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))
  }

  async fn key_generate(&self, did: &CoreDID, key_type: KeyType, fragment: &str) -> AccountStorageResult<KeyLocation> {
    let promise: Promise =
      Promise::resolve(&self.key_generate(did.clone().into(), key_type.into(), fragment.to_owned()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let location: KeyLocation = result
      .to_account_error()?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;

    Ok(location)
  }

  async fn key_insert(
    &self,
    did: &CoreDID,
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

  async fn key_public(&self, did: &CoreDID, location: &KeyLocation) -> AccountStorageResult<PublicKey> {
    let promise: Promise = Promise::resolve(&self.key_public(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let public_key: Vec<u8> = result.to_account_error().map(uint8array_to_bytes)??;
    Ok(public_key.into())
  }

  async fn key_delete(&self, did: &CoreDID, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_delete(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn key_sign(&self, did: &CoreDID, location: &KeyLocation, data: Vec<u8>) -> AccountStorageResult<Signature> {
    let promise: Promise = Promise::resolve(&self.key_sign(did.clone().into(), location.clone().into(), data));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.to_account_error()?;
    let signature: Signature = js_value
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(signature)
  }

  async fn key_exists(&self, did: &CoreDID, location: &KeyLocation) -> AccountStorageResult<bool> {
    let promise: Promise = Promise::resolve(&self.key_exists(did.clone().into(), location.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    result.into()
  }

  async fn data_encrypt(
    &self,
    did: &CoreDID,
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    public_key: PublicKey,
  ) -> AccountStorageResult<EncryptedData> {
    let promise: Promise = Promise::resolve(&self.data_encrypt(
      did.clone().into(),
      plaintext,
      associated_data,
      (*encryption_algorithm).into(),
      cek_algorithm.clone().into(),
      public_key.as_ref().to_vec(),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let encrypted_data: EncryptedData = result
      .to_account_error()?
      .into_serde()
      .map_err(|err| AccountStorageError::SerializationError(err.to_string()))?;
    Ok(encrypted_data)
  }

  async fn data_decrypt(
    &self,
    did: &CoreDID,
    data: EncryptedData,
    encryption_algorithm: &EncryptionAlgorithm,
    cek_algorithm: &CekAlgorithm,
    private_key: &KeyLocation,
  ) -> AccountStorageResult<Vec<u8>> {
    let promise: Promise = Promise::resolve(&self.data_decrypt(
      did.clone().into(),
      data.into(),
      (*encryption_algorithm).into(),
      cek_algorithm.clone().into(),
      private_key.clone().into(),
    ));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let data: Vec<u8> = result.to_account_error().map(uint8array_to_bytes)??;
    Ok(data)
  }

  async fn blob_get(&self, did: &CoreDID) -> AccountStorageResult<Option<Vec<u8>>> {
    let promise: Promise = Promise::resolve(&self.blob_get(did.clone().into()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js_value: JsValue = result.to_account_error()?;
    if js_value.is_null() || js_value.is_undefined() {
      return Ok(None);
    }
    let value: Vec<u8> = uint8array_to_bytes(js_value)?;
    Ok(Some(value))
  }

  async fn blob_set(&self, did: &CoreDID, blob: Vec<u8>) -> AccountStorageResult<()> {
    let promise: Promise = Promise::resolve(&self.blob_set(did.clone().into(), blob));
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
/** An interface for Account storage implementations.

The `Storage` interface is used for secure key operations, such as key generation and signing,
as well as key-value like storage of data structures, such as DID documents.

# Identifiers

Implementations of this interface are expected to uniquely identify keys through the
combination of DID _and_ `KeyLocation`.

An implementation recommendation is to use the DID as a partition key. Everything related to a DID
can be stored in a partition identified by that DID. Keys belonging to a DID can then be identified
by `KeyLocation`s in that partition.

# DID List

The storage is expected to maintain a list of stored DIDs. DIDs created with `did_create` should be
inserted into the list, and removed when calling `did_purge`.
Other operations on the list are `did_exists` and `did_list`.

# Implementation example

See the `MemStore` example for a test implementation. */
interface Storage {
  /** Creates a new identity of the type declared in `did_type` for the given `network`.

   - Uses the given Ed25519 `private_key` or generates a new key if it's `None`.
   - Returns an error if the DID already exists.
   - Adds the newly created DID to a list which can be accessed via `did_list`.

   Returns the generated DID represented as a [`CoreDID`] and the location at which the key was stored. */
  didCreate: (didType: DIDType, network: string, fragment: string, privateKey?: Uint8Array) => Promise<[CoreDID, KeyLocation]>;

  /** Removes the keys and any other state for the given `did`.

   This operation is idempotent: it does not fail if the given `did` does not (or no longer) exist.

   Returns `true` if the did and its associated data was removed, `false` if nothing was done. */
  didPurge: (did: CoreDID) => Promise<boolean>;

  /** Returns `true` if `did` exists in the list of stored DIDs. */
  didExists: (did: CoreDID) => Promise<boolean>;

  /** Returns the list of stored DIDs. */
  didList: () => Promise<Array<CoreDID>>;

  /** Generates a new key for the given `did` with the given `key_type` and `fragment` identifier
   and returns the location of the newly generated key. */
  keyGenerate: (did: CoreDID, keyType: KeyType, fragment: string) => Promise<KeyLocation>;

  /** Inserts a private key at the specified `location`.

   If a key at `location` exists, it is overwritten. */
  keyInsert: (did: CoreDID, keyLocation: KeyLocation, privateKey: Uint8Array) => Promise<void>;

  /** Retrieves the public key from `location`. */
  keyPublic: (did: CoreDID, keyLocation: KeyLocation) => Promise<Uint8Array>;

  /** Deletes the key at `location`.

   This operation is idempotent: it does not fail if the key does not exist.

   Returns `true` if it removed the key, `false` if nothing was done. */
  keyDelete: (did: CoreDID, keyLocation: KeyLocation) => Promise<boolean>;

  /** Signs `data` with the private key at the specified `location`. */
  keySign: (did: CoreDID, keyLocation: KeyLocation, data: Uint8Array) => Promise<Signature>;

  /** Returns `true` if a key exists at the specified `location`. */
  keyExists: (did: CoreDID, keyLocation: KeyLocation) => Promise<boolean>;

  /** Encrypts the given `plaintext` with the specified `encryptionAlgorithm` and `cekAlgorithm`.
   *
   *  Returns an `EncryptedData` instance.
   */
  dataEncrypt: (did: CoreDID, plaintext: Uint8Array, associatedData: Uint8Array, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, publicKey: Uint8Array) => Promise<EncryptedData>;

  /** Decrypts the given `data` with the specified `encryptionAlgorithm` and `cekAlgorithm`.
   *
   *  Returns the decrypted text.
   */
  dataDecrypt: (did: CoreDID, data: EncryptedData, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, privateKey: KeyLocation) => Promise<Uint8Array>;

  /** Returns the blob stored by the identity specified by `did`. */
  blobGet: (did: CoreDID) => Promise<Uint8Array | undefined>;

  /** Stores an arbitrary blob for the identity specified by `did`. */
  blobSet: (did: CoreDID, blob: Uint8Array) => Promise<void>;

  /** Persists any unsaved changes. */
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
