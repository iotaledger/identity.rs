// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::crypto::IotaKeyPair;
use identity_iota_interaction::types::crypto::SignatureScheme;
use identity_iota_interaction::KeytoolStorage;
use js_sys::Array;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::WasmPublicKey;

use super::signer::WasmKeytoolSigner;

const __TS_IMPORTS: &str = r#"
import { PublicKey } from "@iota/iota-sdk/cryptography";
"#;

fn make_pk_alias_tuple(pk: &WasmPublicKey, alias: &str) -> Array {
  let arr = Array::new();
  arr.push(pk.as_ref());
  arr.push(JsString::from(alias).as_ref());

  arr
}

/// IOTA Keytool CLI wrapper.
#[derive(Default, Clone)]
#[wasm_bindgen(js_name = KeytoolStorage)]
pub struct WasmKeytoolStorage(pub(crate) KeytoolStorage);

impl AsRef<KeytoolStorage> for WasmKeytoolStorage {
  fn as_ref(&self) -> &KeytoolStorage {
    &self.0
  }
}

#[wasm_bindgen(js_class = KeytoolStorage)]
impl WasmKeytoolStorage {
  /// Creates a new {@link KeytoolStorage} that wraps the given iota binary.
  /// Attempts to use the one in PATH if none is provided.
  #[wasm_bindgen(constructor)]
  pub fn new(iota_bin: Option<String>) -> Self {
    iota_bin
      .as_deref()
      .map(KeytoolStorage::new_with_custom_bin)
      .map(Self)
      .unwrap_or_default()
  }

  /// Returns a {@link KeytoolSigner} that will use the provided `address`
  /// to sign transactions. If no address is provided the current active
  /// one will be used.
  pub fn signer(&self, address: Option<String>) -> Result<WasmKeytoolSigner> {
    let address = address.map(|s| IotaAddress::from_str(&s)).transpose().wasm_result()?;
    let mut signer_builder = self.0.signer();
    if let Some(address) = address {
      signer_builder = signer_builder.with_address(address);
    }

    signer_builder.build().map(WasmKeytoolSigner).wasm_result()
  }

  /// Creates a new key of type `key_scheme`.
  /// Returns the tuple ([`PublicKey`](https://docs.iota.org/ts-sdk/api/cryptography/classes/PublicKey), alias).
  #[wasm_bindgen(
    js_name = generateKey,
    unchecked_return_type = "[PublicKey, string]",
  )]
  pub fn generate_key(
    &self,
    #[wasm_bindgen(unchecked_param_type = "'ed25519' | 'secp256r1' | 'secp256k1'")] key_scheme: &str,
  ) -> Result<Array> {
    let key_scheme = match key_scheme {
      "ed25519" => SignatureScheme::ED25519,
      "secp256r1" => SignatureScheme::Secp256r1,
      "secp256k1" => SignatureScheme::Secp256k1,
      _ => return Err(JsError::new("invalid key type").into()),
    };

    self
      .0
      .generate_key(key_scheme)
      .wasm_result()
      .map(|(pk, alias)| make_pk_alias_tuple(&WasmPublicKey::try_from(&pk).unwrap(), &alias))
  }

  /// Inserts a Bech32-encoded private key in the keystore.
  /// The key must use the prefix `iotaprivkey`.
  ///
  /// Returns the key's alias.
  #[wasm_bindgen(js_name = insertKey)]
  pub fn insert_key(&self, bech32_secret_key: &str) -> Result<String> {
    let key = IotaKeyPair::decode(bech32_secret_key)
      .map_err(|e| anyhow::anyhow!("{e:?}"))
      .wasm_result()?;
    self.0.insert_key(key).wasm_result()
  }

  /// Signs `data` with `address`'s secret key.
  #[wasm_bindgen(js_name = signRaw)]
  pub fn sign_raw(&self, address: &str, data: &[u8]) -> Result<Vec<u8>> {
    let address = address.parse().wasm_result()?;
    self.0.sign_raw(address, data).wasm_result()
  }

  /// Updates an alias from `old_alias` to `new_alias`
  /// If no value for `new_alias` is provided, a randomly generated one will be used.
  #[wasm_bindgen(js_name = updateAlias)]
  pub fn update_alias(&self, old_alias: &str, new_alias: Option<String>) -> Result<()> {
    let new_alias = new_alias.as_deref();
    self.0.update_alias(old_alias, new_alias).wasm_result()
  }

  /// Returns the [`PublicKey`](https://docs.iota.org/ts-sdk/api/cryptography/classes/PublicKey) for the given address together with its alias.
  #[wasm_bindgen(
    js_name = getKey,
    unchecked_return_type = "[PublicKey, string]",
  )]
  pub fn get_key(&self, address: &str) -> Result<Array> {
    let address = address.parse().wasm_result()?;
    self
      .0
      .get_key(address)
      .wasm_result()?
      .map(|(pk, alias)| make_pk_alias_tuple(&WasmPublicKey::try_from(&pk).unwrap(), &alias))
      .ok_or_else(|| anyhow::anyhow!("the requested address is not in the keystore"))
      .wasm_result()
  }

  /// Returns the [`PublicKey`](https://docs.iota.org/ts-sdk/api/cryptography/classes/PublicKey) that has the given alias.
  #[wasm_bindgen(js_name = getKeyByAlias)]
  pub fn get_key_by_alias(&self, alias: &str) -> Result<WasmPublicKey> {
    self
      .0
      .get_key_by_alias(alias)
      .wasm_result()?
      .map(|pk| WasmPublicKey::try_from(&pk).unwrap())
      .ok_or_else(|| anyhow::anyhow!("the requested alias is not in the keystore"))
      .wasm_result()
  }
}
