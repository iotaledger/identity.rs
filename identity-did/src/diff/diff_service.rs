// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::did::DID;
use crate::service::Service;

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
  T: Diff + Serialize + for<'de> Deserialize<'de> + Default,
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

    Ok(Service {
      id,
      type_,
      service_endpoint,
      properties,
    })
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let id: DID = diff
      .id
      .map(DID::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.id`"))?;

    let type_: String = diff
      .type_
      .map(String::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.type_`"))?;

    let service_endpoint: Url = diff
      .service_endpoint
      .map(Url::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `service.service_endpoint`"))?;

    let properties: T = diff.properties.map(T::from_diff).transpose()?.unwrap_or_default();

    Ok(Service {
      id,
      type_,
      service_endpoint,
      properties,
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffService {
      id: Some(self.id().to_string().into_diff()?),
      type_: Some(self.type_().to_string().into_diff()?),
      service_endpoint: Some(self.service_endpoint().to_string().into_diff()?),
      properties: Some(self.properties().clone().into_diff()?),
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use identity_core::common::Object;

  fn controller() -> DID {
    "did:example:1234".parse().unwrap()
  }

  fn service() -> Service {
    let controller = controller();
    let mut properties: Object = Object::default();
    properties.insert("key1".to_string(), "value1".into());
    Service::builder(properties)
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
    *new.id_mut() = "did:diff:123".parse().unwrap();

    let diff = service.diff(&new).unwrap();
    assert!(diff.properties.is_none());
    assert!(diff.service_endpoint.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(diff.id, Some(DiffString(Some("did:diff:123".to_string()))));
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_type() {
    let service = service();
    let mut new = service.clone();
    *new.type_mut() = "test_service_2".parse().unwrap();

    let diff = service.diff(&new).unwrap();
    assert!(diff.properties.is_none());
    assert!(diff.service_endpoint.is_none());
    assert!(diff.id.is_none());
    assert_eq!(diff.type_, Some(DiffString(Some("test_service_2".to_string()))));
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_service_endpoint() {
    let service = service();
    let mut new = service.clone();
    let new_url = "did:test:1234".to_string();
    *new.service_endpoint_mut() = Url::parse(new_url.clone()).unwrap();

    let diff = service.diff(&new).unwrap();
    assert!(diff.id.is_none());
    assert!(diff.properties.is_none());
    assert!(diff.type_.is_none());
    assert_eq!(diff.service_endpoint, Some(DiffString(Some(new_url))));
    let merge = service.merge(diff).unwrap();
    assert_eq!(merge, new);
  }

  #[test]
  fn test_replace_properties() {
    let service = service();
    let mut new = service.clone();

    // update properties
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

    // update properties
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
  fn test_from_into_roundtrip() {
    let service = service();

    let diff = service.clone().into_diff().unwrap();
    let new = Service::from_diff(diff).unwrap();
    assert_eq!(service, new);
  }
}
