// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
pub struct DiffService<T = ()>
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
