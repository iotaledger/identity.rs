// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use anyhow::Context as _;
use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use fastcrypto::traits::EncodeDecodeBase64;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::TransactionDataBcs;
use jsonpath_rust::JsonPathQuery as _;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;
use serde_json::Value;
use tokio::process::Command;

use super::Error;

/// IOTA Keytool [Signer] implementation.
#[derive(Debug)]
pub struct KeytoolSigner {
  address: IotaAddress,
}

impl KeytoolSigner {
  /// Returns a [KeytoolSigner] that signs data using `address`'s private key.
  pub fn new(address: IotaAddress) -> Self {
    Self { address }
  }

  /// Returns a new [KeytoolSigner] using the address that is returned by
  /// invoking the shell command `$ iota client active-address`.
  pub async fn new_active_address() -> Result<Self, Error> {
    let output = run_iota_cli_command("client active-address")
      .await
      .map_err(Error::AnyError)?;

    let address = String::from_utf8(output)
      .context("command output is not valid utf8")
      .and_then(|s| s.trim().parse().context("command output is not a valid IOTA address"))
      .map_err(Error::AnyError)?;

    Ok(Self { address })
  }

  /// Returns the [IotaAddress] used by this [KeytoolSigner].
  pub fn address(&self) -> IotaAddress {
    self.address
  }
}

#[cfg_attr(feature = "send-sync", async_trait)]
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
impl Signer<IotaKeySignature> for KeytoolSigner {
  type KeyId = IotaAddress;

  fn key_id(&self) -> &Self::KeyId {
    &self.address
  }

  async fn public_key(&self) -> Result<PublicKey, SecretStorageError> {
    let query = format!("$[?(@.iotaAddress==\"{}\")].publicBase64Key", self.address);
    let Value::String(base64_key_str) = run_iota_cli_command("keytool list --json")
      .await
      .and_then(|output_bytes| String::from_utf8(output_bytes).context("command output is not valid utf8"))
      .and_then(|output_str| {
        serde_json::from_str::<Value>(output_str.trim()).context("failed to parse command output to JSON")
      })
      .and_then(|json_value| {
        json_value
          .path(&query)
          .map_err(|e| anyhow!("failed to query JSON output: {e}"))
          .and_then(|mut results| results.get_mut(0).context("key not found").map(Value::take))
      })
      .map_err(SecretStorageError::Other)?
    else {
      return Err(SecretStorageError::Other(anyhow!("keytool key encoding error")));
    };

    PublicKey::decode_base64(&base64_key_str).map_err(|e| SecretStorageError::Other(anyhow!("{e}")))
  }

  async fn sign(&self, data: &TransactionDataBcs) -> Result<Signature, SecretStorageError> {
    let base64_data = Base64::encode(data);
    let command = format!("keytool sign --address {} --data {base64_data} --json", self.address);

    run_iota_cli_command(&command)
      .await
      .and_then(|output_bytes| serde_json::from_slice::<Value>(&output_bytes).context("output is not JSON"))
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

async fn run_iota_cli_command(args: &str) -> anyhow::Result<Vec<u8>> {
  let output = Command::new("iota")
    .args(args.split_ascii_whitespace())
    .output()
    .await
    .map_err(|e| Error::AnyError(anyhow!("failed to run command: {e}")))?;

  if !output.status.success() {
    let err_msg =
      String::from_utf8(output.stderr).map_err(|e| anyhow!("command failed with non-utf8 error message: {e}"))?;
    return Err(anyhow!("failed to run \"iota client active-address\": {err_msg}"));
  }

  Ok(output.stdout)
}
