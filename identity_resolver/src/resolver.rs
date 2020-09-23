use bytestream::*;
use identity_core::{common::Timestamp, did::DID, document::DIDDocument};
use identity_diff::Diff;
use identity_integration::{did_helper::get_iota_address, tangle_reader::TangleReader, tangle_writer::Differences};
use std::{collections::HashMap, io::Write, time::Instant};

#[derive(Debug)]
pub struct ResolutionResult {
    pub metadata: ResolutionMetadata,
    pub did_document: Option<DIDDocument>,
    pub did_document_string: Option<String>,
    pub did_document_metadata: Option<HashMap<String, String>>,
}
impl ResolutionResult {
    pub fn default() -> Self {
        Self {
            metadata: ResolutionMetadata::default(),
            did_document: None,
            did_document_string: None,
            did_document_metadata: None,
        }
    }
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
    pub error: Option<String>,
    pub content_type: Option<String>,
}
impl ResolutionMetadata {
    pub fn default() -> Self {
        Self {
            driver_id: "did:iota".into(),
            retrieved: Timestamp::now().to_rfc3339(),
            duration: 0,
            input_did: "".into(),
            error: None,
            content_type: None,
        }
    }
}
pub struct ResolutionInputMetadata {
    pub accept: Option<String>,
    pub service_type: Option<String>,
    pub follow_redirect: Option<bool>,
    pub include_all_messages: bool,
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
        did: String,
        resolution_metadata: ResolutionInputMetadata,
    ) -> crate::Result<ResolutionResult> {
        let start_time = Instant::now();
        let did = match DID::parse_from_str(did.clone()) {
            Ok(did) => {
                if did.method_name != "iota" {
                    return Ok(ResolutionResult {
                        metadata: ResolutionMetadata {
                            duration: start_time.elapsed().as_millis(),
                            input_did: did.to_string(),
                            error: Some("not-supported".to_string()),
                            ..ResolutionMetadata::default()
                        },
                        ..ResolutionResult::default()
                    });
                }

                did
            }
            _ => {
                return Ok(ResolutionResult {
                    metadata: ResolutionMetadata {
                        duration: start_time.elapsed().as_millis(),
                        input_did: did.to_string(),
                        error: Some("invalid-did".to_string()),
                        ..ResolutionMetadata::default()
                    },
                    ..ResolutionResult::default()
                });
            }
        };

        let start_time = Instant::now();
        let mut metadata = HashMap::new();
        let nodes = get_nodes(&did, self.nodes.clone())?;
        let reader = TangleReader::new(nodes.to_vec());
        let messages = match reader.fetch(&get_iota_address(&did)?).await {
            Ok(messages) => messages,
            _ => {
                return Ok(ResolutionResult {
                    metadata: ResolutionMetadata {
                        duration: start_time.elapsed().as_millis(),
                        input_did: did.to_string(),
                        error: Some("not-found".to_string()),
                        ..ResolutionMetadata::default()
                    },
                    ..ResolutionResult::default()
                });
            }
        };
        let documents = get_ordered_documents(messages.clone(), &did)?;
        let diffs = get_ordered_diffs(messages.clone(), &did)?;
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
                let diff_document = DIDDocument::get_diff_from_str(diff.diff.diff.clone())?;
                latest_document.document = latest_document.document.merge(diff_document)?;
                metadata.insert(format!("diff_tail_transaction {}", i), diff.tailhash.clone());
            }
        }

        metadata.insert("document_tail_transaction".into(), latest_document.tailhash.clone());
        if resolution_metadata.accept != None {
            let result = ResolutionResult {
                did_document: Some(latest_document.document.clone()),
                did_document_string: Some(latest_document.document.to_string()),
                metadata: ResolutionMetadata {
                    duration: start_time.elapsed().as_millis(),
                    input_did: did.to_string(),
                    content_type: Some("ByteOrder::BigEndian".into()),
                    ..ResolutionMetadata::default()
                },
                did_document_metadata: Some(metadata),
            };
            return Ok(result);
        }
        let result = ResolutionResult {
            did_document: Some(latest_document.document.clone()),
            did_document_string: Some(latest_document.document.to_string()),
            metadata: ResolutionMetadata {
                duration: start_time.elapsed().as_millis(),
                input_did: did.to_string(),
                ..ResolutionMetadata::default()
            },
            did_document_metadata: Some(metadata),
        };
        Ok(result)
    }
    pub async fn resolve_stream<W: Write>(
        &self,
        did: String,
        resolution_metadata: ResolutionInputMetadata,
        buffer: &mut W,
    ) -> crate::Result<()> {
        let res = self.resolve(did, resolution_metadata).await?;
        match res.did_document_string {
            Some(document) => {
                document.write_to(buffer, ByteOrder::BigEndian)?;
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}

fn get_nodes(did: &DID, nodes: NetworkNodes) -> crate::Result<Vec<&'static str>> {
    let did_segments = &did.id_segments;
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
    Ok(nodes)
}

/// Order documents: first element is latest
fn get_ordered_documents(messages: HashMap<String, String>, did: &DID) -> crate::Result<Vec<HashWithMessage>> {
    let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
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
                    == iota_specific_idstring
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
fn get_ordered_diffs(messages: HashMap<String, String>, did: &DID) -> crate::Result<Vec<HashWithDiff>> {
    let iota_specific_idstring = did.id_segments.last().expect("Failed to get id_segment");
    let mut diffs: Vec<HashWithDiff> = messages
        .iter()
        .filter_map(|(tailhash, msg)| {
            if let Ok(diff) = serde_json::from_str::<Differences>(&msg) {
                if diff.did.id_segments.last().expect("Failed to get id_segment") == iota_specific_idstring {
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
