use core::{cmp::Ordering, slice::from_ref};
use iota::{
    client::{FindTransactionsResponse, GetTrytesResponse},
    ternary::{Trits, T1B1},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField as _},
};

use crate::{
    client::{Client, TangleMessage, TransactionPrinter},
    error::{Error, Result},
    utils::encode_trits,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReadTransactionsResponse {
    pub address: Address,
    pub messages: Vec<TangleMessage>,
}

#[derive(Debug)]
pub struct ReadTransactionsRequest<'a> {
    pub(crate) client: &'a Client,
    pub(crate) address: Address,
    pub(crate) allow_empty: bool,
}

impl<'a> ReadTransactionsRequest<'a> {
    pub const fn new(client: &'a Client, address: Address) -> Self {
        Self {
            client,
            address,
            allow_empty: true,
        }
    }

    pub fn allow_empty(mut self, value: bool) -> Self {
        self.allow_empty = value;
        self
    }

    pub async fn send(self) -> Result<ReadTransactionsResponse> {
        trace!(
            "Find Transactions at Address: {}",
            encode_trits(self.address.to_inner())
        );

        // Fetch all transaction hashes containing the tangle address.
        let response: FindTransactionsResponse = self
            .client
            .client
            .find_transactions()
            .addresses(from_ref(&self.address))
            .send()
            .await?;

        trace!(
            "FindTransactions Response: {:?}",
            response
                .hashes
                .iter()
                .map(|hash| encode_trits(hash))
                .collect::<Vec<_>>()
        );

        if response.hashes.is_empty() {
            if self.allow_empty {
                return Ok(ReadTransactionsResponse {
                    address: self.address,
                    messages: Vec::new(),
                });
            } else {
                return Err(Error::InvalidTransactionHashes);
            }
        }

        // Fetch the content of all transactions.
        let content: GetTrytesResponse = self.client.client.get_trytes(&response.hashes).await?;

        trace!(
            "GetTrytes Response: {:?}",
            content.trytes.iter().map(TransactionPrinter::full).collect::<Vec<_>>()
        );

        if content.trytes.is_empty() {
            return Err(Error::InvalidTransactionTrytes);
        }

        // Re-build the fragmented messages stored in the bundle.
        let messages: Vec<TangleMessage> = bundles_from_trytes(content.trytes)
            .into_iter()
            .map(TangleMessage::try_from_bundle)
            .collect::<Result<_>>()?;

        trace!("Tangle Messages: {:?}", messages);

        Ok(ReadTransactionsResponse {
            address: self.address,
            messages,
        })
    }
}

fn bundles_from_trytes(mut transactions: Vec<BundledTransaction>) -> Vec<Vec<BundledTransaction>> {
    transactions.sort_by(|a, b| {
        // TODO: impl Ord for Address, Tag, Hash
        cmp_trits(a.address().to_inner(), b.address().to_inner())
            .then(cmp_trits(a.tag().to_inner(), b.tag().to_inner()))
            // different messages may have the same bundle hash!
            .then(cmp_trits(a.bundle().to_inner(), b.bundle().to_inner()))
            // reverse order of transactions will be extracted from back with `pop`
            .then(a.index().to_inner().cmp(b.index().to_inner()).reverse())
    });

    let mut bundles: Vec<Vec<BundledTransaction>> = Vec::new();

    if let Some(root) = transactions.pop() {
        let mut bundle: Vec<BundledTransaction> = vec![root];

        loop {
            if let Some(transaction) = transactions.pop() {
                if cmp_transaction(&bundle[0], &transaction) {
                    bundle.push(transaction);
                } else {
                    bundles.push(bundle);
                    bundle = vec![transaction];
                }
            } else {
                bundles.push(bundle);
                break;
            }
        }
    }

    // TODO: Check the bundles
    bundles
}

fn cmp_trits(a: &Trits<T1B1>, b: &Trits<T1B1>) -> Ordering {
    a.iter().cmp(b.iter())
}

fn cmp_transaction(a: &BundledTransaction, b: &BundledTransaction) -> bool {
    a.address() == b.address() && a.tag() == b.tag() && a.bundle() == b.bundle()
}
