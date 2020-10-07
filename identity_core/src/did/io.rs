use crate::did::DID;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityWriter {
    type Payload;
    type Hash;
    type Error;
    async fn send(&self, did_document: &Self::Payload) -> Result<Self::Hash, Self::Error>;
}

#[async_trait]
pub trait IdentityReader {
    type FetchResponse;
    type HashDocument;
    type HashDiff;
    type Error;
    // Fetch documents and diffs with a single API call
    async fn fetch(&self, did: &DID) -> Result<Self::FetchResponse, Self::Error>;
    async fn fetch_documents(&self, did: &DID) -> Result<Vec<Self::HashDocument>, Self::Error>;
    async fn fetch_diffs(&self, did: &DID) -> Result<Vec<Self::HashDiff>, Self::Error>;
    // Fetch documents from FetchResponse
    // fn fetch_documents_from(&self, response: Self::FetchResponse) -> Result<Vec<Self::HashDocument>, Self::Error>;
    // Fetch diffs from FetchResponse
    // fn fetch_diffs_from(&self, response: Self::FetchResponse) -> Result<Vec<Self::HashDocument>, Self::Error>;
}
