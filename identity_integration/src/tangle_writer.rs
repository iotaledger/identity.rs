use crate::helpers::get_iota_address;
use identity_core::{did::DID, document::DIDDocument};
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
use std::{fmt, thread, time::Duration};

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
    pub async fn new(nodes: Vec<&'static str>, network: iota::client::builder::Network) -> crate::Result<Self> {
        let iota = iota::ClientBuilder::new()
            .nodes(&nodes)?
            .network(network.clone())
            .build()
            .await?;
        Ok(Self { iota, network })
    }
    /// Sends document or diff to the Tangle and promotes the transaction until it's confirmed
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
            Payload::DIDDocument(document) => (document.derive_did()?, document.to_string()),
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

fn check_network(id_segments: Vec<String>, network: &iota_network) -> crate::Result<()> {
    match id_segments[0] {
        _ if id_segments[0] == "dev" => match network {
            iota_network::Devnet => {}
            _ => return Err(crate::Error::NetworkNodeError("dev")),
        },
        _ if id_segments[0] == "com" => match network {
            iota_network::Comnet => {}
            _ => return Err(crate::Error::NetworkNodeError("com")),
        },
        _ => match network {
            iota_network::Mainnet => {}
            _ => return Err(crate::Error::NetworkNodeError("main")),
        },
    };
    Ok(())
}
