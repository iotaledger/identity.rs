// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::once;
use identity_core::common::Object;
use identity_core::convert::ToJson;
use serde::Serialize;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::verification::MethodBuilder;
use crate::verification::MethodData;
use crate::verification::MethodRef;
use crate::verification::MethodType;

/// A DID Document Verification Method.
///
/// [Specification](https://www.w3.org/TR/did-core/#verification-method-properties)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerificationMethod<T = Object> {
  pub(crate) id: DID,
  pub(crate) controller: DID,
  #[serde(rename = "type")]
  pub(crate) key_type: MethodType,
  #[serde(flatten)]
  pub(crate) key_data: MethodData,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T> VerificationMethod<T> {
  /// Creates a `MethodBuilder` to configure a new `Method`.
  ///
  /// This is the same as `MethodBuilder::new()`.
  pub fn builder(properties: T) -> MethodBuilder<T> {
    MethodBuilder::new(properties)
  }

  /// Returns a new `Method` based on the `MethodBuilder` configuration.
  pub fn from_builder(builder: MethodBuilder<T>) -> Result<Self> {
    Ok(VerificationMethod {
      id: builder.id.ok_or(Error::BuilderInvalidMethodId)?,
      controller: builder.controller.ok_or(Error::BuilderInvalidMethodController)?,
      key_type: builder.key_type.ok_or(Error::BuilderInvalidMethodType)?,
      key_data: builder.key_data.ok_or(Error::BuilderInvalidMethodData)?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the verification `Method` id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the verification `Method` id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Returns a reference to the verification `Method` controller.
  pub fn controller(&self) -> &DID {
    &self.controller
  }

  /// Returns a mutable reference to the verification `Method` controller.
  pub fn controller_mut(&mut self) -> &mut DID {
    &mut self.controller
  }

  /// Returns a reference to the verification `Method` type.
  pub fn key_type(&self) -> MethodType {
    self.key_type
  }

  /// Returns a mutable reference to the verification `Method` type.
  pub fn key_type_mut(&mut self) -> &mut MethodType {
    &mut self.key_type
  }

  /// Returns a reference to the verification `Method` data.
  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  /// Returns a mutable reference to the verification `Method` data.
  pub fn key_data_mut(&mut self) -> &mut MethodData {
    &mut self.key_data
  }

  /// Returns a reference to the custom verification `Method` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom verification `Method` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  pub fn try_into_fragment(&self) -> Result<String> {
    self
      .id
      .fragment()
      .ok_or(Error::InvalidMethodFragment)
      .map(|fragment| once('#').chain(fragment.chars()).collect())
  }

  /// Creates a new [`MethodRef`] from `self`.
  pub fn into_ref(self) -> MethodRef<T> {
    MethodRef::Embed(self)
  }
}

impl<T> Display for VerificationMethod<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

impl<T> AsRef<DID> for VerificationMethod<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
