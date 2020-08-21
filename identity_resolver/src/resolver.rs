use chrono::prelude::*;
use chrono::DateTime;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{did_helper::did_iota_address, tangle_reader::TangleReader};

pub struct ResolutionMetadata {}

pub struct Resolver {
    pub nodes: Vec<&'static str>,
}
impl Resolver {
    /// Resolve a DID document
    pub async fn resolve(&self, did: DID, _resolution_metadata: ResolutionMetadata) -> crate::Result<DIDDocument> {
        let reader = TangleReader {
            nodes: self.nodes.clone(),
        };
        let messages = reader
            .fetch(&did_iota_address(&DID::parse_from_str(did).unwrap().id_segments[0]))
            .await
            .unwrap();

        let mut documents: Vec<DIDDocument> = messages
            .iter()
            .filter_map(|s| {
                if let Ok(payload) = serde_json::from_str::<DIDDocument>(&s) {
                    Some(payload)
                } else {
                    None
                }
            })
            .collect();

        documents.sort_by(|a, b| {
            b.updated
                .parse::<DateTime<Utc>>()
                .unwrap()
                .cmp(&a.updated.parse::<DateTime<Utc>>().unwrap())
        });

        if documents.len() > 0 {
            Ok(documents.remove(0))
        } else {
            Err(crate::Error::DocumentNotFound)
        }
    }
}
