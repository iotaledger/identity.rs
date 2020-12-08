use core::slice::from_ref;
use identity_core::{common::Url, convert::ToJson};
use iota::{
    client::{FindTransactionsResponse, GetTrytesResponse, Transfer},
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField as _},
};

use crate::{
    chain::{AuthChain, DiffChain, DocumentChain},
    client::{ClientBuilder, Network, TangleMessage, TransactionPrinter},
    did::{DocumentDiff, IotaDID, IotaDocument},
    error::{Error, Result},
    utils::{bundles_from_trytes, create_address_from_trits, encode_trits},
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

    pub fn from_network(network: Network) -> Result<Self> {
        ClientBuilder::new()
            .node(network.node_url().as_str())
            .network(network)
            .build()
    }

    pub fn from_builder(builder: ClientBuilder) -> Result<Self> {
        let mut client: iota::ClientBuilder = iota::ClientBuilder::new();

        if builder.nodes.is_empty() {
            client = client.node(builder.network.node_url().as_str())?;
        } else {
            for node in builder.nodes {
                client = client.node(&node)?;
            }
        }

        client = client.network(builder.network.into());

        Ok(Self {
            client: client.build()?,
            network: builder.network,
        })
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn explorer_url(&self) -> &'static Url {
        self.network.explorer_url()
    }

    pub fn transaction_url(&self, transaction: &BundledTransaction) -> Url {
        self.network.transaction_url(transaction)
    }

    pub fn transaction_hash(&self, transaction: &BundledTransaction) -> String {
        TransactionPrinter::hash(transaction).to_string()
    }

    pub async fn publish_document(&self, document: &IotaDocument) -> Result<BundledTransaction> {
        trace!("Publishing DID Document: {}", document.id());
        trace!("Authentication: {:?}", document.authentication());
        trace!("Document Proof: {}", document.verify().is_ok());
        trace!("Tangle Address: {}", document.id().address());

        self.check_network(document.id())?;

        let address: String = document.id().address();
        let transfer: Transfer = Self::json_transfer(&address, document)?;

        self.send_transfer(transfer).await
    }

    pub async fn publish_diff(&self, diff: &DocumentDiff, message_id: &str) -> Result<BundledTransaction> {
        trace!("Publish DID Document Diff: {}", diff.did);
        trace!("Previous Message: {}", diff.previous_message_id);
        trace!("Document Changes: {}", diff.diff);
        trace!("Tangle Address: {}", IotaDocument::diff_address(message_id));

        self.check_network(&diff.did)?;

        let address: String = IotaDocument::diff_address(message_id);
        let transfer: Transfer = Self::json_transfer(&address, diff)?;

        self.send_transfer(transfer).await
    }

    pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
        trace!("Read Document Chain: {}", did);
        trace!("Auth Chain Address: {}", did.address());

        // Fetch all messages for the auth chain.
        let address: Address = create_address_from_trits(did.address())?;
        let messages: Vec<TangleMessage> = self.read_transactions(&address, true).await?;

        trace!("Tangle Messages: {:?}", messages);

        let auth: AuthChain = AuthChain::try_from_messages(did, &messages)?;

        let diff: DiffChain = if auth.current().immutable() {
            DiffChain::new()
        } else {
            // Fetch all messages for the diff chain.
            let address: String = IotaDocument::diff_address(auth.current_message_id());
            let address: Address = create_address_from_trits(&address)?;
            let messages: Vec<TangleMessage> = self.read_transactions(&address, true).await?;

            trace!("Tangle Messages: {:?}", messages);

            DiffChain::try_from_messages(&auth, &messages)?
        };

        Ok(DocumentChain::new(auth, diff))
    }

    pub async fn read_document(&self, did: &IotaDID) -> Result<IotaDocument> {
        self.read_document_chain(did).await.and_then(DocumentChain::fold)
    }

    pub(crate) fn check_network(&self, did: &IotaDID) -> Result<()> {
        // Ensure the correct network is selected.
        if !self.network.matches_did(did) {
            return Err(Error::InvalidDIDNetwork);
        }

        Ok(())
    }

    async fn send_transfer(&self, transfer: Transfer) -> Result<BundledTransaction> {
        trace!("Sending Transfer: {:?}", transfer.message);

        self.client
            .send(None)
            .transfers(vec![transfer])
            .send()
            .await?
            .into_iter()
            .find(|transaction| transaction.is_tail())
            .ok_or(Error::InvalidTransferTail)
    }

    async fn read_transactions(&self, address: &Address, allow_empty: bool) -> Result<Vec<TangleMessage>> {
        fn __dbg_transactions(response: &FindTransactionsResponse) -> Vec<String> {
            response.hashes.iter().map(|hash| encode_trits(hash)).collect()
        }

        fn __dbg_trytes(response: &GetTrytesResponse) -> Vec<TransactionPrinter> {
            response.trytes.iter().map(TransactionPrinter::full).collect()
        }

        trace!("Read Transactions: {}", encode_trits(address.to_inner()));

        // Fetch all transaction hashes containing the tangle address.
        let response: FindTransactionsResponse = self
            .client
            .find_transactions()
            .addresses(from_ref(address))
            .send()
            .await?;

        trace!("Transactions Found: {:?}", __dbg_transactions(&response));

        if response.hashes.is_empty() {
            if allow_empty {
                return Ok(Vec::new());
            } else {
                return Err(Error::InvalidTransactionHashes);
            }
        }

        // Fetch the content of all transactions.
        let content: GetTrytesResponse = self.client.get_trytes(&response.hashes).await?;

        trace!("Transaction Trytes: {:?}", __dbg_trytes(&content));

        if content.trytes.is_empty() {
            return Err(Error::InvalidTransactionTrytes);
        }

        // Re-build the fragmented messages stored in the bundle.
        bundles_from_trytes(content.trytes)
            .into_iter()
            .map(TangleMessage::try_from_bundle)
            .collect()
    }

    fn json_transfer<T>(address: &str, data: &T) -> Result<Transfer>
    where
        T: ToJson,
    {
        let address: Address = create_address_from_trits(address)?;
        let message: String = data.to_json()?;

        Ok(Transfer {
            address,
            value: 0,
            message: Some(message),
            tag: None,
        })
    }
}
