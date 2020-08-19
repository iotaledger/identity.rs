use anyhow::Result;
pub use iota::client::builder::Network as iota_network;
use iota::{
    client::Transfer,
    crypto::ternary::{
        sponge::{CurlP81, Sponge},
        Hash,
    },
    ternary::{T1B1Buf, TritBuf, TryteBuf},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField, Index, Tag},
};
use iota_conversion::Trinary;
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DIDMessage {
    // signature: Signature,?
    pub payload: Payload,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Payload {
    DIDDocument(String),
    DIDDocumentDifferences(String),
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct TangleWriter {
    pub nodes: Vec<&'static str>,
    pub network: iota::client::builder::Network,
    // Add the sent trytes here?
}

fn serialize_did_message(did_message: &DIDMessage) -> Result<String> {
    Ok(serde_json::to_string(&did_message)?)
}

impl TangleWriter {
    /// Publishes DID document to the Tangle
    pub async fn publish_document(&self, did_message: &DIDMessage) -> Result<Vec<BundledTransaction>> {
        // Get address from did_document?
        // Diff chain address in did_document?
        // Is it possible to get the address from the did_document after an auth change?
        let serialzed_did_message = serialize_did_message(did_message)?;
        let mut transfers = Vec::new();
        transfers.push(Transfer {
            address: Address::from_inner_unchecked(
                TryteBuf::try_from_str(&did_message.address)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ),
            value: 0,
            message: Some(serialzed_did_message),
            tag: Some(
                Tag::try_from_inner(
                    TryteBuf::try_from_str("DID999999999999999999999999")
                        .unwrap()
                        .as_trits()
                        .encode(),
                )
                .unwrap(),
            ),
        });

        // Create a client instance
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            // Investigate why this doesn't work, would be better than setting mwm
            .network(self.network.clone())
            .build()?;

        // Send the transaction
        let res = iota
            .send(None)
            .transfers(transfers)
            // .min_weight_magnitude(10)
            .send()
            .await?;

        // Wait with res until confirmed?
        Ok(res)
    }
    /// Promotes a transaction to get it faster confirmed
    pub async fn promote(&self, tail_transaction: Hash) -> Result<String, Box<dyn std::error::Error>> {
        let mut transfers = Vec::new();
        transfers.push(Transfer {
            address: Address::from_inner_unchecked(
                TryteBuf::try_from_str(&String::from(
                    "PROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDR",
                ))
                .unwrap()
                .as_trits()
                .encode(),
            ),
            value: 0,
            message: None,
            tag: None,
        });

        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;

        let prepared_transfers = iota.prepare_transfers(None).transfers(transfers).build().await?;
        let tips = iota.get_transactions_to_approve().depth(2).send().await?;
        let attached_trytes = iota
            .attach_to_tangle()
            .trunk_transaction(&tail_transaction)
            .branch_transaction(&tips.branch_transaction)
            .trytes(&[prepared_transfers[0].clone()])
            .send()
            .await?;

        iota.broadcast_transactions(&attached_trytes.trytes).await?;
        Ok(prepared_transfers[0]
            .bundle()
            .to_inner()
            .as_i8_slice()
            .trytes()
            .unwrap())
    }

    /// Reattaches a bundle in case the first attachment failed or didn't get confirmed
    pub async fn reattach(&self, trytes: Vec<BundledTransaction>) -> Result<String, Box<dyn std::error::Error>> {
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;
        let reattached = iota.send_trytes().trytes(trytes).depth(2).send().await?;
        Ok(reattached[0].bundle().to_inner().as_i8_slice().trytes().unwrap())
    }

    /// Returns confirmation status and latest tail tx that can be used to promote
    pub async fn is_confirmed(&self, bundle: Hash) -> Result<(Option<Hash>, bool), Box<dyn std::error::Error>> {
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;
        // We could store everything locally and rely on this information, then we don't need to get the transactions
        // In that case we wouldn't notice if someone else reattaches our bundle, but should be no problem because it
        // can get confirmed multiple times
        let response = iota.find_transactions().bundles(&[bundle]).send().await?;

        let trytes = iota.get_trytes(&response.hashes).await?;

        // Get all tail txs
        let tail_txs = trytes
            .trytes
            .iter()
            .filter(|tx| tx.index() == &Index::from_inner_unchecked(0))
            .collect::<Vec<&BundledTransaction>>();

        // Use local time to ignore txs with a timestamp in the future, because if there is one that is much time ahead,
        // it will always be selected
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut filtered_tail_txs = trytes
            .trytes
            .iter()
            // tx.get_timestamp() > time-140 to check if it's not too old already
            .filter(|tx| {
                tx.index() == &Index::from_inner_unchecked(0)
                    && tx.get_timestamp() < time
                    && tx.get_timestamp() > time - 140
            })
            .collect::<Vec<&BundledTransaction>>();

        // Sort txs based on attachment timestamp
        filtered_tail_txs.sort_by(|a, b| b.get_timestamp().cmp(&a.get_timestamp()));
        let tail_tx_for_promotion = match filtered_tail_txs.first() {
            Some(tx) => {
                let mut curl = CurlP81::new();
                let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
                tx.into_trits_allocated(&mut trits);
                Some(Hash::from_inner_unchecked(curl.digest(&trits).unwrap()))
            }
            None => None,
        };

        let tail_txs_hashes: Vec<Hash> = tail_txs
            .iter()
            .map(|tx| {
                let mut curl = CurlP81::new();
                let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
                tx.into_trits_allocated(&mut trits);
                Hash::from_inner_unchecked(curl.digest(&trits).unwrap())
            })
            .collect();

        // Get confirmation status
        let inclusion_states = iota
            .get_inclusion_states()
            .transactions(&tail_txs_hashes)
            .send()
            .await?;
        Ok((tail_tx_for_promotion, inclusion_states.states.contains(&true)))
    }
}
