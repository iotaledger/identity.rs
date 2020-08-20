use anyhow::Result as AnyhowResult;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{
    did_helper::did_iota_address,
    tangle_reader::TangleReader,
    tangle_writer::{DIDMessage, Payload},
};
use thiserror::Error;

pub struct ResolutionMetadata {}

pub struct Resolver {
    pub nodes: Vec<&'static str>,
}
impl Resolver {
    /// Resolve a DID document
    pub async fn resolve(&self, did: DID, _resolution_metadata: ResolutionMetadata) -> Result<DIDDocument> {
        let reader = TangleReader {
            nodes: self.nodes.clone(),
        };
        let messages = reader
            .fetch(&did_iota_address(&DID::parse_from_str(did).unwrap().id_segments[0]))
            .await
            .unwrap();
        let fetched_did_message: DIDMessage = serde_json::from_str(&messages[0]).unwrap();
        if let Payload::DIDDocument(doc) = fetched_did_message.payload {
            Ok(doc)
        } else {
            Err(Error::DocumentNotFound)
        }
    }
}
#[derive(Debug, Error)]
pub enum Error {
    /// Didn't found a document
    #[error("Resolve Error: No document found")]
    DocumentNotFound,
}
pub type Result<T> = AnyhowResult<T, Error>;
