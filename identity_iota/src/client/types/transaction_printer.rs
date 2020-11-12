use core::{
    fmt::{Debug, Display, Formatter, Result},
    marker::PhantomData,
};
use iota::transaction::bundled::{BundledTransaction, BundledTransactionField as _};

use crate::utils::{encode_trits, txn_hash_trytes};

pub enum TransactionFmtFull {}

pub enum TransactionFmtMini {}

pub enum TransactionFmtHash {}

pub struct TransactionPrinter<'a, T = TransactionFmtFull>(&'a BundledTransaction, PhantomData<T>);

impl<'a> TransactionPrinter<'a, TransactionFmtFull> {
    pub fn full(transaction: &'a BundledTransaction) -> Self {
        Self(transaction, PhantomData)
    }
}

impl<'a> TransactionPrinter<'a, TransactionFmtMini> {
    pub fn mini(transaction: &'a BundledTransaction) -> Self {
        Self(transaction, PhantomData)
    }
}

impl<'a> TransactionPrinter<'a, TransactionFmtHash> {
    pub fn hash(transaction: &'a BundledTransaction) -> Self {
        Self(transaction, PhantomData)
    }
}

impl Debug for TransactionPrinter<'_, TransactionFmtFull> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("BundledTransaction")
            .field("hash", &txn_hash_trytes(self.0))
            .field("address", &encode_trits(self.0.address().to_inner()))
            .field("value", &self.0.value().to_inner())
            .field("index", &self.0.index().to_inner())
            .field("last_index", &self.0.last_index().to_inner())
            .field("bundle", &encode_trits(self.0.bundle()))
            .field("tag", &encode_trits(self.0.tag().to_inner()))
            .field("attachment_ts", &self.0.attachment_ts().to_inner())
            .field("attachment_lbts", &self.0.attachment_lbts().to_inner())
            .field("attachment_ubts", &self.0.attachment_ubts().to_inner())
            .field("nonce", &encode_trits(self.0.nonce().to_inner()))
            .finish()
    }
}

impl Debug for TransactionPrinter<'_, TransactionFmtMini> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("BundledTransaction")
            .field("hash", &txn_hash_trytes(self.0))
            .field("address", &encode_trits(self.0.address().to_inner()))
            .finish()
    }
}

impl Debug for TransactionPrinter<'_, TransactionFmtHash> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", txn_hash_trytes(self.0))
    }
}

impl Display for TransactionPrinter<'_, TransactionFmtHash> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", txn_hash_trytes(self.0))
    }
}
