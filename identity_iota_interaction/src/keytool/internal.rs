// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr as _;

use anyhow::anyhow;
use anyhow::Context as _;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding as _;
use jsonpath_rust::JsonPathQuery as _;
use serde::Deserialize;
use serde_json::Value;

use crate::types::base_types::IotaAddress;
use crate::types::crypto::PublicKey;
use crate::types::crypto::SignatureScheme;

#[derive(Debug, Clone)]
pub(super) struct IotaCliWrapper {
  iota_bin: PathBuf,
}

impl Default for IotaCliWrapper {
  fn default() -> Self {
    Self {
      iota_bin: PathBuf::from_str("iota").expect("infallible"),
    }
  }
}

impl IotaCliWrapper {
  /// Creates a new [IotaCliWrapper] that will use the iota binary found at
  /// the provided path.
  pub fn new_with_custom_bin(iota_bin: impl AsRef<Path>) -> Self {
    Self {
      iota_bin: iota_bin.as_ref().to_owned(),
    }
  }

  /// Returns the location of the iota binary used.
  pub fn iota_bin(&self) -> &Path {
    &self.iota_bin
  }

  /// Executes a given "iota" command with the provided string-encoded args.
  /// Returns the parsed JSON output.
  pub fn run_command(&self, args: &str) -> anyhow::Result<Value> {
    cfg_if::cfg_if! {
      if #[cfg(not(target_arch = "wasm32"))] {
      let output = std::process::Command::new(&self.iota_bin)
        .args(args.split_ascii_whitespace())
        .arg("--json")
        .output()
        .map_err(|e| anyhow!("failed to run command: {e}"))?;

      if !output.status.success() {
        let err_msg =
          String::from_utf8(output.stderr).map_err(|e| anyhow!("command failed with non-utf8 error message: {e}"))?;
        return Err(anyhow!("failed to run keytool cmd: {err_msg}"));
      }

      let trimmed_output = {
        let start_of_json = output.stdout.iter().enumerate().find_map(|(i, b)| matches!(*b, b'[' | b'{' | b'\"').then_some(i)).context("no JSON in command output")?;
        &output.stdout[start_of_json..]
      };

      serde_json::from_slice(trimmed_output).context("invalid JSON object in command output")
      } else {
        extern "Rust" {
          fn __wasm_exec_iota_cmd(cmd: &str) -> anyhow::Result<Value>;
        }
        let iota_bin = self.iota_bin.to_str().context("invalid IOTA bin path")?;
        let cmd = format!("{iota_bin} {args} --json");
        unsafe { __wasm_exec_iota_cmd(&cmd) }
      }
    }
  }

  /// Returns the current active address.
  pub fn get_active_address(&self) -> anyhow::Result<IotaAddress> {
    self
      .run_command("client active-address")
      .and_then(|value| serde_json::from_value(value).context("failed to parse IotaAddress from output"))
  }

  fn get_key_impl(&self, json_path_query: &str) -> anyhow::Result<Option<(PublicKey, String)>> {
    let Some(pk_json_data) = self
      .run_command("keytool list")?
      .path(json_path_query)
      .map_err(|e| anyhow!("failed to query JSON output: {e}"))?
      .get_mut(0)
      .map(Value::take)
    else {
      return Ok(None);
    };

    let KeytoolPublicKeyHelper {
      public_base64_key,
      flag,
      alias,
      ..
    } = serde_json::from_value(pk_json_data)?;

    let signature_scheme =
      SignatureScheme::from_flag_byte(&flag).context(format!("invalid signature flag `{flag}`"))?;
    let pk_bytes = Base64::decode(&public_base64_key).context("invalid base64 encoding for key")?;

    let pk = PublicKey::try_from_bytes(signature_scheme, &pk_bytes).map_err(|e| anyhow!("{e:?}"))?;

    Ok(Some((pk, alias)))
  }

  /// Returns the public key of a given address, if any.
  pub fn get_key(&self, address: IotaAddress) -> anyhow::Result<Option<(PublicKey, String)>> {
    let query = format!("$[?(@.iotaAddress==\"{}\")]", address);
    self.get_key_impl(&query)
  }

  /// Returns the public key with the given alias, if any.
  pub fn get_key_by_alias(&self, alias: &str) -> anyhow::Result<Option<PublicKey>> {
    let query = format!("$[?(@.alias==\"{}\")]", alias);
    Ok(self.get_key_impl(&query)?.map(|(pk, _)| pk))
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeytoolPublicKeyHelper {
  alias: String,
  public_base64_key: String,
  flag: u8,
}
