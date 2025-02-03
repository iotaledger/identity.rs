// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use crate::types::base_types::IotaAddress;
use crate::types::crypto::PublicKey;
use crate::types::crypto::Signature;
use crate::IotaKeySignature;
use crate::TransactionDataBcs;
use anyhow::anyhow;
use anyhow::Context as _;
use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use fastcrypto::traits::EncodeDecodeBase64;
use jsonpath_rust::JsonPathQuery as _;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;
use serde_json::Value;

/// Builder structure to ease the creation of a [KeytoolSigner].
#[derive(Debug, Default)]
pub struct KeytoolSignerBuilder {
  address: Option<IotaAddress>,
  iota_bin: Option<PathBuf>,
}

impl KeytoolSignerBuilder {
  /// Returns a new [KeytoolSignerBuilder] with default configuration:
  /// - use current active address;
  /// - assumes `iota` binary to be in PATH;
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the address the signer will use.
  /// Defaults to current active address if no address is provided.
  pub fn with_address(mut self, address: IotaAddress) -> Self {
    self.address = Some(address);
    self
  }

  /// Sets the path to the `iota` binary to use.
  /// Assumes `iota` to be in PATH if no value is provided.
  pub fn iota_bin_location(mut self, path: impl AsRef<Path>) -> Self {
    let path = path.as_ref().to_path_buf();
    self.iota_bin = Some(path);

    self
  }

  /// Builds a new [KeytoolSigner] using the provided configuration.
  pub async fn build(self) -> anyhow::Result<KeytoolSigner> {
    let KeytoolSignerBuilder { address, iota_bin } = self;
    let iota_bin = iota_bin.unwrap_or_else(|| "iota".into());
    let address = if let Some(address) = address {
      address
    } else {
      get_active_address(&iota_bin).await?
    };

    let public_key = get_key(&iota_bin, address).await.context("cannot find key")?;

    Ok(KeytoolSigner {
      public_key,
      iota_bin,
      address,
    })
  }
}

/// IOTA Keytool [Signer] implementation.
#[derive(Debug)]
pub struct KeytoolSigner {
  public_key: PublicKey,
  iota_bin: PathBuf,
  address: IotaAddress,
}

impl KeytoolSigner {
  /// Returns a [KeytoolSignerBuilder].
  pub fn builder() -> KeytoolSignerBuilder {
    KeytoolSignerBuilder::default()
  }

  /// Returns the [IotaAddress] used by this [KeytoolSigner].
  pub fn address(&self) -> IotaAddress {
    self.address
  }

  /// Returns the [PublicKey] used by this [KeytoolSigner].
  pub fn public_key(&self) -> &PublicKey {
    &self.public_key
  }

  async fn run_iota_cli_command(&self, args: &str) -> anyhow::Result<Value> {
    run_iota_cli_command_with_bin(&self.iota_bin, args).await
  }
}

#[cfg_attr(feature = "send-sync-transaction", async_trait)]
#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
impl Signer<IotaKeySignature> for KeytoolSigner {
  type KeyId = IotaAddress;

  fn key_id(&self) -> &Self::KeyId {
    &self.address
  }

  async fn public_key(&self) -> Result<PublicKey, SecretStorageError> {
    Ok(self.public_key.clone())
  }

  async fn sign(&self, data: &TransactionDataBcs) -> Result<Signature, SecretStorageError> {
    let base64_data = Base64::encode(data);
    let command = format!("keytool sign --address {} --data {base64_data}", self.address);

    self
      .run_iota_cli_command(&command)
      .await
      .and_then(|json| {
        json
          .get("iotaSignature")
          .context("invalid JSON output: missing iotaSignature")?
          .as_str()
          .context("not a JSON string")?
          .parse()
          .map_err(|e| anyhow!("invalid IOTA signature: {e}"))
      })
      .map_err(SecretStorageError::Other)
  }
}

async fn run_iota_cli_command_with_bin(iota_bin: impl AsRef<Path>, args: &str) -> anyhow::Result<Value> {
  let iota_bin = iota_bin.as_ref();

  cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
    let output = tokio::process::Command::new(iota_bin)
      .args(args.split_ascii_whitespace())
      .arg("--json")
      .output()
      .await
      .map_err(|e| anyhow!("failed to run command: {e}"))?;

    if !output.status.success() {
      let err_msg =
        String::from_utf8(output.stderr).map_err(|e| anyhow!("command failed with non-utf8 error message: {e}"))?;
      return Err(anyhow!("failed to run \"iota client active-address\": {err_msg}"));
    }

    serde_json::from_slice(&output.stdout).context("invalid JSON object in command output")
    } else {
      extern "Rust" {
        async fn __wasm_exec_iota_cmd(cmd: &str) -> anyhow::Result<Value>;
      }
      let cmd = format!("{iota_bin} {args} --json");
      unsafe { __wasm_exec_iota_cmd(&cmd).await }
    }
  }
}

async fn get_active_address(iota_bin: impl AsRef<Path>) -> anyhow::Result<IotaAddress> {
  run_iota_cli_command_with_bin(iota_bin, "client active-address")
    .await
    .and_then(|value| serde_json::from_value(value).context("failed to parse IotaAddress from output"))
}

async fn get_key(iota_bin: impl AsRef<Path>, address: IotaAddress) -> anyhow::Result<PublicKey> {
  let query = format!("$[?(@.iotaAddress==\"{}\")].publicBase64Key", address);

  let base64_pk_json = run_iota_cli_command_with_bin(iota_bin, "keytool list")
    .await
    .and_then(|json_value| {
      json_value
        .path(&query)
        .map_err(|e| anyhow!("failed to query JSON output: {e}"))?
        .get_mut(0)
        .context("key not found")
        .map(Value::take)
    })?;
  let base64_pk = base64_pk_json
    .as_str()
    .context("invalid JSON public key representation")?;

  PublicKey::decode_base64(base64_pk).context("failed to decode base64 public key")
}
