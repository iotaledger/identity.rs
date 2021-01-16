use core::convert::TryFrom as _;
use did_doc::{
    url::Url, DIDKey, Document, DocumentBuilder, Method, MethodBuilder, MethodData, MethodRef, MethodType, Object,
    OrderedSet, Service, ServiceBuilder,
};
use did_url::DID;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    hashmap::DiffHashMap,
    string::DiffString,
    traits::Diff,
    vec::DiffVec,
};

// =============================================================================
// =============================================================================

impl Diff for DID {
    type Type = DiffString;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        self.to_string().diff(&other.to_string())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        self.to_string()
            .merge(diff)
            .and_then(|this| Self::parse(&this).map_err(Error::merge))
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(Error::convert))
    }

    fn into_diff(self) -> Result<Self::Type> {
        self.to_string().into_diff()
    }
}

// =============================================================================
// =============================================================================

impl Diff for Url {
    type Type = DiffString;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        self.to_string().diff(&other.to_string())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        self.to_string()
            .merge(diff)
            .and_then(|this| Self::parse(&this).map_err(Error::merge))
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(Error::convert))
    }

    fn into_diff(self) -> Result<Self::Type> {
        self.to_string().into_diff()
    }
}

// =============================================================================
// =============================================================================

impl<T> Diff for OrderedSet<T>
where
    T: Diff + Serialize + for<'de> Deserialize<'de>,
{
    type Type = DiffVec<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        self.clone().into_vec().diff(&other.clone().into_vec())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        self.clone()
            .into_vec()
            .merge(diff)
            .and_then(|this| Self::try_from(this).map_err(Error::merge))
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        Vec::from_diff(diff).and_then(|this| Self::try_from(this).map_err(Error::convert))
    }

    fn into_diff(self) -> Result<Self::Type> {
        self.into_vec().into_diff()
    }
}

// =============================================================================
// =============================================================================

impl<T> Diff for DIDKey<T>
where
    T: AsRef<DID> + Diff,
{
    type Type = <T as Diff>::Type;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        self.clone().into_inner().diff(&other.clone().into_inner())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        self.clone().into_inner().merge(diff).map(Self::new)
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        T::from_diff(diff).map(Self::new)
    }

    fn into_diff(self) -> Result<Self::Type> {
        self.into_inner().into_diff()
    }
}

// =============================================================================
// =============================================================================

pub type DiffObject = DiffHashMap<String, Value>;
pub type MapProxy = HashMap<String, Value>;

impl Diff for Object {
    type Type = DiffObject;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        let a: MapProxy = self.clone().into_iter().collect();
        let b: MapProxy = other.clone().into_iter().collect();

        a.diff(&b)
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        let this: MapProxy = self.clone().into_iter().collect();
        let this: MapProxy = this.merge(diff)?;

        Ok(this.into_iter().collect())
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        Ok(MapProxy::from_diff(diff)?.into_iter().collect())
    }

    fn into_diff(self) -> Result<Self::Type> {
        self.into_iter().collect::<MapProxy>().into_diff()
    }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(bound(deserialize = ""))]
pub struct DiffDocument<T = Object, U = Object, V = Object>
where
    T: Diff + Serialize + for<'__de> Deserialize<'__de>,
    U: Diff + Serialize + for<'__de> Deserialize<'__de>,
    V: Diff + Serialize + for<'__de> Deserialize<'__de>,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<Option<DiffString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    also_known_as: Option<DiffVec<Url>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_method: Option<DiffVec<DIDKey<Method<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<DiffVec<DIDKey<MethodRef<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assertion_method: Option<DiffVec<DIDKey<MethodRef<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_agreement: Option<DiffVec<DIDKey<MethodRef<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capability_delegation: Option<DiffVec<DIDKey<MethodRef<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capability_invocation: Option<DiffVec<DIDKey<MethodRef<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<DiffVec<Service<V>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<<T as Diff>::Type>,
}

