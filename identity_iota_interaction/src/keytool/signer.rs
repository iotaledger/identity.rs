// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use crate::types::base_types::IotaAddress;
use crate::types::crypto::PublicKey;
use crate::types::crypto::Signature;
use crate::types::transaction::TransactionData;
use crate::IotaKeySignature;
use anyhow::anyhow;
use anyhow::Context as _;
use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;

use super::internal::IotaCliWrapper;

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
  pub fn build(self) -> anyhow::Result<KeytoolSigner> {
    let KeytoolSignerBuilder { address, iota_bin } = self;
    let iota_cli_wrapper = iota_bin.map(IotaCliWrapper::new_with_custom_bin).unwrap_or_default();
    let address = if let Some(address) = address {
      address
    } else {
      iota_cli_wrapper.get_active_address()?
    };

    let public_key = iota_cli_wrapper.get_key(address)?.context("key doens't exist")?.0;

    Ok(KeytoolSigner {
      public_key,
      iota_cli_wrapper,
      address,
    })
  }
}

/// IOTA Keytool [Signer] implementation.
#[derive(Debug)]
pub struct KeytoolSigner {
  public_key: PublicKey,
  iota_cli_wrapper: IotaCliWrapper,
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
}

#[cfg_attr(feature = "send-sync-transaction", async_trait)]
#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
impl Signer<IotaKeySignature> for KeytoolSigner {
  type KeyId = IotaAddress;

  fn key_id(&self) -> Self::KeyId {
    self.address
  }

  async fn public_key(&self) -> Result<PublicKey, SecretStorageError> {
    Ok(self.public_key.clone())
  }

  async fn sign(&self, data: &TransactionData) -> Result<Signature, SecretStorageError> {
    let tx_data_bcs =
      bcs::to_bytes(data).map_err(|e| SecretStorageError::Other(anyhow!("bcs serialization failed: {e}")))?;
    let base64_data = Base64::encode(&tx_data_bcs);
    let command = format!("keytool sign --address {} --data {base64_data}", self.address);

    self
      .iota_cli_wrapper
      .run_command(&command)
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
