// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::verification::MethodScope;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;

use crate::chain::ChainData;
use crate::chain::ChainKey;
use crate::error::Result;
use crate::types::ChainId;
use crate::types::Timestamp;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
  AuthMessage {
    message: MessageId,
  },
  DiffMessage {
    message: MessageId,
  },

  ChainCreated {
    document: DID,
    timestamp: Timestamp,
  },
  MethodCreated {
    scope: MethodScope,
    location: ChainKey,
    timestamp: Timestamp,
  },
}

impl Event {
  pub fn respond_one(event: Self) -> Result<Option<Vec<Self>>> {
    Self::respond_many(vec![event])
  }

  pub fn respond_many(events: Vec<Self>) -> Result<Option<Vec<Self>>> {
    Ok(Some(events))
  }

  pub async fn apply(self, mut state: ChainData) -> Result<ChainData> {
    let chain: ChainId = state.chain();

    debug!("[Commit::apply] Chain = {:#?}", chain);
    debug!("[Commit::apply] Event = {:#?}", self);
    trace!("[Commit::apply] State = {:#?}", state);

    match self {
      Self::AuthMessage { message } => {
        state.set_auth_message_id(message);
        // state.auth_index.try_increment()?;
      }
      Self::DiffMessage { message } => {
        state.set_diff_message_id(message);
        // state.diff_index.try_increment()?;
      }
      Self::ChainCreated { document, timestamp } => {
        state.set_created(timestamp);
        state.set_document(document);
      }
      Self::MethodCreated {
        scope,
        location,
        timestamp,
      } => {
        state.append_method(scope, location);
        state.set_updated(timestamp);
      }
    }

    Ok(state)
  }
}
