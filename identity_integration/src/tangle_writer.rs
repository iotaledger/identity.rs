use crate::did_helper::did_iota_address;
use identity_core::did::DID;
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
pub struct Differences {
    pub did: DID,
    pub diff: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Payload {
    DIDDocument(DIDDocument),
    DIDDocumentDifferences(Differences),
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct TangleWriter {
    iota: iota::Client,
    network: iota::client::builder::Network,
}

impl TangleWriter {
    pub fn new(nodes: Vec<&'static str>, network: iota::client::builder::Network) -> crate::Result<Self> {
        let iota = iota::ClientBuilder::new()
            .nodes(&nodes)?
            .network(network.clone())
            .build()?;
        Ok(Self { iota, network })
    }
    /// Publishes DID document to the Tangle
    pub async fn publish_document(&self, did_document: &Payload) -> crate::Result<Hash> {
        let id_segments;
        let document = match did_document {
            Payload::DIDDocument(document) => {
                id_segments = document.derive_did()?.id_segments;
                document.to_string()
            }
            Payload::DIDDocumentDifferences(differences) => {
                id_segments = differences.did.id_segments.clone();
                serde_json::to_string(&differences)?
            }
        };
        // Check if correct network
        check_network(id_segments.clone(), &self.network)?;

        let address = did_iota_address(id_segments.last().expect("Failed to get id_segment"));

        // Diff chain address in did_document?
        // Is it possible to get the address from the did_document after an auth change?
        // let serialzed_did_message = serde_json::to_string(&did_document.to_string())?;
        let transfers = vec![Transfer {
            address: Address::from_inner_unchecked(TryteBuf::try_from_str(&address)?.as_trits().encode()),
            value: 0,
            message: Some(document),
            tag: Some(
                Tag::try_from_inner(
                    TryteBuf::try_from_str("DID999999999999999999999999")?
                        .as_trits()
                        .encode(),
                )
                .expect("Can't convert tag"),
            ),
        }];

        // Send the transaction
        let bundle = self.iota.send(None).transfers(transfers).send().await?;

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
        // Get confirmation status
        let inclusion_states = self
            .iota
            .get_inclusion_states()
            .transactions(&[tail_transaction])
            .send()
            .await?;
        Ok(inclusion_states.states.contains(&true))
    }
}

fn check_network(id_segments: Vec<String>, network: &iota_network) -> crate::Result<()> {
    match id_segments[0] {
        _ if id_segments[0] == "dev" => match network {
            iota_network::Devnet => {}
            _ => return Err(crate::Error::NetworkNodeError),
        },
        _ if id_segments[0] == "com" => match network {
            iota_network::Comnet => {}
            _ => return Err(crate::Error::NetworkNodeError),
        },
        _ => match network {
            iota_network::Mainnet => {}
            _ => return Err(crate::Error::NetworkNodeError),
        },
    };
    Ok(())
}
