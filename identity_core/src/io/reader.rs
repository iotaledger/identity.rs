use async_trait::async_trait;

use crate::{
    did::{DIDDocument, DID},
    error::Result,
};

#[async_trait]
pub trait IdentityReader {
    type Hash: PartialEq;

    fn hash(&self, did: &DID) -> Result<Self::Hash>;

    async fn fetch_document(&self, did: &DID) -> Result<DIDDocument>;
}
