// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::identity::ChainState;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_iota_core_legacy::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;

/// Holds the internal state for the identity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub(crate) struct IdentityState {
  version: StateVersion,
  document: Option<Vec<u8>>,
  chain_state: Option<Vec<u8>>,
}

impl IdentityState {
  /// Creates a new [`IdentityState`].
  pub(crate) fn new(document: Option<&IotaDocument>, chain_state: Option<&ChainState>) -> Result<Self> {
    let document: Option<Vec<u8>> = document.map(|iota_doc| iota_doc.to_json_vec()).transpose()?;
    let chain_state: Option<Vec<u8>> = chain_state.map(|chain_state| chain_state.to_json_vec()).transpose()?;
    Ok(IdentityState {
      version: StateVersion::default(),
      document,
      chain_state,
    })
  }

  /// Returns the deserialized [`IotaDocument`].
  pub(crate) fn document(&self) -> Result<Option<IotaDocument>> {
    match self.version {
      StateVersion::V1 => Ok(self.document.as_ref().map(IotaDocument::from_json_slice).transpose()?),
    }
  }

  /// Returns the deserialized [`ChainState`].
  pub(crate) fn chain_state(&self) -> Result<Option<ChainState>> {
    match self.version {
      StateVersion::V1 => Ok(self.chain_state.as_ref().map(ChainState::from_json_slice).transpose()?),
    }
  }

  #[allow(dead_code)]
  /// Returns the [`StateVersion`] of the [`IdentityState`].
  pub(crate) fn version(&self) -> StateVersion {
    self.version
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) enum StateVersion {
  #[default]
  V1 = 1,
}
