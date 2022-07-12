// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::diff::Diff;
use identity_core::diff::Error;
use identity_core::diff::Result;

use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::service::Service;
use crate::service::ServiceBuilder;
use crate::service::ServiceEndpoint;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffService<D = CoreDID, T = Object>
where
  D: DID + Diff,
  T: Diff,
{
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<<DIDUrl<D> as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  type_: Option<<OneOrSet<String> as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  service_endpoint: Option<<ServiceEndpoint as Diff>::Type>,
  #[serde(skip_serializing_if = "Option::is_none")]
  properties: Option<<T as Diff>::Type>,
}

impl<D, T> Diff for Service<D, T>
where
  D: Diff + DID + Serialize + for<'de> Deserialize<'de>,
  T: Diff + Serialize + for<'de> Deserialize<'de> + Default,
{
  type Type = DiffService<D, T>;

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
        Some(self.type_().diff(other.type_())?)
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
    let id: DIDUrl<D> = diff
      .id
      .map(|value| self.id().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.id().clone());

    let type_: OneOrSet<String> = diff
      .type_
      .map(|value| self.type_().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.type_().clone());

    let service_endpoint: ServiceEndpoint = diff
      .service_endpoint
      .map(|value| self.service_endpoint().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.service_endpoint().clone());

    let properties: T = diff
      .properties
      .map(|value| self.properties().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.properties().clone());

    // Use builder to enforce invariants.
    ServiceBuilder::new(properties)
      .id(id)
      .types(type_)
      .service_endpoint(service_endpoint)
      .build()
      .map_err(Error::merge)
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: DIDUrl<D> = diff
      .id
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.id`"))?;

    let type_: OneOrSet<String> = diff
      .type_
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.type_`"))?;

    let service_endpoint: ServiceEndpoint = diff
      .service_endpoint
      .map(Diff::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.service_endpoint`"))?;

    let properties: T = diff.properties.map(Diff::from_diff).transpose()?.unwrap_or_default();

    // Use builder to enforce invariants.
    ServiceBuilder::new(properties)
      .id(id)
      .types(type_)
      .service_endpoint(service_endpoint)
      .build()
      .map_err(Error::convert)
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffService {
      id: Some(self.id.into_diff()?),
      type_: Some(self.type_.into_diff()?),
      service_endpoint: Some(self.service_endpoint.into_diff()?),
      properties: if self.properties != T::default() {
        Some(self.properties.into_diff()?)
      } else {
        None
      },
    })
  }
}

impl Diff for ServiceEndpoint {
  type Type = ServiceEndpoint;

  fn diff(&self, other: &Self) -> identity_core::diff::Result<Self::Type> {
    if self != other {
      Ok(other.clone())
    } else {
      Ok(self.clone())
    }
  }

  fn merge(&self, diff: Self::Type) -> identity_core::diff::Result<Self> {
    if self != &diff {
      Ok(diff)
    } else {
      Ok(self.clone())
    }
  }

  fn from_diff(diff: Self::Type) -> identity_core::diff::Result<Self> {
    Ok(diff)
  }

  fn into_diff(self) -> identity_core::diff::Result<Self::Type> {
    Ok(self)
  }
}

#[cfg(test)]
mod test {
  use indexmap::IndexMap;

  use crate::did::CoreDIDUrl;
  use identity_core::common::Object;
  use identity_core::common::OrderedSet;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::diff::DiffString;
  use identity_core::diff::DiffVec;

  use super::*;

  fn controller() -> CoreDIDUrl {
    "did:example:1234#service".parse().unwrap()
  }

  fn service() -> Service {
    let controller = controller();
    Service::builder(Object::default())
      .id(controller)
      .service_endpoint(Url::parse("did:service:1234").unwrap())
      .type_("test_service")
      .build()
      .unwrap()
  }

  #[test]
  fn test_id() {
    let service = service();
    let mut new = service.clone();
    new.set_id("did:diff:123#new-service".parse().unwrap()).unwrap();

    let diff = service.diff(&new).unwrap();
    assert!(diff.properties.is_none());
    assert!(diff.service_endpoint.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(diff.id, Some(DiffString(Some("did:diff:123#new-service".to_string()))));
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_type_one() {
    let service = service();
    let mut new = service.clone();
    *new.type_mut() = OneOrSet::new_one("test_service_2".to_owned());

    let diff = service.diff(&new).unwrap();
    assert!(diff.properties.is_none());
    assert!(diff.service_endpoint.is_none());
    assert!(diff.id.is_none());
    assert!(diff.type_.is_some());
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_type_set() {
    let service = service();

    // One to set.
    let mut service_set = service.clone();
    *service_set.type_mut() =
      OneOrSet::try_from(vec!["test_service_set_1".to_owned(), "test_service_set_2".to_owned()]).unwrap();

    let diff_set = service.diff(&service_set).unwrap();
    assert!(diff_set.properties.is_none());
    assert!(diff_set.service_endpoint.is_none());
    assert!(diff_set.id.is_none());
    assert!(diff_set.type_.is_some());
    let merge = service.merge(diff_set).unwrap();
    assert_eq!(merge, service_set);

    // Set to one.
    let mut service_one = service_set.clone();
    *service_one.type_mut() = OneOrSet::new_one("one_service_type".to_owned());

    let diff_one = service_set.diff(&service_one).unwrap();
    assert!(diff_one.properties.is_none());
    assert!(diff_one.service_endpoint.is_none());
    assert!(diff_one.id.is_none());
    assert!(diff_one.type_.is_some());
    let merge = service_set.merge(diff_one).unwrap();
    assert_eq!(merge, service_one);
  }

  #[test]
  fn test_service_endpoint_one() {
    let service = service();
    let mut new = service.clone();
    let new_url = Url::parse("did:test:1234#service").unwrap();
    *new.service_endpoint_mut() = ServiceEndpoint::One(new_url.clone());

    let diff = service.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.properties.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(diff.service_endpoint, Some(ServiceEndpoint::One(new_url)),);
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_service_endpoint_set() {
    let service = service();

    let mut new = service.clone();
    let new_url_set = vec![
      Url::parse("https://example.com/").unwrap(),
      Url::parse("did:test:1234#service").unwrap(),
    ];
    *new.service_endpoint_mut() = ServiceEndpoint::Set(new_url_set.clone().try_into().unwrap());

    let diff = service.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.properties.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(
      diff.service_endpoint,
      Some(ServiceEndpoint::Set(new_url_set.try_into().unwrap())),
    );
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_service_endpoint_map() {
    let service = service();

    let mut new = service.clone();
    let mut new_url_map = IndexMap::new();
    new_url_map.insert(
      "origins".to_owned(),
      vec![
        Url::parse("https://example.com/").unwrap(),
        Url::parse("did:test:1234#service").unwrap(),
      ]
      .try_into()
      .unwrap(),
    );
    *new.service_endpoint_mut() = ServiceEndpoint::Map(new_url_map.clone());

    let diff = service.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.properties.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(diff.service_endpoint, Some(ServiceEndpoint::Map(new_url_map)),);
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_properties() {
    let mut service = service();
    service.properties.insert("key1".to_string(), "value1".into());
    let mut new = service.clone();

    // Replace properties.
    *new.properties_mut() = Object::default();

    assert_ne!(service, new);
    let diff = service.diff(&new).unwrap();
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_add_properties() {
    let service = service();
    let mut new = service.clone();

    // Update properties.
    assert!(new
      .properties_mut()
      .insert("key2".to_string(), "value2".into())
      .is_none());

    assert_ne!(service, new);
    let diff = service.diff(&new).unwrap();
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_from_into_diff() {
    let service: Service = service();

    let diff: DiffService = service.clone().into_diff().unwrap();
    let new: Service = Service::from_diff(diff.clone()).unwrap();
    assert_eq!(new, service);

    let ser: String = diff.to_json().unwrap();
    let de: DiffService = DiffService::from_json(&ser).unwrap();
    assert_eq!(diff, de);
    let from: Service = Service::from_diff(de).unwrap();
    assert_eq!(from, service);
  }

  #[test]
  fn test_serde() {
    let service = service();

    // Empty diff.
    {
      let diff: DiffService = service.clone().into_diff().unwrap();
      let ser: String = diff.to_json().unwrap();
      let de: DiffService = DiffService::from_json(&ser).unwrap();
      assert_eq!(diff, de);
    }

    // Updated fields.
    {
      let mut updated: Service = service.clone();
      updated.id = CoreDIDUrl::parse("did:test:serde").unwrap();
      updated.type_ = OneOrSet::new_one("TestSerde".to_owned());
      updated.service_endpoint = ServiceEndpoint::One(Url::parse("https://test.serde/").unwrap());
      updated.properties.insert("a".into(), 42.into());
      let diff: DiffService = Diff::diff(&service, &updated).unwrap();
      let ser: String = diff.to_json().unwrap();
      let de: DiffService = DiffService::from_json(&ser).unwrap();
      assert_eq!(diff, de);
    }
  }

  #[test]
  fn test_ordered_set_service_diff_serde() {
    let mut service = service();
    service.type_ = OneOrSet::from("".to_string());
    let set0: OrderedSet<Service> = OrderedSet::new();
    let set1: OrderedSet<Service> = OrderedSet::try_from(vec![service]).unwrap();

    let diff: DiffVec<Service> = Diff::diff(&set0, &set1).unwrap();
    let merge: OrderedSet<Service> = set0.merge(diff.clone()).unwrap();
    assert_eq!(merge, set1);

    let ser: String = diff.to_json().unwrap();
    let de: DiffVec<Service> = DiffVec::from_json(&ser).unwrap();
    assert_eq!(diff, de);
  }
}
