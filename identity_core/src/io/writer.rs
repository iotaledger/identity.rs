use async_trait::async_trait;

use crate::{did::DIDDocument, error::Result};

#[async_trait]
pub trait IdentityWriter {
    type Hash: PartialEq;

    async fn write_document(&self, document: &DIDDocument) -> Result<Self::Hash>;
}
