use core::fmt::{Debug, Formatter, Result as FmtResult};
use identity_core::convert::FromJson as _;
use iota::{
    crypto::ternary::Hash,
    ternary::{T1B1Buf, TritBuf},
    transaction::bundled::{BundledTransaction, BundledTransactionField as _, Timestamp},
};

use crate::{
    did::{DocumentDiff, IotaDID, IotaDocument},
    error::{Error, Result},
    tangle::{MessageId, TangleRef},
    utils::{encode_trits, trytes_to_utf8, txn_hash},
};

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

    pub fn try_extract_document(&self, did: &IotaDID) -> Option<IotaDocument> {
        try_extract!(IotaDocument, self, did)
    }

    pub fn try_extract_diff(&self, did: &IotaDID) -> Option<DocumentDiff> {
        try_extract!(DocumentDiff, self, did)
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("Message")
            .field("address", &self.address)
            .field("message", &self.message_str())
            .field("timestamp", &self.timestamp)
            .finish()
    }
}
