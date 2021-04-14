// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;
use identity_did::verification::Method;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;
use identity_iota::did::DID;

use crate::error::Result;
use crate::traits::Integer;
use crate::types::Fragment;
use crate::types::Index;

#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChainKey {
  #[serde(rename = "type")]
  pub(crate) type_: MethodType,
  pub(crate) auth: Index,
  pub(crate) diff: Index,
  #[serde(rename = "frag")]
  pub(crate) fragment: Fragment,
}

impl ChainKey {
  pub const AUTH: &'static str = "_auth";

  pub fn auth(type_: MethodType, index: Index) -> Self {
    Self {
      type_,
      auth: index,
      diff: Index::ZERO,
      fragment: Fragment::new(Self::AUTH.into()),
    }
  }

  pub fn type_(&self) -> MethodType {
    self.type_
  }

  pub fn auth_index(&self) -> Index {
    self.auth
  }

  pub fn diff_index(&self) -> Index {
    self.diff
  }

  pub fn fragment(&self) -> &str {
    self.fragment.value()
  }

  pub fn to_core(&self, document: &DID, key_data: MethodData, properties: Object) -> Result<Method> {
    Method::builder(properties)
      .id(document.join(self.fragment.ident()).map(Into::into)?)
      .controller(document.clone().into())
      .key_type(self.type_)
      .key_data(key_data)
      .build()
      .map_err(Into::into)
  }

  pub fn encode(&self) -> Vec<u8> {
    let fragment: &[u8] = self.fragment.value().as_bytes();
    let mut output: Vec<u8> = Vec::with_capacity(12 + fragment.len());
    output.extend_from_slice(&self.type_.as_u32().to_be_bytes());
    output.extend_from_slice(&self.auth.to_bytes());
    output.extend_from_slice(&self.diff.to_bytes());
    output.extend_from_slice(fragment);
    output
  }

  pub fn decode(data: &[u8]) -> Option<Self> {
    let type_: u32 = data.get(..4).and_then(u32::decode_opt)?;
    let auth: u32 = data.get(4..8).and_then(u32::decode_opt)?;
    let diff: u32 = data.get(8..12).and_then(u32::decode_opt)?;

    let fragment: Fragment = data
      .get(12..)
      .and_then(|data| String::from_utf8(data.into()).ok())
      .map(Fragment::new)?;

    Some(Self {
      type_: MethodType::from_u32(type_)?,
      auth: auth.into(),
      diff: diff.into(),
      fragment,
    })
  }
}

impl Display for ChainKey {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!(
      "({}:{}:{}:{})",
      self.auth,
      self.diff,
      self.fragment,
      self.type_.as_u32()
    ))
  }
}

impl Debug for ChainKey {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("ChainKey{}", self))
  }
}
