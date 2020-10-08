use crate::did::{DIDDocument, DID};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityWriter {
    type Diff;
    type Error: Into<anyhow::Error>;
    async fn send_doc(&self, did_document: &DIDDocument) -> Result<Vec<u8>, Self::Error>;
    async fn send_diff(&self, did_document_diff: &Self::Diff) -> Result<Vec<u8>, Self::Error>;
}

#[async_trait]
pub trait IdentityReader {
    type HashDocument;
    type HashDiff;
    type Error;
    async fn fetch(
        &self,
        did: &DID,
    ) -> Result<(Option<Vec<Self::HashDocument>>, Option<Vec<Self::HashDiff>>), Self::Error>;
    async fn fetch_documents(&self, did: &DID) -> Result<Option<Vec<Self::HashDocument>>, Self::Error>;
    async fn fetch_diffs(&self, did: &DID) -> Result<Option<Vec<Self::HashDiff>>, Self::Error>;
}
