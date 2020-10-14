use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    did::{DIDDocument as Document, DID},
    error::Result,
    resolver::{DocumentMetadata, InputMetadata},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MetaDocument {
    pub data: Document,
    pub meta: DocumentMetadata,
}

#[async_trait]
pub trait ResolverMethod {
    fn is_supported(&self, did: &DID) -> bool;

    async fn read(&self, did: &DID, input: InputMetadata) -> Result<Option<MetaDocument>>;
}

#[async_trait]
impl<T> ResolverMethod for &'_ T
where
    T: ResolverMethod + Send + Sync,
{
    fn is_supported(&self, did: &DID) -> bool {
        (**self).is_supported(did)
    }

    async fn read(&self, did: &DID, input: InputMetadata) -> Result<Option<MetaDocument>> {
        (**self).read(did, input).await
    }
}
