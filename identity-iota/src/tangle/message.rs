// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::convert::FromJson;
use iota::crypto::ternary::Hash;
use iota::ternary::T1B1Buf;
use iota::ternary::TritBuf;
use iota::transaction::bundled::BundledTransaction;
use iota::transaction::bundled::BundledTransactionField;
use iota::transaction::bundled::Timestamp;

use crate::did::Document;
use crate::did::DocumentDiff;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::MessageId;
use crate::tangle::TangleRef;
use crate::utils::encode_trits;
use crate::utils::trytes_to_utf8;
use crate::utils::txn_hash;

macro_rules! try_extract {
  ($ty:ty, $this:expr, $did:expr) => {{
    let mut resource: $ty = $this
      .message_utf8()
      .ok()
      .and_then(|json| <$ty>::from_json(&json).ok())?;

    if $did.authority() != resource.id().authority() {
      return None;
    }

    TangleRef::set_message_id(&mut resource, $this.message_id());

    Some(resource)
  }};
}

pub struct Message {
  pub address: String,
  pub message: TritBuf<T1B1Buf>,
  pub tail_hash: Hash,
  pub timestamp: Timestamp,
}

impl Message {
  pub fn try_from_bundle(bundle: Vec<BundledTransaction>) -> Result<Self> {
    let message: TritBuf<T1B1Buf> = bundle
      .iter()
      .flat_map(|transaction| transaction.payload().to_inner().iter())
      .collect();

    let tail: &BundledTransaction = bundle
      .iter()
      .find(|transaction| transaction.is_tail())
      .ok_or(Error::InvalidBundleTail)?;

    Ok(Self {
      message,
      tail_hash: txn_hash(&tail),
      timestamp: tail.timestamp().clone(),
      address: encode_trits(tail.address().to_inner()),
    })
  }

  /// Returns the contents of the message as a tryte-encoded string.
  pub fn message_str(&self) -> String {
    encode_trits(&self.message)
  }

  /// Returns the contents of the message as a utf8-encoded string.
  pub fn message_utf8(&self) -> Result<String> {
    trytes_to_utf8(&self.message_str())
  }

  /// Returns the `MessageId` identifying the Tangle message.
  pub fn message_id(&self) -> MessageId {
    MessageId::new(encode_trits(&self.tail_hash))
  }

  pub fn try_extract_document(&self, did: &DID) -> Option<Document> {
    try_extract!(Document, self, did)
  }

  pub fn try_extract_diff(&self, did: &DID) -> Option<DocumentDiff> {
    try_extract!(DocumentDiff, self, did)
  }
}

impl Debug for Message {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Message")
      .field("address", &self.address)
      .field("message", &self.message_str())
      .field("timestamp", &self.timestamp)
      .finish()
  }
}
