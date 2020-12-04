use core::fmt::{Debug, Formatter, Result as FmtResult};
use iota::{
    crypto::ternary::Hash,
    ternary::{T1B1Buf, TritBuf},
    transaction::bundled::{BundledTransaction, BundledTransactionField as _, Timestamp},
};

use crate::{
    error::{Error, Result},
    utils::{encode_trits, trytes_to_utf8, txn_hash},
};

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
