// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_iota::client::ExplorerUrl;
use identity_iota::iota_core::IotaDID;
use identity_iota::iota_core::MessageId;
use wasm_bindgen::prelude::*;

use crate::did::UWasmDID;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = ExplorerUrl)]
#[derive(Clone, Debug)]
pub struct WasmExplorerUrl(ExplorerUrl);

#[wasm_bindgen(js_class = ExplorerUrl)]
impl WasmExplorerUrl {
  /// Constructs a new Tangle explorer URL from a string.
  ///
  /// Use `ExplorerUrl::mainnet` or `ExplorerUrl::devnet` unless using a private Tangle
  /// or local explorer.
  #[wasm_bindgen]
  pub fn parse(url: &str) -> Result<WasmExplorerUrl> {
    ExplorerUrl::parse(url).map(WasmExplorerUrl).wasm_result()
  }

  /// Returns the Tangle explorer URL for the mainnet.
  #[wasm_bindgen]
  pub fn mainnet() -> WasmExplorerUrl {
    Self(ExplorerUrl::mainnet().clone())
  }

  /// Returns the Tangle explorer URL for the devnet.
  #[wasm_bindgen]
  pub fn devnet() -> WasmExplorerUrl {
    Self(ExplorerUrl::devnet().clone())
  }

  /// Returns the web explorer URL of the given `message_id`.
  ///
  /// E.g. https://explorer.iota.org/mainnet/message/{message_id}
  #[wasm_bindgen(js_name = messageUrl)]
  pub fn message_url(&self, message_id: &str) -> Result<String> {
    let message_id = MessageId::from_str(message_id)
      .map_err(identity_iota::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.message_url(&message_id).map(|url| url.to_string()).wasm_result()
  }

  /// Returns the web identity resolver URL for the given DID.
  ///
  /// E.g. https://explorer.iota.org/mainnet/identity-resolver/{did}
  #[wasm_bindgen(js_name = resolverUrl)]
  pub fn resolver_url(&self, did: UWasmDID) -> Result<String> {
    let did: IotaDID = IotaDID::try_from(did)?;
    self.0.resolver_url(&did).map(|url| url.to_string()).wasm_result()
  }

  #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl From<WasmExplorerUrl> for ExplorerUrl {
  fn from(other: WasmExplorerUrl) -> Self {
    other.0
  }
}

impl From<ExplorerUrl> for WasmExplorerUrl {
  fn from(other: ExplorerUrl) -> Self {
    Self(other)
  }
}
