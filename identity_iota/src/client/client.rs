use core::slice::from_ref;
use iota::crypto::ternary::Hash;

use crate::{
    client::{ClientBuilder, ReadTransactionsRequest, SendTransferRequest},
    did::IotaDID,
    error::Result,
    network::Network,
};

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) client: iota::Client,
    pub(crate) network: Network,
}

impl Client {
    pub fn new() -> Result<Self> {
        Self::from_builder(ClientBuilder::new())
    }

    pub fn from_builder(builder: ClientBuilder) -> Result<Self> {
        let mut client: iota::ClientBuilder = iota::ClientBuilder::new();

        for node in builder.nodes {
            client = client.node(&node)?;
        }

        client = client.network(builder.network.into());

        Ok(Self {
            client: client.build()?,
            network: builder.network,
        })
    }

    pub fn read_transactions<'a>(&'a self, did: &IotaDID) -> ReadTransactionsRequest<'a> {
        ReadTransactionsRequest::new(self, did.create_address().unwrap())
    }

    pub fn send_transfer(&self) -> SendTransferRequest {
        SendTransferRequest::new(self)
    }

    pub async fn is_transaction_confirmed(&self, hash: &Hash) -> Result<bool> {
        self.client
            .get_inclusion_states()
            .transactions(from_ref(hash))
            .send()
            .await
            .map_err(Into::into)
            .map(|states| states.states.as_slice() == [true])
    }
}
