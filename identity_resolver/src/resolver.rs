use chrono::prelude::*;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{did_helper::did_iota_address, tangle_reader::TangleReader, tangle_writer::Differences};
use serde_diff::Apply;
use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
pub struct ResolutionResult {
    pub metadata: ResolutionMetadata,
    pub did_document: DIDDocument,
    pub did_document_metadata: HashMap<String, String>,
}
#[derive(Debug)]
pub struct ResolutionMetadata {
    driver_id: String,
    retrieved: String,
    duration: u128,
}
pub struct ResolutionInputMetadata {
    pub accept: Option<String>,
    pub service_type: Option<String>,
    pub follow_redirect: Option<bool>,
}
impl ResolutionInputMetadata {
    pub fn default() -> Self {
        Self {
            accept: None,
            service_type: None,
            follow_redirect: None,
        }
    }
}
pub struct Resolver {
    pub nodes: NetworkNodes,
}
#[derive(Clone, Debug)]
pub enum NetworkNodes {
    Main(Vec<&'static str>),
    Com(Vec<&'static str>),
    Dev(Vec<&'static str>),
}
#[derive(Clone)]
struct HashWithMessage {
    tailhash: String,
    document: DIDDocument,
}
struct HashWithDiff {
    tailhash: String,
    diff: Differences,
}

impl Resolver {
    pub fn new(nodes: NetworkNodes) -> Self {
        Self { nodes }
    }
    /// Resolve a DID document
    pub async fn resolve(
        &self,
        did: DID,
        _resolution_metadata: ResolutionInputMetadata,
    ) -> crate::Result<ResolutionResult> {
        let start_time = Instant::now();
        let did = DID::parse_from_str(did)?;
        let (did_id, nodes) = get_id_and_nodes(&did.id_segments, self.nodes.clone())?;
        let reader = TangleReader::new(nodes.to_vec());
        let messages = reader.fetch(&did_iota_address(&did_id)).await?;
        let documents = get_ordered_documents(messages.clone(), &did_id)?;
        let diffs = get_ordered_diffs(messages, &did_id)?;
        let mut latest_document = documents[0].clone();
        let mut metadata = HashMap::new();
        // Apply diffs
        for (i, diff) in diffs.iter().enumerate() {
            let mut deserializer = serde_json::Deserializer::from_str(&diff.diff.diff);
            Apply::apply(&mut deserializer, &mut latest_document.document)?;
            metadata.insert(format!("diff_tail_transaction {}", i), diff.tailhash.clone());
        }

        metadata.insert("document_tail_transaction".into(), latest_document.tailhash.clone());
        let result = ResolutionResult {
            did_document: latest_document.document,
            metadata: ResolutionMetadata {
                driver_id: "did:iota".into(),
                retrieved: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                duration: start_time.elapsed().as_millis(),
            },
            did_document_metadata: metadata,
        };
        Ok(result)
    }
}

fn get_id_and_nodes(did_segments: &[String], nodes: NetworkNodes) -> crate::Result<(String, Vec<&'static str>)> {
    let nodes: Vec<&'static str> = match did_segments[0] {
        _ if did_segments[0] == "dev" => match nodes {
            NetworkNodes::Dev(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError("dev")),
        },
        _ if did_segments[0] == "com" => match nodes {
            NetworkNodes::Com(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError("com")),
        },
        _ => match nodes {
            NetworkNodes::Main(nodes) => nodes,
            _ => return Err(crate::Error::NetworkNodeError("main")),
        },
    };
    if nodes.is_empty() {
        return Err(crate::Error::NodeError);
    }
    Ok((did_segments.last().expect("Failed to get id_segment").into(), nodes))
}

/// Order documents: first element is latest
fn get_ordered_documents(messages: HashMap<String, String>, did_id: &str) -> crate::Result<Vec<HashWithMessage>> {
    let mut documents: Vec<HashWithMessage> = messages
        .iter()
        .filter_map(|(tailhash, msg)| {
            if let Ok(document) = serde_json::from_str::<DIDDocument>(&msg) {
                if document
                    .derive_did()
                    .expect("Failed to get DID from document")
                    .id_segments
                    .last()
                    .expect("Failed to get id_segment")
                    == did_id
                {
                    Some(HashWithMessage {
                        tailhash: tailhash.into(),
                        document,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    if documents.is_empty() {
        return Err(crate::Error::DocumentNotFound);
    }
    documents.sort_by(|a, b| b.document.updated.cmp(&a.document.updated));
    Ok(documents)
}

/// Order diffs: first element is oldest
fn get_ordered_diffs(messages: HashMap<String, String>, did_id: &str) -> crate::Result<Vec<HashWithDiff>> {
    let mut diffs: Vec<HashWithDiff> = messages
        .iter()
        .filter_map(|(tailhash, msg)| {
            if let Ok(diff) = serde_json::from_str::<Differences>(&msg) {
                if diff.did.id_segments.last().expect("Failed to get id_segment") == did_id {
                    Some(HashWithDiff {
                        tailhash: tailhash.into(),
                        diff,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    if diffs.is_empty() {
        return Ok(diffs);
    }
    diffs.sort_by(|a, b| a.diff.time.cmp(&b.diff.time));
    Ok(diffs)
}
