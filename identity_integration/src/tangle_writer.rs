use anyhow::Result;
use identity_core::document::DIDDocument;
pub use iota::client::builder::Network as iota_network;
use iota::{
    client::Transfer,
    crypto::ternary::{
        sponge::{CurlP81, Sponge},
        Hash,
    },
    ternary::{T1B1Buf, TritBuf, TryteBuf},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField, Tag},
};
use iota_conversion::Trinary;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DIDMessage {
    // signature: Signature,?
    pub payload: Payload,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Payload {
    DIDDocument(DIDDocument),
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
}

impl TangleWriter {
    /// Publishes DID document to the Tangle
    pub async fn publish_document(&self, did_message: &DIDMessage) -> Result<Hash> {
        // Get address from did_document?
        // Diff chain address in did_document?
        // Is it possible to get the address from the did_document after an auth change?
        let serialzed_did_message = serde_json::to_string(&did_message)?;
        let transfers = vec![Transfer {
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
        }];

        // Create a client instance
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;

        // Send the transaction
        let bundle = iota.send(None).transfers(transfers).send().await?;

        let mut curl = CurlP81::new();
        let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
        bundle[0].into_trits_allocated(&mut trits);
        Ok(Hash::from_inner_unchecked(curl.digest(&trits).unwrap()))
    }
    /// Promotes a transaction to get it faster confirmed
    pub async fn promote(&self, tail_transaction: Hash) -> Result<String, Box<dyn std::error::Error>> {
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;
        let transfers = vec![Transfer {
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
        }];

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

    /// Returns confirmation status
    pub async fn is_confirmed(&self, tail_transaction: Hash) -> Result<bool, Box<dyn std::error::Error>> {
        let iota = iota::ClientBuilder::new()
            .nodes(&self.nodes)?
            .network(self.network.clone())
            .build()?;

        // Get confirmation status
        let inclusion_states = iota
            .get_inclusion_states()
            .transactions(&[tail_transaction])
            .send()
            .await?;
        Ok(inclusion_states.states.contains(&true))
    }
}
