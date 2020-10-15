use identity_core::{
    common::{Object, Timestamp},
    did::{DIDDocument, DID},
    diff::Diff as _,
};
use iota::{
    client::{FindTransactionsResponse, GetTrytesResponse},
    transaction::{
        bundled::{Address, BundledTransaction, BundledTransactionField as _},
        Vertex as _,
    },
};
use serde_json::from_str;

use std::collections::BTreeMap;

use crate::{
    did::{create_address, method_id, DIDDiff},
    error::{DocumentError, Error, Result, TransactionError},
    network::{Network, NodeList},
    types::{TangleDiff, TangleDoc},
    utils::{encode_trits, trytes_to_utf8, txn_hash_trytes},
};

type Bundles = BTreeMap<String, Vec<BundledTransaction>>;
type Content = BTreeMap<String, String>;

#[derive(Debug)]
pub struct TangleReader {
    client: iota::Client,
    network: Network,
}

impl TangleReader {
    pub fn new(nodelist: &NodeList) -> Result<Self> {
        let mut builder: iota::ClientBuilder = iota::ClientBuilder::new();

        for node in nodelist.nodes() {
            builder = builder.node(node)?;
        }

        builder = builder.network(nodelist.network().into());

        Ok(Self {
            client: builder.build()?,
            network: nodelist.network(),
        })
    }

    pub async fn fetch(&self, did: &DID) -> Result<Content> {
        let address: Address = create_address(&did)?;

        let response: FindTransactionsResponse = self.client.find_transactions().addresses(&[address]).send().await?;

        let content: GetTrytesResponse = self.client.get_trytes(&response.hashes).await?;

        if content.trytes.is_empty() {
            return Err(Error::InvalidTransaction(TransactionError::MissingTrytes));
        }

        let bundles: Bundles = Self::index_bundles(content.trytes);

        if bundles.is_empty() {
            return Err(Error::InvalidTransaction(TransactionError::MissingBundle));
        }

        let content: Content = Self::index_content(bundles)?;

        if content.is_empty() {
            return Err(Error::InvalidTransaction(TransactionError::MissingContent));
        }

        Ok(content)
    }

    pub async fn fetch_latest(&self, did: &DID) -> Result<(TangleDoc, Object)> {
        let messages: Content = self.fetch(did).await?;

        let mut documents: Vec<TangleDoc> = Self::extract_documents(did, &messages)?;

        if documents.is_empty() {
            return Err(Error::InvalidDocument(DocumentError::MissingPayload));
        }

        let diffs: Vec<TangleDiff> = Self::extract_diffs(did, &messages)?;

        // We checked above if the `documents` vec was empty - this should not fail.
        let mut latest: TangleDoc = documents.remove(0);
        let mut metadata: Object = Object::new();

        for (index, diff) in diffs.into_iter().enumerate() {
            let updated: &Timestamp = if let Some(updated) = latest.data.updated.as_ref() {
                updated
            } else {
                return Err(Error::InvalidDocument(DocumentError::MissingUpdated));
            };

            if diff.data.proof.created > *updated {
                latest.data = latest.data.merge(DIDDocument::get_diff_from_str(&diff.data.diff)?)?;
                metadata.insert(format!("hash:diff:{}", index), diff.hash.into());
            }
        }

        metadata.insert("hash:doc".into(), latest.hash.as_str().into());

        Ok((latest, metadata))
    }

    pub fn extract_documents<'a>(did: &DID, content: &'a Content) -> Result<Vec<TangleDoc>> {
        let mid: &str = method_id(did)?;

        let mut documents: Vec<TangleDoc> = content
            .iter()
            .filter_map(|(hash, payload)| {
                from_str::<DIDDocument>(payload)
                    .ok()
                    .filter(|document| matches!(method_id(document.did()), Ok(id) if id == mid))
                    .map(|data| TangleDoc {
                        data,
                        hash: hash.into(),
                    })
            })
            .collect();

        // Sort documents by updated timestamp in descending order
        documents.sort_by(|a, b| b.data.updated.cmp(&a.data.updated));

        Ok(documents)
    }

    pub fn extract_diffs<'a>(did: &DID, content: &'a Content) -> Result<Vec<TangleDiff>> {
        let mid: &str = method_id(did)?;

        let mut diffs: Vec<TangleDiff> = content
            .iter()
            .filter_map(|(hash, payload)| {
                from_str::<DIDDiff>(payload)
                    .ok()
                    .filter(|diff| matches!(method_id(&diff.id), Ok(id) if id == mid))
                    .map(|data| TangleDiff {
                        data,
                        hash: hash.into(),
                    })
            })
            .collect();

        // Sort diffs by timestamp in ascending order
        diffs.sort_by(|a, b| a.data.proof.created.cmp(&b.data.proof.created));

        Ok(diffs)
    }

    fn index_bundles(txns: Vec<BundledTransaction>) -> Bundles {
        let mut bundles: Bundles = BTreeMap::new();
        let mut overflow: BTreeMap<String, BundledTransaction> = BTreeMap::new();

        for txn in txns {
            // Distinguish between tail transactions, because the content can be
            // changed at reattachments.
            if txn.is_tail() {
                bundles.insert(txn_hash_trytes(&txn), vec![txn]);
            } else {
                overflow.insert(txn_hash_trytes(&txn), txn);
            }
        }

        for txns in bundles.values_mut() {
            for index in 0..Self::bundle_idx(txns) {
                let hash: String = encode_trits(txns[index].trunk().to_inner());

                if let Some(trunk) = overflow.get(&hash) {
                    txns.push(trunk.clone());
                } else {
                    println!("[+] Missing Trunk Transaction: {}", hash);
                }
            }
        }

        // Remove incomplete bundles
        bundles
            .into_iter()
            .filter(|(_, txns)| txns.len() == Self::bundle_idx(&txns) + 1)
            .collect()
    }

    fn index_content(bundles: Bundles) -> Result<Content> {
        bundles
            .into_iter()
            .map(|(hash, txns)| {
                let trytes: String = txns.iter().map(|txn| encode_trits(txn.payload().to_inner())).collect();

                trytes_to_utf8(&trytes).map(|trytes| (hash, trytes))
            })
            .collect()
    }

    fn bundle_idx(txns: &[BundledTransaction]) -> usize {
        txns.get(0).map(|txn| *txn.last_index().to_inner()).expect("infallible")
    }
}
