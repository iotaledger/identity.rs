// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use iota_client::bee_message::payload::Payload;
use iota_client::bee_message::Message;
use iota_client::bee_message::MessageId;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::TangleRef;

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
    MessageId::null().eq(self)
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
