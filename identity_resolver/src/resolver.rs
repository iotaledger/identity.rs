use chrono::{prelude::*, DateTime};
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{did_helper::did_iota_address, tangle_reader::TangleReader};

pub struct ResolutionMetadata {}

pub struct Resolver {
    pub nodes: NetworkNodes,
}
#[derive(Clone, Debug)]
pub enum NetworkNodes {
    Main(Vec<&'static str>),
    Com(Vec<&'static str>),
    Dev(Vec<&'static str>),
}

impl Resolver {
    pub fn new(nodes: NetworkNodes) -> Self {
        Self { nodes }
    }
    /// Resolve a DID document
    pub async fn resolve(&self, did: DID, _resolution_metadata: ResolutionMetadata) -> crate::Result<DIDDocument> {
        let did = DID::parse_from_str(did)?;
        let (did_id, nodes) = get_id_and_nodes(&did.id_segments, self.nodes.clone())?;
        let reader = TangleReader::new(nodes.to_vec());
        let messages = reader
            .fetch(&did_iota_address(&did_id))
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

fn get_id_and_nodes(did_segments: &[String], nodes: NetworkNodes) -> crate::Result<(String, Vec<&'static str>)> {
    let nodes: Vec<&'static str> = match did_segments[0] {
        _ if did_segments[0] == "dev" => match nodes {
            NetworkNodes::Dev(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError),
        },
        _ if did_segments[0] == "com" => match nodes {
            NetworkNodes::Com(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError),
        },
        _ => match nodes {
            NetworkNodes::Main(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError),
        },
    };
    if nodes.is_empty() {
        return Err(crate::Error::NetworkNodeError);
    }
    Ok((did_segments.last().expect("Failed to get id_segment").into(), nodes))
}
