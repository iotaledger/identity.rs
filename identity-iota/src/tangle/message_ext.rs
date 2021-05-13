// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use iota_client::bee_message::payload::transaction::Essence;
use iota_client::bee_message::payload::Payload;
use iota_client::bee_message::Message;
use iota_client::bee_message::MessageId;
use iota_client::bee_message::MESSAGE_ID_LENGTH;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::TangleRef;

// TODO: Use MessageId when it has a const ctor
static NULL: &[u8; MESSAGE_ID_LENGTH] = &[0; MESSAGE_ID_LENGTH];

macro_rules! try_extract {
  ($ty:ty, $this:expr, $did:expr) => {{
    match $this.payload() {
      Some(Payload::Indexation(payload)) => {
        let mut resource: $ty = <$ty>::from_json_slice(payload.data()).ok()?;

        if $did.authority() != resource.id().authority() {
          return None;
        }

        TangleRef::set_message_id(&mut resource, $this.id().0);

        Some(resource)
      }
      Some(Payload::Transaction(tx_payload)) => match tx_payload.essence() {
        Essence::Regular(regular_essence) => match regular_essence.payload() {
          Some(Payload::Indexation(payload)) => {
            let mut resource: $ty = <$ty>::from_json_slice(payload.data()).ok()?;

            if $did.authority() != resource.id().authority() {
              return None;
            }

            TangleRef::set_message_id(&mut resource, $this.id().0);

            Some(resource)
          }
          _ => None,
        },
      },
      _ => None,
    }
  }};
}

pub trait MessageIdExt: Sized {
  fn is_null(&self) -> bool;

  fn encode_hex(&self) -> String;

  fn decode_hex(hex: &str) -> Result<Self>;
}

impl MessageIdExt for MessageId {
  fn is_null(&self) -> bool {
    self.as_ref() == NULL
  }

  fn encode_hex(&self) -> String {
    self.to_string()
  }

  fn decode_hex(hex: &str) -> Result<Self> {
    hex.parse().map_err(Into::into)
  }
}

pub trait MessageExt {
  fn try_extract_document(&self, did: &IotaDID) -> Option<IotaDocument>;

  fn try_extract_diff(&self, did: &IotaDID) -> Option<DocumentDiff>;
}

impl MessageExt for Message {
  fn try_extract_document(&self, did: &IotaDID) -> Option<IotaDocument> {
    try_extract!(IotaDocument, self, did)
  }

  fn try_extract_diff(&self, did: &IotaDID) -> Option<DocumentDiff> {
    try_extract!(DocumentDiff, self, did)
  }
}
