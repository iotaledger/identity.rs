// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::identity::ChainState;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

/// Holds the internal state for the identity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityState {
  version: StateVersion,
  document: Option<Vec<u8>>,
  chain_state: Option<Vec<u8>>,
}

impl IdentityState {
  /// Creates a new [`IdentityState`].
  pub fn new(document: Option<&IotaDocument>, chain_state: Option<&ChainState>) -> Result<Self> {
    let document: Option<Vec<u8>> = document
      .map(|iota_doc| {
        serde_json::to_vec(iota_doc)
          .map_err(|e| Error::SerializationError("unable to serialize document".to_owned(), e))
      })
      .transpose()?;
    let chain_state: Option<Vec<u8>> = chain_state
      .map(|chain_state| {
        serde_json::to_vec(chain_state)
          .map_err(|e| Error::SerializationError("unable to serialize chain state".to_owned(), e))
      })
      .transpose()?;
    Ok(IdentityState {
      version: StateVersion::default(),
      document,
      chain_state,
    })
  }

  /// Returns the deserialized [`IotaDocument`].
  pub fn document(&self) -> Result<Option<IotaDocument>> {
    match self.version {
      StateVersion::V0 => Ok(
        self
          .document
          .as_ref()
          .map(|bytes| {
            serde_json::from_slice(bytes)
              .map_err(|e| Error::SerializationError("unable to deserialize document".to_owned(), e))
          })
          .transpose()?,
      ),
    }
  }

  /// Returns the deserialized [`ChainState`].
  pub fn chain_state(&self) -> Result<Option<ChainState>> {
    match self.version {
      StateVersion::V0 => Ok(
        self
          .chain_state
          .as_ref()
          .map(|bytes| {
            serde_json::from_slice(bytes)
              .map_err(|e| Error::SerializationError("unable to deserialize chain state".to_owned(), e))
          })
          .transpose()?,
      ),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
enum StateVersion {
  #[default]
  V0,
}
