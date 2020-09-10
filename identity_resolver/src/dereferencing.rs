use crate::resolver::{NetworkNodes, ResolutionInputMetadata, ResolutionMetadata, ResolutionResult, Resolver};
use identity_core::{
    did::{Param, DID},
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
        let resolver_result = resolver.resolve(did_url.clone(), input_metadata).await?;
        let deref_result = dereference_raw(did_url.clone(), resolver_result.did_document.clone().unwrap())?;
        Ok(DereferenceResult {
            resolver_result,
            dereference_result: deref_result,
            metadata: ResolutionMetadata {
                duration: start_time.elapsed().as_millis(),
                input_did: did_url,
                ..ResolutionMetadata::default()
            },
        })
    }
}

pub fn dereference_raw(did_url: String, did_document: DIDDocument) -> crate::Result<DereferenceRawResult> {
    let did_url = DID::parse_from_str(did_url)?;
    if let Some(query) = did_url.clone().query {
        let service_endpoint_url = service_endpoint_construction(did_url, query, did_document)?;
        return Ok(DereferenceRawResult::Stringresult(service_endpoint_url));
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

fn service_endpoint_construction(did_url: DID, query: Vec<Param>, did_document: DIDDocument) -> crate::Result<String> {
    let mut service_endpoint_url = String::new();
    let mut relative_ref = String::new();
    for param in query.clone() {
        if param.key == "service" && !did_document.clone().services.is_empty() {
            for service in did_document.clone().services {
                if &service.id.to_did()?.fragment.expect("Couldn't get fragment")
                    == param.value.as_ref().expect("Couldn't get param value from DID URL")
                {
                    let parsed = Url::parse(&service.endpoint.context.as_inner()[0].clone())?;
                    service_endpoint_url = parsed[..Position::AfterPath].to_string();
                }
            }
        }
        if param.key == "relative-ref" {
            let decoded_val = percent_decode_str(&param.value.as_ref().expect("Couldn't get param valuefrom DID URL"));
            relative_ref = decoded_val.decode_utf8()?.into_owned();
        }
    }
    if let Some(path_segments) = did_url.clone().path_segments {
        if service_endpoint_url != "" {
            service_endpoint_url = format!("{}{}", service_endpoint_url, path_segments[0].clone());
        }
    }
    if service_endpoint_url != "" && relative_ref != "" {
        service_endpoint_url = format!("{}{}", service_endpoint_url, relative_ref);
    }
    if let Some(fragment) = did_url.fragment {
        service_endpoint_url = format!("{}#{}", service_endpoint_url, fragment);
    };

    Ok(service_endpoint_url)
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
