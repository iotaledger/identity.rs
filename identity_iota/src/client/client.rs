use async_trait::async_trait;
use core::slice::from_ref;
use identity_core::{
    common::Url,
    convert::{SerdeInto as _, ToJson as _},
    did_doc::Document,
    did_url::DID,
    error::{Error as CoreError, Result as CoreResult},
    resolver::{DocumentMetadata, InputMetadata, MetaDocument, ResolverMethod},
};
use iota::{client::Transfer, crypto::ternary::Hash, transaction::bundled::BundledTransaction};

use crate::{
    client::{
        ClientBuilder, Network, ReadDocumentRequest, ReadDocumentResponse, ReadTransactionsRequest, TransactionPrinter,
    },
    did::{DIDDiff, IotaDID, IotaDocument},
    error::{Error, Result},
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

    pub fn explorer_url(&self) -> &'static Url {
        self.network.explorer_url()
    }

    pub fn transaction_url(&self, transaction: &BundledTransaction) -> Url {
        self.network.transaction_url(transaction)
    }

    pub fn transaction_hash(&self, transaction: &BundledTransaction) -> String {
        TransactionPrinter::hash(transaction).to_string()
    }

    pub fn read_transactions<'a>(&'a self, did: &IotaDID) -> ReadTransactionsRequest<'a> {
        ReadTransactionsRequest::new(self, did.address().unwrap())
    }

    pub fn read_document<'a, 'b>(&'a self, did: &'b IotaDID) -> ReadDocumentRequest<'a, 'b> {
        ReadDocumentRequest::new(self, did)
    }

    pub async fn publish_document(&self, document: &IotaDocument) -> Result<BundledTransaction> {
        trace!("Publish Document with DID: {}", document.id());

        trace!("Authentication: {:?}", document.authentication());
        trace!("Tangle Address: {}", document.id().address_hash());

        self.check_network(document.id())?;

        self.send_transfer(Transfer {
            address: document.id().address()?,
            value: 0,
            message: Some(document.to_json()?),
            tag: None,
        })
        .await
    }

    pub async fn publish_diff(&self, diff: &DIDDiff, index: usize) -> Result<BundledTransaction> {
        trace!("Publish Diff with DID: {}", diff.did);

        trace!("Previous Message: {}", diff.prev_msg);
        trace!("Document Changes: {}", diff.diff);
        trace!("Tangle Address: {}", diff.did.diff_address_hash(index));

        self.check_network(&diff.did)?;

        self.send_transfer(Transfer {
            address: diff.did.diff_address(index)?,
            value: 0,
            message: Some(diff.to_json()?),
            tag: None,
        })
        .await
    }

    pub async fn send_transfer(&self, transfer: Transfer) -> Result<BundledTransaction> {
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

    pub async fn is_transaction_confirmed(&self, hash: &Hash) -> Result<bool> {
        self.client
            .get_inclusion_states()
            .transactions(from_ref(hash))
            .send()
            .await
            .map_err(Into::into)
            .map(|states| states.states.as_slice() == [true])
    }

    fn check_network(&self, did: &IotaDID) -> Result<()> {
        // Ensure the correct network is selected.
        if !self.network.matches_did(did) {
            return Err(Error::InvalidDIDNetwork);
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl ResolverMethod for Client {
    fn is_supported(&self, did: &DID) -> bool {
        match IotaDID::try_from_borrowed(did) {
            Ok(did) => self.network.matches_did(&did),
            Err(_) => false,
        }
    }

    async fn read(&self, did: &DID, _input: InputMetadata) -> CoreResult<Option<MetaDocument>> {
        let did: &IotaDID =
            IotaDID::try_from_borrowed(did).map_err(|error| CoreError::ResolutionError(error.into()))?;

        let response: ReadDocumentResponse = self
            .read_document(&did)
            .send()
            .await
            .map_err(|error| CoreError::ResolutionError(error.into()))?;

        let mut metadata: DocumentMetadata = DocumentMetadata::new();

        metadata.created = Some(response.document.created());
        metadata.updated = Some(response.document.updated());
        metadata.properties = response.metadata;

        let data: Document = response
            .document
            .serde_into()
            .map_err(|error| CoreError::ResolutionError(error.into()))?;

        Ok(Some(MetaDocument { data, meta: metadata }))
    }
}
