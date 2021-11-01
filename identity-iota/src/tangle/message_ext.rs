// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::payload::transaction::Essence;
use iota_client::bee_message::payload::Payload;
use iota_client::bee_message::Message;
use iota_client::bee_message::MessageId;
use iota_client::bee_message::MESSAGE_ID_LENGTH;

use identity_core::convert::FromJson;
use identity_did::did::DID;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::TangleRef;

// TODO: Use MessageId when it has a const ctor
static NULL: &[u8; MESSAGE_ID_LENGTH] = &[0; MESSAGE_ID_LENGTH];

fn parse_message<T: FromJson + TangleRef>(message: &Message, did: &IotaDID) -> Option<T> {
  let message_id: MessageId = message.id().0;
  let payload: Option<&Payload> = message.payload().as_ref();
  let resource: T = parse_payload(message_id, payload)?;

  if did.authority() != resource.did().authority() {
    return None;
  }

  Some(resource)
}

fn parse_payload<T: FromJson + TangleRef>(message_id: MessageId, payload: Option<&Payload>) -> Option<T> {
  match payload {
    Some(Payload::Indexation(indexation)) => parse_data(message_id, indexation.data()),
    Some(Payload::Transaction(transaction)) => match transaction.essence() {
      Essence::Regular(essence) => match essence.payload() {
        Some(Payload::Indexation(payload)) => parse_data(message_id, payload.data()),
        _ => None,
      },
    },
    _ => None,
  }
}

fn parse_data<T: FromJson + TangleRef>(message_id: MessageId, data: &[u8]) -> Option<T> {
  let mut resource: T = T::from_json_slice(data).ok()?;

  resource.set_message_id(message_id);

  Some(resource)
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
    IotaDocument::try_from_message(self, did)
  }

  fn try_extract_diff(&self, did: &IotaDID) -> Option<DocumentDiff> {
    DocumentDiff::try_from_message(self, did)
  }
}

pub trait TryFromMessage: Sized {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self>;
}

impl TryFromMessage for IotaDocument {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self> {
    parse_message(message, did)
  }
}

impl TryFromMessage for DocumentDiff {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self> {
    parse_message(message, did)
  }
}
