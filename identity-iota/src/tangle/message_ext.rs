// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::payload::transaction::Essence;
use iota_client::bee_message::payload::Payload;
use iota_client::bee_message::Message;
use iota_client::bee_message::MessageId;
use iota_client::bee_message::MESSAGE_ID_LENGTH;

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_did::did::DID;

use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::compression_brotli;
use crate::tangle::message_encoding::DIDMessageEncoding;
use crate::tangle::message_version::DIDMessageVersion;
use crate::tangle::message_version::CURRENT_MESSAGE_VERSION;
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

// TODO: allow this to return errors?
fn parse_data<T: FromJson + TangleRef>(message_id: MessageId, data: &[u8]) -> Option<T> {
  let version: DIDMessageVersion = DIDMessageVersion::try_from(*data.get(0)?).ok()?;
  if version != DIDMessageVersion::V1 {
    return None;
  }

  let encoding: DIDMessageEncoding = DIDMessageEncoding::try_from(*data.get(1)?).ok()?;
  let inner: &[u8] = data.get(2..)?;
  let mut resource: T = match encoding {
    DIDMessageEncoding::Json => T::from_json_slice(inner),
    DIDMessageEncoding::JsonBrotli => T::from_json_slice(&compression_brotli::decompress_brotli(inner).ok()?),
  }
  .ok()?;
  resource.set_message_id(message_id);
  Some(resource)
}

/// Encodes the data and prepends the current message version and encoding flags to it.
pub(crate) fn pack_did_message<T: ToJson>(data: &T, encoding: DIDMessageEncoding) -> Result<Vec<u8>> {
  // Encode data.
  let encoded_message_data: Vec<u8> = match encoding {
    DIDMessageEncoding::Json => data.to_json_vec()?,
    DIDMessageEncoding::JsonBrotli => compression_brotli::compress_brotli(&data.to_json()?)?,
  };

  // Prepend flags.
  let encoded_message_data_with_flags = add_flags_to_message(encoded_message_data, CURRENT_MESSAGE_VERSION, encoding);

  Ok(encoded_message_data_with_flags)
}

/// Prepends the message version and encoding flags to message data.
fn add_flags_to_message(
  mut data: Vec<u8>,
  message_version: DIDMessageVersion,
  encoding: DIDMessageEncoding,
) -> Vec<u8> {
  let message_version_flag = message_version as u8;
  let encoding_flag = encoding as u8;

  data.splice(0..0, [message_version_flag, encoding_flag]);
  data
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

#[cfg(test)]
mod test {
  use identity_core::crypto::KeyPair;

  use crate::did::IotaDocument;
  use crate::tangle::message_encoding::DIDMessageEncoding;
  use crate::tangle::message_version::CURRENT_MESSAGE_VERSION;
  use crate::tangle::MessageId;

  use super::*;

  #[test]
  fn test_pack_did_message_round_trip() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(keypair.private(), &document.default_signing_method().unwrap().id())
      .unwrap();

    for encoding in [DIDMessageEncoding::Json, DIDMessageEncoding::JsonBrotli] {
      let encoded: Vec<u8> = pack_did_message(&document, encoding).unwrap();
      assert_eq!(encoded[0], CURRENT_MESSAGE_VERSION as u8);
      assert_eq!(encoded[1], encoding as u8);

      let decoded: IotaDocument = parse_data(MessageId::null(), &encoded).unwrap();
      assert_eq!(decoded, document);
    }
  }
}
