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
    pub async fn fetch(&self, address: &str) -> crate::Result<Vec<String>> {
        let iota = iota::ClientBuilder::new().nodes(&self.nodes)?.build()?;

        let address = Address::from_inner_unchecked(TryteBuf::try_from_str(address)?.as_trits().encode());

        let response = iota.find_transactions().addresses(&[address]).send().await?;
        let txs = iota.get_trytes(&response.hashes).await?;
        if txs.trytes.is_empty() {
            return Err(crate::Error::TransactionsNotFound);
        }
        // Order transactions to bundles
        let mut bundles = sort_txs_to_bundles(txs.trytes)?;

        if bundles.keys().len() == 0 {
            return Err(crate::Error::MissingTailTransaction);
        }

        // Convert messages to ascii
        let mut messages = Vec::new();
        for bundle in bundles.values_mut() {
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
                messages.push(message);
            }
        }

        Ok(messages)
    }
}

/// Sorts transactions to bundles by tail transactions
fn sort_txs_to_bundles(trytes: Vec<BundledTransaction>) -> crate::Result<HashMap<Hash, Vec<BundledTransaction>>> {
    let mut bundles = HashMap::new();
    let mut transactions = HashMap::new();
    for tx in trytes {
        let mut curl = CurlP81::new();
        let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
        tx.into_trits_allocated(&mut trits);
        let tx_hash = Hash::from_inner_unchecked(curl.digest(&trits)?);
        if tx.index() == &Index::from_inner_unchecked(0) {
            // Distinguish between tail transactions, because the content can be changed at reattachments
            bundles.insert(tx_hash, vec![tx]);
        } else {
            transactions.insert(tx_hash, tx);
        }
    }
    for bundle in bundles.values_mut() {
        for index in 0..*bundle[0].last_index().to_inner() {
            if let Some(trunk_transaction) = transactions.get(&Hash::from_inner_unchecked(
                TritBuf::from_i8s(bundle[index].trunk().to_inner().as_i8_slice())
                    .expect("Can't get TritBuf from i8_slice"),
            )) {
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
    Ok(bundles)
}
