// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_did::DID;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::diff::DiffMessage;
use identity_iota_core_legacy::tangle::Message;
use identity_iota_core_legacy::tangle::MessageId;
use iota_client::bee_message::payload::transaction::Essence;
use iota_client::bee_message::payload::Payload;

use crate::document::ResolvedIotaDocument;
use crate::error::Result;
use crate::tangle::message::compression_brotli;
use crate::tangle::DIDMessageEncoding;
use crate::tangle::DIDMessageVersion;
use crate::tangle::TangleRef;

/// Magic bytes used to mark DID messages.
const DID_MESSAGE_MARKER: &[u8] = b"DID";

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
  // Check version.
  let version: DIDMessageVersion = DIDMessageVersion::try_from(*data.first()?).ok()?;
  if version != DIDMessageVersion::V1 {
    return None;
  }

  // Check marker.
  let marker: &[u8] = data.get(1..4)?;
  if marker != DID_MESSAGE_MARKER {
    return None;
  }

  // Decode data.
  let encoding: DIDMessageEncoding = DIDMessageEncoding::try_from(*data.get(4)?).ok()?;
  let inner: &[u8] = data.get(5..)?;
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
    DIDMessageEncoding::JsonBrotli => compression_brotli::compress_brotli(data.to_json()?)?,
  };

  // Prepend flags.
  let encoded_message_data_with_flags =
    add_flags_to_message(encoded_message_data, DIDMessageVersion::CURRENT, encoding);
  Ok(encoded_message_data_with_flags)
}

/// Prepends the message flags and marker magic bytes to the data in the following order:
/// `[version, marker, encoding, data]`.
fn add_flags_to_message(
  mut data: Vec<u8>,
  message_version: DIDMessageVersion,
  encoding: DIDMessageEncoding,
) -> Vec<u8> {
  let mut buffer: Vec<u8> = Vec::with_capacity(1 + DID_MESSAGE_MARKER.len() + 1 + data.len());
  buffer.push(message_version as u8);
  buffer.extend_from_slice(DID_MESSAGE_MARKER);
  buffer.push(encoding as u8);
  buffer.append(&mut data);
  buffer
}

pub trait MessageExt {
  fn try_extract_document(&self, did: &IotaDID) -> Option<ResolvedIotaDocument>;

  fn try_extract_diff(&self, did: &IotaDID) -> Option<DiffMessage>;
}

impl MessageExt for Message {
  fn try_extract_document(&self, did: &IotaDID) -> Option<ResolvedIotaDocument> {
    ResolvedIotaDocument::try_from_message(self, did)
  }

  fn try_extract_diff(&self, did: &IotaDID) -> Option<DiffMessage> {
    DiffMessage::try_from_message(self, did)
  }
}

pub trait TryFromMessage: Sized {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self>;
}

impl TryFromMessage for ResolvedIotaDocument {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self> {
    parse_message(message, did)
  }
}

impl TryFromMessage for DiffMessage {
  fn try_from_message(message: &Message, did: &IotaDID) -> Option<Self> {
    parse_message(message, did)
  }
}

#[cfg(test)]
mod test {
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::service::ServiceBuilder;
  use identity_did::service::ServiceEndpoint;
  use identity_did::verification::MethodScope;
  use identity_iota_core_legacy::document::IotaDocument;
  use identity_iota_core_legacy::document::IotaVerificationMethod;
  use identity_iota_core_legacy::tangle::MessageId;

  use crate::document::ResolvedIotaDocument;
  use crate::tangle::message::message_encoding::DIDMessageEncoding;

  use super::*;

  #[test]
  fn test_pack_did_message() {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(
        keypair.private(),
        document.default_signing_method().unwrap().id().clone(),
      )
      .unwrap();

    for encoding in [DIDMessageEncoding::Json, DIDMessageEncoding::JsonBrotli] {
      let encoded: Vec<u8> = pack_did_message(&document, encoding).unwrap();
      assert_eq!(encoded[0], DIDMessageVersion::CURRENT as u8);
      assert_eq!(&encoded[1..4], DID_MESSAGE_MARKER);
      assert_eq!(encoded[4], encoding as u8);

      let decoded: ResolvedIotaDocument = parse_data(MessageId::null(), &encoded).unwrap();
      assert_eq!(decoded.document, document);
    }
  }

  #[test]
  fn test_pack_did_message_diff() {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let mut doc1: IotaDocument = IotaDocument::new(&keypair).unwrap();
    doc1
      .sign_self(keypair.private(), doc1.default_signing_method().unwrap().id().clone())
      .unwrap();

    let mut doc2: IotaDocument = doc1.clone();
    assert!(doc2.insert_service(
      ServiceBuilder::default()
        .id(doc1.id().to_url().join("#linked-domain").unwrap())
        .service_endpoint(ServiceEndpoint::One(Url::parse("https://example.com/").unwrap()))
        .type_("LinkedDomains")
        .build()
        .unwrap()
    ));
    doc2
      .insert_method(
        IotaVerificationMethod::new(doc1.id().clone(), KeyType::Ed25519, keypair.public(), "key-1").unwrap(),
        MethodScope::authentication(),
      )
      .unwrap();
    let diff: DiffMessage = doc1
      .diff(
        &doc2,
        MessageId::new([1; 32]),
        keypair.private(),
        doc1.default_signing_method().unwrap().id(),
      )
      .unwrap();

    for encoding in [DIDMessageEncoding::Json, DIDMessageEncoding::JsonBrotli] {
      let encoded: Vec<u8> = pack_did_message(&diff, encoding).unwrap();
      assert_eq!(encoded[0], DIDMessageVersion::CURRENT as u8);
      assert_eq!(&encoded[1..4], DID_MESSAGE_MARKER);
      assert_eq!(encoded[4], encoding as u8);

      let decoded: DiffMessage = parse_data(MessageId::null(), &encoded).unwrap();
      assert_eq!(decoded, diff);
    }
  }
}
