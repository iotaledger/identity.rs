use async_trait::async_trait;
use identity_core::{
    self as core,
    did::DID,
    resolver::{DocumentMetadata, InputMetadata, MetaDocument, ResolverMethod},
};

use crate::{error::Result, io::TangleReader, network::NodeList, types::TangleDoc};

#[derive(Debug)]
pub struct TangleResolver {
    nodes: NodeList,
}

impl TangleResolver {
    pub const fn new() -> Self {
        Self { nodes: NodeList::new() }
    }

    pub fn with_nodes(nodes: impl Into<NodeList>) -> Self {
        Self { nodes: nodes.into() }
    }

    pub fn set_nodes(&mut self, nodes: impl Into<NodeList>) {
        self.nodes = nodes.into();
    }

    pub fn nodes(&self) -> &NodeList {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut NodeList {
        &mut self.nodes
    }

    pub async fn document(&self, did: &DID) -> Result<MetaDocument> {
        let reader: TangleReader = TangleReader::new(&self.nodes)?;
        let document: TangleDoc = reader.fetch_latest(did).await.map(|(doc, _)| doc)?;

        let mut metadata: DocumentMetadata = DocumentMetadata::new();

        metadata.created = document.data.created;
        metadata.updated = document.data.updated;

        Ok(MetaDocument {
            data: document.data,
            meta: metadata,
        })
    }
}

#[async_trait(?Send)]
impl ResolverMethod for TangleResolver {
    fn is_supported(&self, did: &DID) -> bool {
        // The DID method MUST be IOTA.
        if did.method_name != "iota" {
            return false;
        }

        // The DID network MUST match the configured network.
        self.nodes.network().matches_did(did)
    }

    async fn read(&self, did: &DID, _input: InputMetadata) -> core::Result<Option<MetaDocument>> {
        self.document(did)
            .await
            .map_err(|error| core::Error::ResolutionError(error.into()))
            .map(Some)
    }
}
