use crate::resolver::{NetworkNodes, ResolutionInputMetadata, ResolutionMetadata, ResolutionResult, Resolver};
use identity_core::{
    common::Timestamp,
    did::DID,
    document::DIDDocument,
    utils::{Authentication, PublicKey, Service},
};
use percent_encoding::percent_decode_str;
use std::time::Instant;
use url::{Position, Url};

pub struct DereferenceResult {
    pub resolver_result: ResolutionResult,
    pub dereference_result: DereferenceRawResult,
    pub metadata: ResolutionMetadata,
}
pub enum DereferenceRawResult {
    Stringresult(String),
    Property(Box<Property>),
}

pub struct Dereferencer {
    pub nodes: NetworkNodes,
}

impl Dereferencer {
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
    pub async fn dereference(
        &self,
        did_url: String,
        input_metadata: ResolutionInputMetadata,
    ) -> crate::Result<DereferenceResult> {
        let start_time = Instant::now();
        let resolver = Resolver::new(self.nodes.clone())?;
        let resolver_result = resolver
            .resolve(DID::parse_from_str(did_url.clone())?, input_metadata)
            .await?;
        let deref_result = dereference_raw(did_url, resolver_result.did_document.clone())?;
        Ok(DereferenceResult {
            resolver_result,
            dereference_result: deref_result,
            metadata: ResolutionMetadata {
                driver_id: "did:iota".into(),
                retrieved: Timestamp::now().to_rfc3339(),
                duration: start_time.elapsed().as_millis(),
            },
        })
    }
}

pub fn dereference_raw(did_url: String, did_document: DIDDocument) -> crate::Result<DereferenceRawResult> {
    let did_url = DID::parse_from_str(did_url)?;
    if let Some(query) = did_url.clone().query {
        let mut output_url;
        let property = get_fragment_property(
            did_document,
            query[0]
                .value
                .as_ref()
                .expect("Couldn't get value from DID URL")
                .to_string(),
        )?;
        match property {
            Property::Service(service) => {
                let parsed = Url::parse(&service.endpoint.context.as_inner()[0].clone())?;
                output_url = parsed[..Position::AfterPath].to_string();
            }
            _ => return Err(crate::Error::DereferencingError),
        };
        if let Some(path_segments) = did_url.clone().path_segments {
            output_url = format!("{}{}", output_url, path_segments[0].clone());
        }
        if query.len() > 1 {
            let decoded_val = percent_decode_str(&query[1].value.as_ref().expect("Couldn't get value from DID URL"));
            output_url = format!("{}{}", output_url, decoded_val.decode_utf8()?,);
        }
        if let Some(fragment) = did_url.fragment.clone() {
            output_url = format!("{}#{}", output_url, fragment);
        };
        return Ok(DereferenceRawResult::Stringresult(output_url));
    }
    if let Some(fragment) = did_url.fragment {
        let property = get_fragment_property(did_document, fragment)?;
        return Ok(DereferenceRawResult::Property(Box::new(property)));
    }

    Err(crate::Error::DereferencingError)
}

pub enum Property {
    PublicKey(PublicKey),
    Service(Service),
    Authentication(Authentication),
}

fn get_fragment_property(did_document: DIDDocument, fragment: String) -> crate::Result<Property> {
    // pub public_key: Vec<PublicKey>,
    if !did_document.public_key.is_empty() {
        for property in did_document.public_key {
            if property.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                // println!("Fragment found! {:?}", property);
                return Ok(Property::PublicKey(property));
            }
        }
    }
    // pub auth: Vec<Authentication>,
    if !did_document.auth.is_empty() {
        for property in did_document.auth {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub assert: Vec<Authentication>,
    if !did_document.assert.is_empty() {
        for property in did_document.assert {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub verification: Vec<Authentication>,
    if !did_document.verification.is_empty() {
        for property in did_document.verification {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub delegation: Vec<Authentication>,
    if !did_document.delegation.is_empty() {
        for property in did_document.delegation {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub invocation: Vec<Authentication>,
    if !did_document.invocation.is_empty() {
        for property in did_document.invocation {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub agreement: Vec<Authentication>,
    if !did_document.agreement.is_empty() {
        for property in did_document.agreement {
            match &property {
                Authentication::Method(subj) => {
                    if subj.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
                Authentication::Key(key) => {
                    if key.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                        return Ok(Property::Authentication(property));
                    }
                }
            }
        }
    }
    // // pub services: Vec<Service>,
    if !did_document.services.is_empty() {
        for property in did_document.services {
            if property.id.to_did()?.fragment.expect("Couldn't get fragment") == fragment {
                // println!("Fragment found! {:?}", property);
                return Ok(Property::Service(property));
            }
        }
    }

    Err(crate::Error::DereferencingError)
}
