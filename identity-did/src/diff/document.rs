// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::DiffVec;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::did::DID;
use crate::document::Document;
use crate::service::Service;
use crate::utils::DIDKey;
use crate::utils::OrderedSet;
use crate::verification::Method;
use crate::verification::MethodRef;

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
  service: Option<DiffVec<DIDKey<Service<V>>>>,
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
        Some(self.service().diff(&other.service())?)
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

    let service: OrderedSet<DIDKey<Service<V>>> = diff
      .service
      .map(|value| self.service().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.service().clone());

    let properties: T = diff
      .properties
      .map(|value| self.properties().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties().clone());

    Ok(Document {
      id,
      controller,
      also_known_as,
      verification_method,
      authentication,
      assertion_method,
      key_agreement,
      capability_delegation,
      capability_invocation,
      service,
      properties,
    })
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

    let service: OrderedSet<DIDKey<Service<V>>> = diff
      .service
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service`"))?;

    let properties: T = diff
      .properties
      .map(T::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `properties`"))?;

    Ok(Document {
      id,
      controller,
      also_known_as,
      verification_method,
      authentication,
      assertion_method,
      key_agreement,
      capability_delegation,
      capability_invocation,
      service,
      properties,
    })
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
