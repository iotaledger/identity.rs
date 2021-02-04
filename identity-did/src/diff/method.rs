// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::Error;
use identity_core::diff::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::did::DID;
use crate::diff::DiffMethodData;
use crate::verification::Method;
use crate::verification::MethodData;
use crate::verification::MethodType;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMethod<T = ()>
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

    Ok(Method {
      id,
      controller,
      key_type,
      key_data,
      properties,
    })
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

    Ok(Method {
      id,
      controller,
      key_type,
      key_data,
      properties,
    })
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffMethod {
      id: Some(self.id().to_string().into_diff()?),
      controller: Some(self.controller().to_string().into_diff()?),
      key_type: Some(self.key_type().into_diff()?),
      key_data: Some(self.key_data().clone().into_diff()?),
      properties: Some(self.properties().clone().into_diff()?),
    })
  }
}
