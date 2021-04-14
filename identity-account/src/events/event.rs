// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::verification::MethodScope;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;

use crate::chain::ChainData;
use crate::chain::TinyMethod;
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
    method: TinyMethod,
    timestamp: Timestamp,
  },
  MethodCreated {
    scope: MethodScope,
    method: TinyMethod,
    timestamp: Timestamp,
  },
  MethodDeleted {
    fragment: String,
    scope: Option<MethodScope>,
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
        state.increment_auth_index()?;
      }
      Self::DiffMessage { message } => {
        state.set_diff_message_id(message);
        state.increment_diff_index()?;
      }
      Self::ChainCreated {
        document,
        method,
        timestamp,
      } => {
        state.set_document(document);
        state.set_created(timestamp);
        state.methods_mut().insert(MethodScope::VerificationMethod, method);
      }
      Self::MethodCreated {
        scope,
        method,
        timestamp,
      } => {
        state.set_updated(timestamp);
        state.methods_mut().insert(scope, method);
      }
      Self::MethodDeleted {
        fragment,
        scope,
        timestamp,
      } => {
        state.set_updated(timestamp);

        if let Some(scope) = scope {
          state.methods_mut().detach(scope, &fragment);
        } else {
          state.methods_mut().delete(&fragment);
        }
      }
    }

    Ok(state)
  }
}
