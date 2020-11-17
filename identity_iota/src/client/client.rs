use async_trait::async_trait;
use core::slice::from_ref;
use identity_core::{
    common::Url,
    convert::SerdeInto as _,
    did_doc::Document,
    did_url::DID,
    error::{Error, Result as CoreResult},
    resolver::{DocumentMetadata, InputMetadata, MetaDocument, ResolverMethod},
};
use iota::{crypto::ternary::Hash, transaction::bundled::BundledTransaction};

use crate::{
    client::{
        ClientBuilder, Network, PublishDocumentRequest, ReadDocumentRequest, ReadDocumentResponse,
        ReadTransactionsRequest, SendTransferRequest,
    },
    did::{IotaDID, IotaDocument},
    error::Result,
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

    pub fn read_transactions<'a>(&'a self, did: &IotaDID) -> ReadTransactionsRequest<'a> {
        ReadTransactionsRequest::new(self, did.address().unwrap())
    }

    pub fn send_transfer(&self) -> SendTransferRequest {
        SendTransferRequest::new(self)
    }

    pub fn publish_document<'a, 'b>(&'a self, document: &'b IotaDocument) -> PublishDocumentRequest<'a, 'b> {
        PublishDocumentRequest::new(self.send_transfer(), document)
    }

    pub fn read_document<'a, 'b>(&'a self, did: &'b IotaDID) -> ReadDocumentRequest<'a, 'b> {
        ReadDocumentRequest::new(self, did)
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

#[async_trait(?Send)]
impl ResolverMethod for Client {
    fn is_supported(&self, did: &DID) -> bool {
        match IotaDID::try_from_borrowed(did) {
            Ok(did) => self.network.matches_did(&did),
            Err(_) => false,
        }
    }

    async fn read(&self, did: &DID, _input: InputMetadata) -> CoreResult<Option<MetaDocument>> {
        let did: &IotaDID = IotaDID::try_from_borrowed(did).map_err(|error| Error::ResolutionError(error.into()))?;

        let response: ReadDocumentResponse = self
            .read_document(&did)
            .send()
            .await
            .map_err(|error| Error::ResolutionError(error.into()))?;

        let mut metadata: DocumentMetadata = DocumentMetadata::new();

        metadata.created = Some(response.document.created());
        metadata.updated = Some(response.document.updated());
        metadata.properties = response.metadata;

        let data: Document = response
            .document
            .serde_into()
            .map_err(|error| Error::ResolutionError(error.into()))?;

        Ok(Some(MetaDocument { data, meta: metadata }))
    }
}
