use chrono::prelude::*;
use chrono::DateTime;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{did_helper::did_iota_address, tangle_reader::TangleReader};

pub struct ResolutionMetadata {}

pub struct Resolver {
    pub nodes: Vec<&'static str>,
}
impl Resolver {
    pub fn new(nodes: Vec<&'static str>) -> Self {
        Self { nodes }
    }
    /// Resolve a DID document
    pub async fn resolve(&self, did: DID, _resolution_metadata: ResolutionMetadata) -> crate::Result<DIDDocument> {
        let reader = TangleReader::new(self.nodes.clone());
        let messages = reader
            .fetch(&did_iota_address(
                &DID::parse_from_str(did)
                    .map_err(|_| crate::Error::DIDParsingError)?
                    .id_segments[0],
            ))
            .await
            .map_err(|_| crate::Error::FetchingError)?;

        let mut documents: Vec<DIDDocument> = messages
            .iter()
            .filter_map(|msg| {
                if let Ok(payload) = serde_json::from_str::<DIDDocument>(&msg) {
                    Some(payload)
                } else {
                    None
                }
            })
            .collect();

        documents.sort_by(|a, b| {
            b.updated
                .parse::<DateTime<Utc>>()
                .expect("Parsing time failed")
                .cmp(&a.updated.parse::<DateTime<Utc>>().expect("Parsing time failed"))
        });

        if !documents.is_empty() {
            Ok(documents.remove(0))
        } else {
            Err(crate::Error::DocumentNotFound)
        }
    }
}
