// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use anyhow::anyhow;
use anyhow::Context as _;
use iota_sdk::types::crypto::IotaKeyPair;
use serde::Deserialize;

use crate::types::base_types::IotaAddress;
use crate::types::crypto::PublicKey;
use crate::types::crypto::SignatureScheme as IotaSignatureScheme;

use super::internal::IotaCliWrapper;
use super::KeytoolSignerBuilder;

#[derive(Clone, Default)]
pub struct KeytoolStorage {
  iota_cli_wrapper: IotaCliWrapper,
}

impl KeytoolStorage {
  /// Returns a new [KeytoolStorage] that will use the IOTA binary in PATH.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns a new [KeytoolStorage] that will use the provided IOTA binary.
  pub fn new_with_custom_bin(iota_bin: impl AsRef<Path>) -> Self {
    Self {
      iota_cli_wrapper: IotaCliWrapper::new_with_custom_bin(iota_bin),
    }
  }

  /// Returns a [KeytoolSignerBuilder] to construct a [super::KeytoolSigner] after
  /// selecting an address.
  pub fn signer(&self) -> KeytoolSignerBuilder {
    KeytoolSignerBuilder::new().iota_bin_location(self.iota_cli_wrapper.iota_bin())
  }

  /// Generates a new keypair of type `key_scheme`.
  /// Returns the resulting [PublicKey] together with its alias.
  pub fn generate_key(&self, key_scheme: IotaSignatureScheme) -> anyhow::Result<(PublicKey, String)> {
    if !matches!(
      &key_scheme,
      IotaSignatureScheme::ED25519 | IotaSignatureScheme::Secp256k1 | IotaSignatureScheme::Secp256r1
    ) {
      anyhow::bail!("key scheme {key_scheme} is not supported by the keytool");
    }

    let cmd = format!("client new-address --key-scheme {key_scheme}");
    let KeyGenOutput { alias, address } = {
      let json_output = self.iota_cli_wrapper.run_command(&cmd)?;
      serde_json::from_value(json_output)?
    };

    let pk = self
      .iota_cli_wrapper
      .get_key(address)?
      .ok_or_else(|| anyhow!("key for address {address} wasn't found"))?
      .0;

    Ok((pk, alias))
  }

  /// Inserts a new key in this keytool.
  /// Returns the alias assigned to the inserted key.
  pub fn insert_key(&self, key: IotaKeyPair) -> anyhow::Result<String> {
    let bech32_encoded_key = key.encode().map_err(|e| anyhow!("{e:?}"))?;
    let key_scheme = key.public().scheme().to_string();
    let cmd = format!("keytool import {bech32_encoded_key} {key_scheme}");

    let json_output = self.iota_cli_wrapper.run_command(&cmd)?;
    let KeyGenOutput { alias, .. } = serde_json::from_value(json_output)?;

    Ok(alias)
  }

  /// Updates an alias from `old_alias` to `new_alias`
  /// If no value for `new_alias` is provided, a randomly generated one will be used.
  pub fn update_alias(&self, old_alias: &str, new_alias: Option<&str>) -> anyhow::Result<()> {
    let cmd = format!("keytool update-alias {old_alias} {}", new_alias.unwrap_or_default());
    self
      .iota_cli_wrapper
      .run_command(&cmd)
      .context("failed to update alias")?;

    Ok(())
  }

  /// Returns the [PublicKey] for the given [IotaAddress] together with its alias.
  pub fn get_key(&self, address: IotaAddress) -> anyhow::Result<Option<(PublicKey, String)>> {
    self.iota_cli_wrapper.get_key(address)
  }

  /// Returns the [PublicKey] that has the given alias, if any.
  pub fn get_key_by_alias(&self, alias: &str) -> anyhow::Result<Option<PublicKey>> {
    self.iota_cli_wrapper.get_key_by_alias(alias)
  }
}

#[derive(Deserialize)]
struct KeyGenOutput {
  alias: String,
  address: IotaAddress,
}
