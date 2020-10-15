use serde::{Deserialize, Serialize};

use crate::{
    common::Url,
    did::{Authentication, DIDDocument as Document, Service, DID},
    key::PublicKey,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Resource {
    Primary(PrimaryResource),
    Secondary(SecondaryResource),
}

impl From<PrimaryResource> for Resource {
    fn from(other: PrimaryResource) -> Self {
        Self::Primary(other)
    }
}

impl From<SecondaryResource> for Resource {
    fn from(other: SecondaryResource) -> Self {
        Self::Secondary(other)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrimaryResource {
    Document(Document),
    Service(Url),
}

impl From<Document> for PrimaryResource {
    fn from(other: Document) -> Self {
        Self::Document(other)
    }
}

impl From<Url> for PrimaryResource {
    fn from(other: Url) -> Self {
        Self::Service(other)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SecondaryResource {
    VerificationDID(DID),
    VerificationKey(PublicKey),
    Service(Service),
}

impl From<DID> for SecondaryResource {
    fn from(other: DID) -> Self {
        Self::VerificationDID(other)
    }
}

impl From<PublicKey> for SecondaryResource {
    fn from(other: PublicKey) -> Self {
        Self::VerificationKey(other)
    }
}

impl From<Authentication> for SecondaryResource {
    fn from(other: Authentication) -> Self {
        match other {
            Authentication::DID(inner) => inner.into(),
            Authentication::Key(inner) => inner.into(),
        }
    }
}

impl From<Service> for SecondaryResource {
    fn from(other: Service) -> Self {
        Self::Service(other)
    }
}
