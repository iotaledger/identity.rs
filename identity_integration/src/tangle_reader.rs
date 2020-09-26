use iota::{
    crypto::ternary::{
        sponge::{CurlP81, Sponge},
        Hash,
    },
    ternary::{T1B1Buf, TritBuf, TryteBuf},
    transaction::{
        bundled::{Address, BundledTransaction, BundledTransactionField, Index},
        Vertex,
    },
};
use iota_conversion::{trytes_converter, Trinary};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TangleReader {
    pub nodes: Vec<&'static str>,
}

impl TangleReader {
    pub fn new(nodes: Vec<&'static str>) -> Self {
        Self { nodes }
    }
    /// Returns all messages from an address
    pub async fn fetch(&self, address: &str) -> crate::Result<HashMap<String, String>> {
        let iota = iota::ClientBuilder::new().nodes(&self.nodes)?.build().await?;

        let address = Address::from_inner_unchecked(TryteBuf::try_from_str(address)?.as_trits().encode());

        let response = iota.find_transactions().addresses(&[address]).send().await?;
        let txs = iota.get_trytes(&response.hashes).await?;
        if txs.trytes.is_empty() {
            return Err(crate::Error::TransactionsNotFound);
        }
        // Order transactions to bundles
        let bundles = sort_txs_to_bundles(txs.trytes)?;

        if bundles.keys().len() == 0 {
            return Err(crate::Error::MissingTransactions);
        }

        // Convert messages to ascii
        let mut messages = HashMap::new();
        for (txhash, bundle) in bundles.iter() {
            let trytes_coll: Vec<String> = bundle
                .iter()
                .map(|t| {
                    t.payload()
                        .to_inner()
                        .as_i8_slice()
                        .trytes()
                        .expect("Couldn't get Trytes")
                })
                .collect();

            if let Ok(message) = trytes_converter::to_string(&trytes_coll.concat()) {
                messages.insert(txhash.clone(), message);
            }
        }

        Ok(messages)
    }
}

/// Sorts transactions to bundles by tail transactions
fn sort_txs_to_bundles(trytes: Vec<BundledTransaction>) -> crate::Result<HashMap<String, Vec<BundledTransaction>>> {
    let mut bundles = HashMap::new();
    let mut transactions = HashMap::new();
    for tx in trytes {
        let mut curl = CurlP81::new();
        let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
        tx.into_trits_allocated(&mut trits);
        let tx_hash = Hash::from_inner_unchecked(curl.digest(&trits)?)
            .as_trits()
            .as_i8_slice()
            .trytes()
            .expect("Couldn't get Trytes");
        if tx.index() == &Index::from_inner_unchecked(0) {
            // Distinguish between tail transactions, because the content can be changed at reattachments
            bundles.insert(tx_hash, vec![tx]);
        } else {
            transactions.insert(tx_hash, tx);
        }
    }
    for bundle in bundles.values_mut() {
        for index in 0..*bundle[0].last_index().to_inner() {
            if let Some(trunk_transaction) = transactions.get(
                &bundle[index]
                    .trunk()
                    .to_inner()
                    .as_i8_slice()
                    .trytes()
                    .expect("Couldn't get Trytes"),
            ) {
                bundle.push(trunk_transaction.clone());
            }
            // Debug
            // else {
            //     println!(
            //         "Trunk transaction not found: https://comnet.thetangle.org/transaction/{}",
            //         bundle[index]
            //             .trunk()
            //             .to_inner()
            //             .as_i8_slice()
            //             .trytes()
            //             .expect("Couldn't get Trytes")
            //     );
            // }
        }
        // Debug check if all transactions are there
        // if bundle.len() != *bundle[0].last_index().to_inner() + 1 {
        //     println!(
        //         "Not all transactions for {} are known",
        //         bundle[0]
        //             .bundle()
        //             .to_inner()
        //             .as_i8_slice()
        //             .trytes()
        //             .expect("Couldn't get Trytes")
        //     );
        // }
    }
    // Remove incomplete bundles
    let complete_bundles = bundles
        .into_iter()
        .filter(|(_, bundle)| bundle.len() == *bundle[0].last_index().to_inner() + 1)
        .collect();
    Ok(complete_bundles)
}
