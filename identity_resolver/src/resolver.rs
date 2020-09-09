use bytestream::*;
use identity_core::{common::Timestamp, did::DID, document::DIDDocument};
use identity_integration::{did_helper::did_iota_address, tangle_reader::TangleReader, tangle_writer::Differences};
use serde_diff::Apply;
use std::{collections::HashMap, io::Write, time::Instant};

#[derive(Debug)]
pub struct ResolutionResult {
    pub metadata: ResolutionMetadata,
    pub did_document: Option<DIDDocument>,
    pub did_document_string: Option<String>,
    pub did_document_metadata: HashMap<String, String>,
}
#[derive(Debug)]
pub struct ResolutionResultStream {
    pub result: ResolutionResult,
}

#[derive(Debug)]
pub struct ResolutionMetadata {
    pub driver_id: String,
    pub retrieved: String,
    pub duration: u128,
    pub input_did: String,
}
pub struct ResolutionInputMetadata {
    pub accept: Option<String>,
    pub service_type: Option<String>,
    pub follow_redirect: Option<bool>,
    include_all_messages: bool,
}
impl ResolutionInputMetadata {
    pub fn default() -> Self {
        Self {
            accept: None,
            service_type: None,
            follow_redirect: None,
            include_all_messages: false,
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
    pub fn new(nodes: NetworkNodes) -> crate::Result<Self> {
        let node_vec = match &nodes {
            NetworkNodes::Com(nodes) => nodes,
            NetworkNodes::Dev(nodes) => nodes,
            NetworkNodes::Main(nodes) => nodes,
        };
        if node_vec.is_empty() {
            return Err(crate::Error::NodeError);
        }
        Ok(Self { nodes })
    }
    /// Resolve a DID document
    pub async fn resolve(
        &self,
        did: DID,
        resolution_metadata: ResolutionInputMetadata,
    ) -> crate::Result<ResolutionResult> {
        if did.method_name != "iota" {
            return Err(crate::Error::DIDMethodError);
        }
        let start_time = Instant::now();
        let mut metadata = HashMap::new();
        let (did_id, nodes) = get_id_and_nodes(&did.id_segments, self.nodes.clone())?;
        let reader = TangleReader::new(nodes.to_vec());
        // let messages = reader.fetch(&did_iota_address(&did_id)).await?;
        let messages = match reader.fetch(&did_iota_address(&did_id)).await {
            Ok(messages) => messages,
            _ => {
                metadata.insert("Error".to_string(), "not-found".to_string());
                return Ok(ResolutionResult {
                    did_document: None,
                    did_document_string: None,
                    metadata: ResolutionMetadata {
                        driver_id: "did:iota".into(),
                        retrieved: Timestamp::now().to_rfc3339(),
                        duration: start_time.elapsed().as_millis(),
                        input_did: did.to_string(),
                    },
                    did_document_metadata: metadata,
                });
            }
        };
        let documents = get_ordered_documents(messages.clone(), &did_id)?;
        let diffs = get_ordered_diffs(messages.clone(), &did_id)?;
        if resolution_metadata.include_all_messages {
            for (tailhash, msg) in messages {
                metadata.insert(tailhash, msg);
            }
        }
        let mut latest_document = documents[0].clone();
        // Apply diffs
        for (i, diff) in diffs.iter().enumerate() {
            if diff.diff.time
                > latest_document
                    .document
                    .updated
                    .clone()
                    .expect("Failed to get updated field")
            {
                let mut deserializer = serde_json::Deserializer::from_str(&diff.diff.diff);
                Apply::apply(&mut deserializer, &mut latest_document.document)?;
                metadata.insert(format!("diff_tail_transaction {}", i), diff.tailhash.clone());
            }
        }

        metadata.insert("document_tail_transaction".into(), latest_document.tailhash.clone());
        let result = ResolutionResult {
            did_document: Some(latest_document.document.clone()),
            did_document_string: Some(latest_document.document.to_string()),
            metadata: ResolutionMetadata {
                driver_id: "did:iota".into(),
                retrieved: Timestamp::now().to_rfc3339(),
                duration: start_time.elapsed().as_millis(),
                input_did: did.to_string(),
            },
            did_document_metadata: metadata,
        };
        Ok(result)
    }
    pub async fn resolve_stream<W: Write>(
        &self,
        did: DID,
        resolution_metadata: ResolutionInputMetadata,
        buffer: &mut W,
    ) -> crate::Result<()> {
        let res = self.resolve(did, resolution_metadata).await?;
        match res.did_document_string {
            Some(document) => {
                document.write_to(buffer, ByteOrder::BigEndian)?;
            }
            _ => return Err(crate::Error::DocumentNotFound),
        }
        Ok(())
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
