// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use iota::Message;
use iota::MessageId;
use iota::Payload;
use iota::MESSAGE_ID_LENGTH;

use crate::did::Document;
use crate::did::DocumentDiff;
use crate::did::DID;
use crate::error::Result;
use crate::tangle::TangleRef;

// TODO: Use MessageId when it has a const ctor
static NULL: &[u8; MESSAGE_ID_LENGTH] = &[0; MESSAGE_ID_LENGTH];

macro_rules! try_extract {
  ($ty:ty, $this:expr, $did:expr) => {{
    if let Some(Payload::Indexation(payload)) = $this.payload() {
      let mut resource: $ty = <$ty>::from_json_slice(payload.data()).ok()?;

      if $did.authority() != resource.id().authority() {
        return None;
      }

      TangleRef::set_message_id(&mut resource, $this.id().0);

      Some(resource)
    } else {
      None
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
  fn try_extract_document(&self, did: &DID) -> Option<Document>;

  fn try_extract_diff(&self, did: &DID) -> Option<DocumentDiff>;
}

impl MessageExt for Message {
  fn try_extract_document(&self, did: &DID) -> Option<Document> {
    try_extract!(Document, self, did)
  }

  fn try_extract_diff(&self, did: &DID) -> Option<DocumentDiff> {
    try_extract!(DocumentDiff, self, did)
  }
}
