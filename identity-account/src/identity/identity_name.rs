// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use crate::identity::IdentityId;

/// Represents an Identity name, whether explicitly set or default.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdentityName {
  Default,
  Literal(String),
}

impl IdentityName {
  /// Returns the user-assigned name, if any.
  pub fn as_opt(&self) -> Option<&str> {
    match self {
      Self::Default => None,
      Self::Literal(ref inner) => Some(inner),
    }
  }

  /// Returns the name of the identity, whether user-assigned or default.
  pub fn as_str(&self, id: IdentityId) -> Cow<'_, str> {
    match self {
      Self::Default => Cow::Owned(default_identifier(id)),
      Self::Literal(ref inner) => Cow::Borrowed(inner),
    }
  }
}

fn default_identifier(id: IdentityId) -> String {
  format!("Identity {}", id.to_u32())
}
