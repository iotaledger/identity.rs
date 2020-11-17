use did_doc::{url::Url, DIDKey, Document, Method, MethodRef, Service};
use did_url::DID;
use serde::{Deserialize, Serialize};

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

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SecondaryResource {
    VerificationDID(DID),
    VerificationKey(Method),
    Service(Service),
}

impl From<DID> for SecondaryResource {
    fn from(other: DID) -> Self {
        Self::VerificationDID(other)
    }
}

impl From<Method> for SecondaryResource {
    fn from(other: Method) -> Self {
        Self::VerificationKey(other)
    }
}

impl From<MethodRef> for SecondaryResource {
    fn from(other: MethodRef) -> Self {
        match other {
            MethodRef::Refer(inner) => inner.into(),
            MethodRef::Embed(inner) => inner.into(),
        }
    }
}

impl From<DIDKey<Method>> for SecondaryResource {
    fn from(other: DIDKey<Method>) -> Self {
        other.into_inner().into()
    }
}

impl From<DIDKey<MethodRef>> for SecondaryResource {
    fn from(other: DIDKey<MethodRef>) -> Self {
        other.into_inner().into()
    }
}

impl From<Service> for SecondaryResource {
    fn from(other: Service) -> Self {
        Self::Service(other)
    }
}
