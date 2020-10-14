use ::core::slice::from_ref;
use async_trait::async_trait;
use identity_core::{
    self as core,
    did::{DIDDocument, DID},
    io::IdentityWriter,
};
use iota::{
    client::{AttachToTangleResponse, GTTAResponse, Transfer},
    crypto::ternary::Hash,
    transaction::bundled::{Bundle, BundledTransaction},
};
use serde::Serialize;
use serde_json::to_string;
use std::{thread, time::Duration};

use crate::{
    did::create_address,
    error::{DocumentError, Error, Result, TransactionError},
    network::{Network, NodeList},
    utils::{create_address_from_trits, encode_trits, txn_hash},
};

/// Tipselection depth
const TS_DEPTH: u8 = 2;

/// Fixed-address used for faster transaction confirmation times
const PROMOTION: &str = "PROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDR";

#[derive(Debug)]
pub struct Config {
    confirm_delay: Duration,
    promote_delay: Duration,
    promote_limit: usize,
}

impl Config {
    const DEFAULT_CONFIRM_DELAY: Duration = Duration::from_secs(5);
    const DEFAULT_PROMOTE_DELAY: Duration = Duration::from_secs(5);
    const DEFAULT_PROMOTE_LIMIT: usize = 20;

    pub const fn new() -> Self {
        Self {
            confirm_delay: Self::DEFAULT_CONFIRM_DELAY,
            promote_delay: Self::DEFAULT_PROMOTE_DELAY,
            promote_limit: Self::DEFAULT_PROMOTE_LIMIT,
        }
    }
}

#[derive(Debug)]
pub struct TangleWriter {
    client: iota::Client,
    config: Config,
    network: Network,
}

impl TangleWriter {
    pub fn new(nodelist: &NodeList) -> Result<Self> {
        let mut builder: iota::ClientBuilder = iota::ClientBuilder::new();

        for node in nodelist.nodes() {
            builder = builder.node(node)?;
        }

        builder = builder.network(nodelist.network().into());

        Ok(Self {
            client: builder.build()?,
            network: nodelist.network(),
            config: Config::new(),
        })
    }

    pub async fn write_json<T>(&self, did: &DID, data: &T) -> Result<Hash>
    where
        T: Serialize,
    {
        let txn: Hash = self.publish_json(did, data).await?;

        println!("[+] Transaction > {}", encode_trits(txn.as_trits()));

        let mut tries: usize = 0;

        thread::sleep(self.config.confirm_delay);

        while !self.is_confirmed(&txn).await? {
            println!("[+] Promoting > {}", encode_trits(txn.as_trits()));

            tries += 1;
            thread::sleep(self.config.promote_delay);

            if tries >= self.config.promote_limit {
                return Err(Error::InvalidTransaction(TransactionError::Unconfirmable));
            }

            self.promote(&txn).await?;
        }

        Ok(txn)
    }

    pub async fn publish_json<T>(&self, did: &DID, data: &T) -> Result<Hash>
    where
        T: Serialize,
    {
        // Ensure the correct network is selected
        if !self.network.matches_did(did) {
            return Err(Error::InvalidDocument(DocumentError::NetworkMismatch));
        }

        // Build the transfer to push the serialized data to the tangle.
        let transfer: Transfer = Transfer {
            address: create_address(&did)?,
            value: 0,
            message: Some(to_string(&data).map_err(core::Error::EncodeJSON)?),
            tag: None,
        };

        // Dispatch the transfer to the network
        let bundle: Vec<BundledTransaction> = self.client.send(None).transfers(vec![transfer]).send().await?;

        // Extract the tail transaction from the response.
        let tail: &BundledTransaction = bundle
            .iter()
            .find(|txn| txn.is_tail())
            .ok_or_else(|| Error::InvalidTransaction(TransactionError::InvalidBundle))?;

        Ok(txn_hash(tail))
    }

    async fn promote(&self, txn: &Hash) -> Result<String> {
        let transfer: Transfer = Transfer {
            address: create_address_from_trits(PROMOTION)?,
            value: 0,
            message: None,
            tag: None,
        };

        let transfers: Bundle = self
            .client
            .prepare_transfers(None)
            .transfers(vec![transfer])
            .build()
            .await?;

        let tips: GTTAResponse = self.client.get_transactions_to_approve().depth(TS_DEPTH).send().await?;
        let tail: &BundledTransaction = transfers.tail();

        let trytes: AttachToTangleResponse = self
            .client
            .attach_to_tangle()
            .trunk_transaction(&txn)
            .branch_transaction(&tips.branch_transaction)
            .trytes(from_ref(tail))
            .send()
            .await?;

        self.client.broadcast_transactions(&trytes.trytes).await?;

        Ok(encode_trits(tail.bundle().as_trits()))
    }

    async fn is_confirmed(&self, txn: &Hash) -> Result<bool> {
        self.client
            .get_inclusion_states()
            .transactions(from_ref(txn))
            .send()
            .await
            .map_err(Into::into)
            .map(|states| states.states.as_slice() == [true])
    }
}

#[async_trait]
impl IdentityWriter for TangleWriter {
    type Hash = Hash;

    async fn write_document(&self, document: &DIDDocument) -> Result<Self::Hash, core::Error> {
        self.write_json(document.derive_did(), document)
            .await
            .map_err(|error| core::Error::ResolutionError(error.into()))
    }
}
