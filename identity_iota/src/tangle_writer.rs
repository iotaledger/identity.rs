use crate::{helpers::get_iota_address, Error};
use async_trait::async_trait;
pub use identity_core::did::IdentityWriter;
use identity_core::did::{DIDDocument, DID};
pub use iota::client::builder::Network;
use iota::{
    client::Transfer,
    crypto::ternary::{
        sponge::{CurlP81, Sponge},
        Hash,
    },
    ternary::{T1B1Buf, TritBuf, TryteBuf},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField},
};
use iota_conversion::Trinary;
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

#[async_trait]
impl IdentityWriter for IOTAWriter {
    type Diff = Differences;
    type Error = crate::Error;
    async fn send_doc(&self, did_document: &DIDDocument) -> crate::Result<Vec<u8>> {
        let tail_transaction = self.send(&Payload::DIDDocument(did_document.clone())).await?;
        Ok(tail_transaction.to_string().into_bytes())
    }
    async fn send_diff(&self, did_document_diff: &Self::Diff) -> crate::Result<Vec<u8>> {
        let tail_transaction = self
            .send(&Payload::DIDDocumentDifferences(did_document_diff.clone()))
            .await?;
        Ok(tail_transaction.to_string().into_bytes())
    }
}

pub struct IOTAWriter {
    iota: iota::Client,
    network: iota::client::builder::Network,
}

impl IOTAWriter {
    pub async fn new(nodes: Vec<&'static str>, network: iota::client::builder::Network) -> crate::Result<IOTAWriter> {
        let iota = iota::ClientBuilder::new()
            .nodes(&nodes)?
            .network(network.clone())
            .build()?;
        Ok(Self { iota, network })
    }
    pub async fn send(&self, did_document: &Payload) -> crate::Result<Hash> {
        let mut tail_transaction = self.publish_document(did_document).await?;
        thread::sleep(Duration::from_secs(5));
        let mut j: usize = 0;
        while !self.is_confirmed(tail_transaction).await? {
            j += 1;
            thread::sleep(Duration::from_secs(5));
            self.promote(tail_transaction).await?;
            // Send the document again if the previous transaction didn't get confirmed after 150 seconds
            if j % 30 == 0 {
                tail_transaction = self.publish_document(&did_document).await?;
            }
        }
        Ok(tail_transaction)
    }
    /// Publishes DID document or diff to the Tangle
    pub async fn publish_document(&self, did_document: &Payload) -> crate::Result<Hash> {
        let (did, document_string) = match did_document {
            Payload::DIDDocument(document) => (document.derive_did().clone(), serde_json::to_string(document)?),
            Payload::DIDDocumentDifferences(differences) => {
                (differences.did.clone(), serde_json::to_string(&differences)?)
            }
        };
        // Check if correct network selected
        check_network(did.id_segments.clone(), &self.network)?;

        // Where does the address for diff changes come from?
        let address = get_iota_address(&did)?;

        let transfers = vec![Transfer {
            address: Address::from_inner_unchecked(TryteBuf::try_from_str(&address)?.as_trits().encode()),
            value: 0,
            message: Some(document_string),
            tag: None,
        }];

        // Send the transaction
        let bundle = self.iota.send(None).transfers(transfers).send().await?;
        // Get the transaction hash
        let mut curl = CurlP81::new();
        let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
        bundle[0].into_trits_allocated(&mut trits);
        Ok(Hash::from_inner_unchecked(curl.digest(&trits)?))
    }
    /// Promotes a transaction to get it confirmed faster
    pub async fn promote(&self, tail_transaction: Hash) -> crate::Result<String> {
        let transfers = vec![Transfer {
            address: Address::from_inner_unchecked(
                TryteBuf::try_from_str(&String::from(
                    "PROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDR",
                ))?
                .as_trits()
                .encode(),
            ),
            value: 0,
            message: None,
            tag: None,
        }];

        let prepared_transfers = self.iota.prepare_transfers(None).transfers(transfers).build().await?;
        let tips = self.iota.get_transactions_to_approve().depth(2).send().await?;
        let attached_trytes = self
            .iota
            .attach_to_tangle()
            .trunk_transaction(&tail_transaction)
            .branch_transaction(&tips.branch_transaction)
            .trytes(&[prepared_transfers[0].clone()])
            .send()
            .await?;

        self.iota.broadcast_transactions(&attached_trytes.trytes).await?;
        Ok(prepared_transfers[0]
            .bundle()
            .to_inner()
            .as_i8_slice()
            .trytes()
            .expect("Couldn't get Trytes"))
    }

    /// Returns confirmation status
    pub async fn is_confirmed(&self, tail_transaction: Hash) -> crate::Result<bool> {
        let inclusion_states = self
            .iota
            .get_inclusion_states()
            .transactions(&[tail_transaction])
            .send()
            .await?;
        Ok(inclusion_states.states.contains(&true))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Differences {
    pub did: DID,
    pub diff: String,
    pub time: String,
    pub auth_key: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Payload {
    DIDDocument(DIDDocument),
    DIDDocumentDifferences(Differences),
}

fn check_network(id_segments: Vec<String>, network: &Network) -> crate::Result<()> {
    match (id_segments[0].as_str(), network) {
        ("dev", Network::Devnet) => Ok(()),
        ("com", Network::Comnet) => Ok(()),
        (_, Network::Devnet) => Err(Error::NetworkNodeError("dev")),
        (_, Network::Comnet) => Err(Error::NetworkNodeError("com")),
        (_, Network::Mainnet) => Ok(()),
    }
}
