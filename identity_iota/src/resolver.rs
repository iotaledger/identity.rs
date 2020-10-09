use async_trait::async_trait;
use identity_core::{
    self as core,
    common::Timestamp,
    did::DID,
    error::Result,
    resolver::{Document, IdentityResolver, ResolutionInput},
};

use crate::{io::TangleReader, network::NodeList, types::TangleDoc};

#[derive(Debug)]
pub struct TangleResolver {
    nodes: NodeList,
}

impl TangleResolver {
    pub const fn new() -> Self {
        Self { nodes: NodeList::new() }
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
}

#[async_trait]
impl IdentityResolver for TangleResolver {
    fn is_supported(&self, did: &DID) -> bool {
        // The DID method MUST be IOTA.
        if did.method_name != "iota" {
            return false;
        }

        // The DID network MUST match the configured network.
        self.nodes.network().matches_did(did)
    }

    async fn document(&self, did: &DID, _input: &ResolutionInput) -> Result<Option<Document>> {
        let reader: TangleReader =
            TangleReader::new(&self.nodes).map_err(|error| core::Error::ResolutionError(error.into()))?;

        // TODO: Support `input.version`
        // TODO: Support `input.no_cache`

        let (document, mut metadata): (TangleDoc, _) = reader
            .fetch_latest(did)
            .await
            .map_err(|error| core::Error::ResolutionError(error.into()))?;

        if let Some(timestamp) = document.created.as_ref().map(Timestamp::to_rfc3339) {
            metadata.insert("created".into(), timestamp.into());
        }

        if let Some(timestamp) = document.updated.as_ref().map(Timestamp::to_rfc3339) {
            metadata.insert("updated".into(), timestamp.into());
        }

        Ok(Some((document.data, metadata)))
    }
}