impl<T, U, V> Diff for Document<T, U, V>
where
    T: Diff + Serialize + for<'de> Deserialize<'de>,
    U: Diff + Serialize + for<'de> Deserialize<'de>,
    V: Diff + Serialize + for<'de> Deserialize<'de>,
{
    type Type = DiffDocument<T, U, V>;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        Ok(DiffDocument {
            id: if self.id() == other.id() {
                None
            } else {
                Some(self.id().diff(other.id())?)
            },
            controller: if self.controller() == other.controller() {
                None
            } else {
                match (self.controller(), other.controller()) {
                    (Some(a), Some(b)) => Some(Some(a.diff(&b)?)),
                    (None, Some(b)) => Some(Some(b.clone().into_diff()?)),
                    _ => Some(None),
                }
            },
            also_known_as: if self.also_known_as() == other.also_known_as() {
                None
            } else {
                Some(self.also_known_as().to_vec().diff(&other.also_known_as().to_vec())?)
            },
            verification_method: if self.verification_method() == other.verification_method() {
                None
            } else {
                Some(self.verification_method().diff(other.verification_method())?)
            },
            authentication: if self.authentication() == other.authentication() {
                None
            } else {
                Some(self.authentication().diff(other.authentication())?)
            },
            assertion_method: if self.assertion_method() == other.assertion_method() {
                None
            } else {
                Some(self.assertion_method().diff(other.assertion_method())?)
            },
            key_agreement: if self.key_agreement() == other.key_agreement() {
                None
            } else {
                Some(self.key_agreement().diff(other.key_agreement())?)
            },
            capability_delegation: if self.capability_delegation() == other.capability_delegation() {
                None
            } else {
                Some(self.capability_delegation().diff(other.capability_delegation())?)
            },
            capability_invocation: if self.capability_invocation() == other.capability_invocation() {
                None
            } else {
                Some(self.capability_invocation().diff(other.capability_invocation())?)
            },
            service: if self.service() == other.service() {
                None
            } else {
                Some(self.service().to_vec().diff(&other.service().to_vec())?)
            },
            properties: if self.properties() == other.properties() {
                None
            } else {
                Some(self.properties().diff(other.properties())?)
            },
        })
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(|value| self.id().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.id().clone());

        let controller: Option<DID> = diff
            .controller
            .flatten()
            .and_then(|value| self.controller().map(|controller| controller.merge(value)))
            .transpose()?;

        let also_known_as: Vec<Url> = diff
            .also_known_as
            .map(|value| self.also_known_as().to_vec().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.also_known_as().to_vec());

        let verification_method: OrderedSet<DIDKey<Method<U>>> = diff
            .verification_method
            .map(|value| self.verification_method().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.verification_method().clone());

        let authentication: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .authentication
            .map(|value| self.authentication().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.authentication().clone());

        let assertion_method: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .assertion_method
            .map(|value| self.assertion_method().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.assertion_method().clone());

        let key_agreement: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .key_agreement
            .map(|value| self.key_agreement().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.key_agreement().clone());

        let capability_delegation: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .capability_delegation
            .map(|value| self.capability_delegation().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.capability_delegation().clone());

        let capability_invocation: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .capability_invocation
            .map(|value| self.capability_invocation().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.capability_invocation().clone());

        let service: Vec<Service<V>> = diff
            .service
            .map(|value| self.service().to_vec().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.service().to_vec());

        let properties: T = diff
            .properties
            .map(|value| self.properties().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.properties().clone());

        let mut builder: DocumentBuilder<T, U, V> = DocumentBuilder::new(properties);

        builder = builder.id(id);

        if let Some(controller) = controller {
            builder = builder.controller(controller);
        }

        for element in also_known_as {
            builder = builder.also_known_as(element);
        }

        for element in verification_method.to_vec() {
            builder = builder.verification_method(element.into_inner());
        }

        for element in authentication.to_vec() {
            builder = builder.authentication(element.into_inner());
        }

        for element in assertion_method.to_vec() {
            builder = builder.assertion_method(element.into_inner());
        }

        for element in key_agreement.to_vec() {
            builder = builder.key_agreement(element.into_inner());
        }

        for element in capability_delegation.to_vec() {
            builder = builder.capability_delegation(element.into_inner());
        }

        for element in capability_invocation.to_vec() {
            builder = builder.capability_invocation(element.into_inner());
        }

        for element in service {
            builder = builder.service(element);
        }

        builder.build().map_err(Error::convert)
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(DID::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `id`"))?;

        let controller: Option<DID> = diff
            .controller
            .map(|diff| match diff {
                Some(diff) => Some(DID::from_diff(diff)).transpose(),
                None => Ok(None),
            })
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `controller`"))?;

        let also_known_as: Vec<Url> = diff
            .also_known_as
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `also_known_as`"))?;

        let verification_method: OrderedSet<DIDKey<Method<U>>> = diff
            .verification_method
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `verification_method`"))?;

        let authentication: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .authentication
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `authentication`"))?;

        let assertion_method: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .assertion_method
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `assertion_method`"))?;

        let key_agreement: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .key_agreement
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `key_agreement`"))?;

        let capability_delegation: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .capability_delegation
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `capability_delegation`"))?;

        let capability_invocation: OrderedSet<DIDKey<MethodRef<U>>> = diff
            .capability_invocation
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `capability_invocation`"))?;

        let service: Vec<Service<V>> = diff
            .service
            .map(Diff::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `service`"))?;

        let properties: T = diff
            .properties
            .map(T::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `properties`"))?;

        let mut builder: DocumentBuilder<T, U, V> = DocumentBuilder::new(properties);

        builder = builder.id(id);

        if let Some(controller) = controller {
            builder = builder.controller(controller);
        }

        for element in also_known_as {
            builder = builder.also_known_as(element);
        }

        for element in verification_method.to_vec() {
            builder = builder.verification_method(element.into_inner());
        }

        for element in authentication.to_vec() {
            builder = builder.authentication(element.into_inner());
        }

        for element in assertion_method.to_vec() {
            builder = builder.assertion_method(element.into_inner());
        }

        for element in key_agreement.to_vec() {
            builder = builder.key_agreement(element.into_inner());
        }

        for element in capability_delegation.to_vec() {
            builder = builder.capability_delegation(element.into_inner());
        }

        for element in capability_invocation.to_vec() {
            builder = builder.capability_invocation(element.into_inner());
        }

        for element in service {
            builder = builder.service(element);
        }

        builder.build().map_err(Error::convert)
    }

    fn into_diff(self) -> Result<Self::Type> {
        Ok(DiffDocument {
            id: Some(self.id().clone().into_diff()?),
            controller: Some(self.controller().cloned().map(|value| value.into_diff()).transpose()?),
            also_known_as: Some(self.also_known_as().to_vec().into_diff()?),
            verification_method: Some(self.verification_method().to_vec().into_diff()?),
            authentication: Some(self.authentication().to_vec().into_diff()?),
            assertion_method: Some(self.assertion_method().to_vec().into_diff()?),
            key_agreement: Some(self.key_agreement().to_vec().into_diff()?),
            capability_delegation: Some(self.capability_delegation().to_vec().into_diff()?),
            capability_invocation: Some(self.capability_invocation().to_vec().into_diff()?),
            service: Some(self.service().to_vec().into_diff()?),
            properties: Some(self.properties().clone().into_diff()?),
        })
    }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMethod<T = Object>
where
    T: Diff,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_type: Option<MethodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_data: Option<DiffMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<<T as Diff>::Type>,
}

impl<T> Diff for Method<T>
where
    T: Diff + Serialize + for<'de> Deserialize<'de>,
{
    type Type = DiffMethod<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        Ok(DiffMethod {
            id: if self.id() == other.id() {
                None
            } else {
                Some(self.id().diff(other.id())?)
            },
            controller: if self.controller() == other.controller() {
                None
            } else {
                Some(self.controller().diff(other.controller())?)
            },
            key_type: if self.key_type() == other.key_type() {
                None
            } else {
                Some(self.key_type().diff(&other.key_type())?)
            },
            key_data: if self.key_data() == other.key_data() {
                None
            } else {
                Some(self.key_data().diff(other.key_data())?)
            },
            properties: if self.properties() == other.properties() {
                None
            } else {
                Some(self.properties().diff(other.properties())?)
            },
        })
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(|value| self.id().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.id().clone());

        let controller: DID = diff
            .controller
            .map(|value| self.controller().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.controller().clone());

        let key_data: MethodData = diff
            .key_data
            .map(|value| self.key_data().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.key_data().clone());

        let key_type: MethodType = diff
            .key_type
            .map(|value| self.key_type().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.key_type());

        let properties: T = diff
            .properties
            .map(|value| self.properties().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.properties().clone());

        MethodBuilder::new(properties)
            .id(id)
            .controller(controller)
            .key_type(key_type)
            .key_data(key_data)
            .build()
            .map_err(Error::merge)
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(DID::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `id`"))?;

        let controller: DID = diff
            .controller
            .map(DID::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `controller`"))?;

        let key_type: MethodType = diff
            .key_type
            .map(MethodType::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `key_type`"))?;

        let key_data: MethodData = diff
            .key_data
            .map(MethodData::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `key_data`"))?;

        let properties: T = diff
            .properties
            .map(T::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `properties`"))?;

        MethodBuilder::new(properties)
            .id(id)
            .controller(controller)
            .key_type(key_type)
            .key_data(key_data)
            .build()
            .map_err(Error::convert)
    }

    fn into_diff(self) -> Result<Self::Type> {
        Ok(DiffMethod {
            id: Some(self.id().clone().into_diff()?),
            controller: Some(self.controller().clone().into_diff()?),
            key_type: Some(self.key_type().into_diff()?),
            key_data: Some(self.key_data().clone().into_diff()?),
            properties: Some(self.properties().clone().into_diff()?),
        })
    }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DiffMethodRef<T = Object>
where
    T: Diff,
{
    Embed(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffMethod<T>>),
    Refer(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffString>),
}

impl<T> Diff for MethodRef<T>
where
    T: Diff + Serialize + for<'de> Deserialize<'de>,
{
    type Type = DiffMethodRef<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        match (self, other) {
            (Self::Embed(a), Self::Embed(b)) if a == b => Ok(DiffMethodRef::Embed(None)),
            (Self::Embed(a), Self::Embed(b)) => a.diff(b).map(Some).map(DiffMethodRef::Embed),
            (Self::Refer(a), Self::Refer(b)) if a == b => Ok(DiffMethodRef::Refer(None)),
            (Self::Refer(a), Self::Refer(b)) => a.diff(b).map(Some).map(DiffMethodRef::Refer),
            (_, _) => other.clone().into_diff(),
        }
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        match (self, diff) {
            (Self::Embed(a), DiffMethodRef::Embed(Some(ref b))) => a.merge(b.clone()).map(Self::Embed),
            (Self::Embed(a), DiffMethodRef::Embed(None)) => Ok(Self::Embed(a.clone())),
            (Self::Refer(a), DiffMethodRef::Refer(Some(ref b))) => a.merge(b.clone()).map(Self::Refer),
            (Self::Refer(a), DiffMethodRef::Refer(None)) => Ok(Self::Refer(a.clone())),
            (_, diff) => Self::from_diff(diff),
        }
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        match diff {
            DiffMethodRef::Embed(Some(value)) => Diff::from_diff(value).map(Self::Embed),
            DiffMethodRef::Embed(None) => Err(Error::convert("Invalid MethodRef Diff")),
            DiffMethodRef::Refer(Some(value)) => Diff::from_diff(value).map(Self::Refer),
            DiffMethodRef::Refer(None) => Err(Error::convert("Invalid MethodRef Diff")),
        }
    }

    fn into_diff(self) -> Result<Self::Type> {
        match self {
            Self::Embed(value) => value.into_diff().map(Some).map(DiffMethodRef::Embed),
            Self::Refer(value) => value.into_diff().map(Some).map(DiffMethodRef::Refer),
        }
    }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DiffMethodData {
    PublicKeyBase58(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffString>),
    PublicKeyHex(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffString>),
    PublicKeyJwk(#[serde(skip_serializing_if = "Option::is_none")] Option<DiffObject>),
}

impl Diff for MethodData {
    type Type = DiffMethodData;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        match (self, other) {
            (Self::PublicKeyBase58(a), Self::PublicKeyBase58(b)) if a == b => Ok(DiffMethodData::PublicKeyBase58(None)),
            (Self::PublicKeyBase58(a), Self::PublicKeyBase58(b)) => {
                a.diff(b).map(Some).map(DiffMethodData::PublicKeyBase58)
            }
            (Self::PublicKeyHex(a), Self::PublicKeyHex(b)) if a == b => Ok(DiffMethodData::PublicKeyHex(None)),
            (Self::PublicKeyHex(a), Self::PublicKeyHex(b)) => a.diff(b).map(Some).map(DiffMethodData::PublicKeyHex),
            (Self::PublicKeyJwk(a), Self::PublicKeyJwk(b)) if a == b => Ok(DiffMethodData::PublicKeyJwk(None)),
            (Self::PublicKeyJwk(a), Self::PublicKeyJwk(b)) => a.diff(b).map(Some).map(DiffMethodData::PublicKeyJwk),
            (_, _) => other.clone().into_diff(),
        }
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        match (self, diff) {
            (Self::PublicKeyBase58(a), DiffMethodData::PublicKeyBase58(Some(ref b))) => {
                a.merge(b.clone()).map(Self::PublicKeyBase58)
            }
            (Self::PublicKeyBase58(a), DiffMethodData::PublicKeyBase58(None)) => Ok(Self::PublicKeyBase58(a.clone())),
            (Self::PublicKeyHex(a), DiffMethodData::PublicKeyHex(Some(ref b))) => {
                a.merge(b.clone()).map(Self::PublicKeyHex)
            }
            (Self::PublicKeyHex(a), DiffMethodData::PublicKeyHex(None)) => Ok(Self::PublicKeyHex(a.clone())),
            (Self::PublicKeyJwk(a), DiffMethodData::PublicKeyJwk(Some(ref b))) => {
                a.merge(b.clone()).map(Self::PublicKeyJwk)
            }
            (Self::PublicKeyJwk(a), DiffMethodData::PublicKeyJwk(None)) => Ok(Self::PublicKeyJwk(a.clone())),
            (_, diff) => Self::from_diff(diff),
        }
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        match diff {
            DiffMethodData::PublicKeyBase58(Some(value)) => Diff::from_diff(value).map(Self::PublicKeyBase58),
            DiffMethodData::PublicKeyBase58(None) => Ok(Self::PublicKeyBase58(Default::default())),
            DiffMethodData::PublicKeyHex(Some(value)) => Diff::from_diff(value).map(Self::PublicKeyHex),
            DiffMethodData::PublicKeyHex(None) => Ok(Self::PublicKeyHex(Default::default())),
            DiffMethodData::PublicKeyJwk(Some(value)) => Diff::from_diff(value).map(Self::PublicKeyJwk),
            DiffMethodData::PublicKeyJwk(None) => Ok(Self::PublicKeyJwk(Default::default())),
        }
    }

    fn into_diff(self) -> Result<Self::Type> {
        match self {
            Self::PublicKeyBase58(value) => value.into_diff().map(Some).map(DiffMethodData::PublicKeyBase58),
            Self::PublicKeyHex(value) => value.into_diff().map(Some).map(DiffMethodData::PublicKeyHex),
            Self::PublicKeyJwk(value) => value.into_diff().map(Some).map(DiffMethodData::PublicKeyJwk),
            _ => Err(Error::convert("Unknown Method Data Variant")),
        }
    }
}

// =============================================================================
// =============================================================================

impl Diff for MethodType {
    type Type = MethodType;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        Ok(*other)
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        Ok(diff)
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        Ok(diff)
    }

    fn into_diff(self) -> Result<Self::Type> {
        Ok(self)
    }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffService<T = Object>
where
    T: Diff,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_endpoint: Option<DiffString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<<T as Diff>::Type>,
}

impl<T> Diff for Service<T>
where
    T: Diff + Serialize + for<'de> Deserialize<'de>,
{
    type Type = DiffService<T>;

    fn diff(&self, other: &Self) -> Result<Self::Type> {
        Ok(DiffService {
            id: if self.id() == other.id() {
                None
            } else {
                Some(self.id().diff(other.id())?)
            },
            type_: if self.type_() == other.type_() {
                None
            } else {
                Some(self.type_().to_string().diff(&other.type_().to_string())?)
            },
            service_endpoint: if self.service_endpoint() == other.service_endpoint() {
                None
            } else {
                Some(self.service_endpoint().diff(other.service_endpoint())?)
            },
            properties: if self.properties() == other.properties() {
                None
            } else {
                Some(self.properties().diff(other.properties())?)
            },
        })
    }

    fn merge(&self, diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(|value| self.id().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.id().clone());

        let type_: String = diff
            .type_
            .map(|value| self.type_().to_string().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.type_().to_string());

        let service_endpoint: Url = diff
            .service_endpoint
            .map(|value| self.service_endpoint().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.service_endpoint().clone());

        let properties: T = diff
            .properties
            .map(|value| self.properties().merge(value))
            .transpose()?
            .unwrap_or_else(|| self.properties().clone());

        ServiceBuilder::new(properties)
            .id(id)
            .type_(type_)
            .service_endpoint(service_endpoint)
            .build()
            .map_err(Error::merge)
    }

    fn from_diff(diff: Self::Type) -> Result<Self> {
        let id: DID = diff
            .id
            .map(DID::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `id`"))?;

        let type_: String = diff
            .type_
            .map(String::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `type_`"))?;

        let service_endpoint: Url = diff
            .service_endpoint
            .map(Url::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `service_endpoint`"))?;

        let properties: T = diff
            .properties
            .map(T::from_diff)
            .transpose()?
            .ok_or_else(|| Error::convert("Missing field `properties`"))?;

        ServiceBuilder::new(properties)
            .id(id)
            .type_(type_)
            .service_endpoint(service_endpoint)
            .build()
            .map_err(Error::convert)
    }

    fn into_diff(self) -> Result<Self::Type> {
        Ok(DiffService {
            id: Some(self.id().clone().into_diff()?),
            type_: Some(self.type_().to_string().into_diff()?),
            service_endpoint: Some(self.service_endpoint().clone().into_diff()?),
            properties: Some(self.properties().clone().into_diff()?),
        })
    }
}
