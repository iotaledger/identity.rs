use serde::{Deserialize, Serialize};

use crate::{
    common::Url,
    did::{Authentication, DIDDocument, PublicKey, Service},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
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
    Document(DIDDocument),
    Service(Url),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SecondaryResource {
    PublicKey(PublicKey),
    Authentication(Authentication),
    Service(Service),
}
