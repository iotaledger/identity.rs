// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::verification::MethodScope;
use identity_iota::did::IotaDID;
use identity_iota::tangle::MessageId;

use crate::error::Result;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyMethodRef;
use crate::identity::TinyService;
use crate::types::Fragment;
use crate::types::UnixTimestamp;

/// Event data tagged with a timestamp.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Event {
  data: EventData,
  time: UnixTimestamp,
}

impl Event {
  /// Creates a new `Event` instance.
  pub fn new(data: EventData) -> Self {
    Self {
      data,
      time: UnixTimestamp::now(),
    }
  }

  /// Returns a reference to the raw event data.
  pub const fn data(&self) -> &EventData {
    &self.data
  }

  /// Returns the unix timestamp of when the event was created.
  pub const fn time(&self) -> UnixTimestamp {
    self.time
  }

  /// Returns a new state created by applying the event to the given `state`.
  pub async fn apply(self, mut state: IdentityState) -> Result<IdentityState> {
    debug!("[Event::apply] Event = {:?}", self);
    trace!("[Event::apply] State = {:?}", state);

    match self.data {
      EventData::AuthMessage(message) => {
        state.set_auth_message_id(message);
        state.increment_auth_generation()?;
      }
      EventData::DiffMessage(message) => {
        state.set_diff_message_id(message);
        state.increment_diff_generation()?;
      }
      EventData::IdentityCreated(did) => {
        state.set_did(did);
        state.set_created(self.time);
        state.set_updated(self.time);
      }
      EventData::MethodCreated(scope, method) => {
        state.methods_mut().insert(scope, TinyMethodRef::Embed(method));
        state.set_updated(self.time);
      }
      EventData::MethodDeleted(fragment) => {
        state.methods_mut().delete(fragment.name());
        state.set_updated(self.time);
      }
      EventData::MethodAttached(fragment, scopes) => {
        let method: TinyMethodRef = TinyMethodRef::Refer(fragment);

        for scope in scopes {
          state.methods_mut().insert(scope, method.clone());
        }

        state.set_updated(self.time);
      }
      EventData::MethodDetached(fragment, scopes) => {
        for scope in scopes {
          state.methods_mut().detach(scope, fragment.name());
        }

        state.set_updated(self.time);
      }
      EventData::ServiceCreated(service) => {
        state.services_mut().insert(service);
        state.set_updated(self.time);
      }
      EventData::ServiceDeleted(fragment) => {
        state.services_mut().delete(fragment.name());
        state.set_updated(self.time);
      }
    }

    Ok(state)
  }
}

// =============================================================================
// EventData
// =============================================================================

/// Raw event data.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum EventData {
  /// Emitted when a new auth message is published to the IOTA Tangle.
  AuthMessage(MessageId),
  /// Emitted when a new diff message is published to the IOTA Tangle.
  DiffMessage(MessageId),
  /// Emitted when a new identity state is created.
  IdentityCreated(IotaDID),
  /// Emitted when a new verification method is created.
  MethodCreated(MethodScope, TinyMethod),
  /// Emitted when a verification method is deleted.
  MethodDeleted(Fragment),
  /// Emitted when a verification method is attached to one or more scopes.
  MethodAttached(Fragment, Vec<MethodScope>),
  /// Emitted when a verification method is detached from one or more scopes.
  MethodDetached(Fragment, Vec<MethodScope>),
  /// Emitted when a new service is created.
  ServiceCreated(TinyService),
  /// Emitted when a service is deleted.
  ServiceDeleted(Fragment),
}
