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

        resource.set_message_id($this.transaction_hash());

        Some(resource)
    }};
}

#[derive(Clone, PartialEq)]
pub struct TangleMessage {
    pub address: String,
    pub message: TritBuf<T1B1Buf>,
    pub tail_hash: Hash,
    pub timestamp: Timestamp,
}

impl TangleMessage {
    pub fn try_from_bundle(bundle: Vec<BundledTransaction>) -> Result<Self> {
        let message: TritBuf<T1B1Buf> = bundle
            .iter()
            .flat_map(|transaction| transaction.payload().to_inner().iter())
            .collect();

        let tail: &BundledTransaction = bundle.first().ok_or(Error::InvalidTransactionBundle)?;

        debug_assert!(tail.is_tail());

        Ok(Self {
            message,
            tail_hash: txn_hash(&tail),
            timestamp: tail.timestamp().clone(),
            address: encode_trits(tail.address().to_inner()),
        })
    }

    pub fn message_str(&self) -> String {
        encode_trits(&self.message)
    }

    pub fn message_utf8(&self) -> Result<String> {
        trytes_to_utf8(&self.message_str())
    }

    pub fn transaction_hash(&self) -> String {
        encode_trits(&self.tail_hash)
    }

    pub fn try_extract_document(&self, did: &IotaDID) -> Option<IotaDocument> {
        try_extract!(IotaDocument, self, did)
    }

    pub fn try_extract_diff(&self, did: &IotaDID) -> Option<DocumentDiff> {
        try_extract!(DocumentDiff, self, did)
    }
}

impl Debug for TangleMessage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("TangleMessage")
            .field("address", &self.address)
            .field("message", &self.message_str())
            .field("timestamp", &self.timestamp)
            .finish()
    }
}
