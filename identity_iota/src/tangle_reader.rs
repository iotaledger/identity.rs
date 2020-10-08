use crate::{helpers::get_iota_address, tangle_writer::Differences};
use async_trait::async_trait;
pub use identity_core::did::IdentityReader;
use identity_core::did::{DIDDocument, DID};
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
use std::{collections::HashMap, str::FromStr};
#[derive(Clone)]
pub struct HashWithDocument {
    pub tailhash: String,
    pub document: DIDDocument,
}

pub struct HashWithDiff {
    pub tailhash: String,
    pub diff: Differences,
}

#[derive(Clone)]
pub struct IOTAReader {
    pub nodes: Vec<&'static str>,
    // iota: iota::Client,
}

#[async_trait]
impl IdentityReader for IOTAReader {
    type HashDocument = HashWithDocument;
    type HashDiff = HashWithDiff;
    type Error = crate::Error;
    // Fetch documents and diffs with a single API call
    async fn fetch(&self, did: &DID) -> crate::Result<(Option<Vec<Self::HashDocument>>, Option<Vec<Self::HashDiff>>)> {
        let address = get_iota_address(&did)?;

        let iota = iota::ClientBuilder::new().nodes(&self.nodes)?.build()?;

        let address = Address::from_inner_unchecked(TryteBuf::try_from_str(&address)?.as_trits().encode());

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

        let mut documents = Vec::new();
        let mut diffs = Vec::new();
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
            // Convert messages to ascii
            if let Ok(message) = trytes_converter::to_string(&trytes_coll.concat()) {
                // Remove invalid stuff (wrong signature/did) here or do we want to keep it at this point for debug
                // purposes?
                if let Ok(document) = DIDDocument::from_str(&message) {
                    documents.push(HashWithDocument {
                        tailhash: txhash.clone(),
                        document,
                    });
                }
                if let Ok(diff) = serde_json::from_str::<Differences>(&message) {
                    diffs.push(HashWithDiff {
                        tailhash: txhash.clone(),
                        diff,
                    });
                }
            }
        }
        match documents.is_empty() {
            true => match diffs.is_empty() {
                true => Ok((None, None)),
                false => Ok((None, Some(diffs))),
            },
            false => match diffs.is_empty() {
                true => Ok((Some(documents), None)),
                false => Ok((Some(documents), Some(diffs))),
            },
        }
    }
    /// Returns all documents ordered from an address
    async fn fetch_documents(&self, did: &DID) -> crate::Result<Option<Vec<Self::HashDocument>>> {
        let messages = self.fetch(did).await?;
        if let Some(documents) = messages.0 {
            let documents = order_documents(documents)?;
            Ok(Some(documents))
        } else {
            Ok(None)
        }
    }
    /// Returns all diffs ordered from an address
    async fn fetch_diffs(&self, did: &DID) -> crate::Result<Option<Vec<Self::HashDiff>>> {
        let messages = self.fetch(did).await?;
        if let Some(diffs) = messages.1 {
            let diffs = order_diffs(diffs)?;
            Ok(Some(diffs))
        } else {
            Ok(None)
        }
    }
}
impl IOTAReader {
    pub fn new(nodes: Vec<&'static str>) -> Self {
        Self { nodes }
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

/// Order documents: first element is latest
pub fn order_documents(mut documents: Vec<HashWithDocument>) -> crate::Result<Vec<HashWithDocument>> {
    // Maybe validate this inside fetch()
    // let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
    // let mut documents: Vec<HashWithDocument> = messages
    //     .iter()
    //     .filter_map(|(tailhash, msg)| {
    //         if let Ok(document) = DIDDocument::from_str(&msg) {
    //             if document
    //                 .derive_did()
    //                 .id_segments
    //                 .last()
    //                 .expect("Failed to get id_segment")
    //                 == iota_specific_idstring
    //             {
    //                 Some(HashWithDocument {
    //                     tailhash: tailhash.into(),
    //                     document,
    //                 })
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // if documents.is_empty() {
    //     return Ok(documents);
    // }
    documents.sort_by(|a, b| b.document.updated.cmp(&a.document.updated));
    Ok(documents)
}

/// Order diffs: first element is oldest
pub fn order_diffs(mut diffs: Vec<HashWithDiff>) -> crate::Result<Vec<HashWithDiff>> {
    // let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
    // let mut diffs: Vec<HashWithDiff> = messages
    //     .iter()
    //     .filter_map(|(tailhash, msg)| {
    //         if let Ok(diff) = serde_json::from_str::<Differences>(&msg) {
    //             if diff.did.id_segments.last().expect("Failed to get id_segment") == iota_specific_idstring {
    //                 Some(HashWithDiff {
    //                     tailhash: tailhash.into(),
    //                     diff,
    //                 })
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // if diffs.is_empty() {
    //     return Ok(diffs);
    // }
    diffs.sort_by(|a, b| a.diff.time.cmp(&b.diff.time));
    Ok(diffs)
}
