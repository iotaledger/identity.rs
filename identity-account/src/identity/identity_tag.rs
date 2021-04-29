// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use core::hash::Hash;
use core::hash::Hasher;
use std::borrow::Cow;

use crate::identity::IdentityId;
use crate::identity::IdentityName;

/// Information used to identify an identity.
#[derive(Clone, Deserialize, Serialize)]
pub struct IdentityTag {
  name: IdentityName,
  method_id: String,
}

impl IdentityTag {
  /// Creates a new IdentityTag with a default name.
  pub fn new(method_id: String) -> Self {
    Self {
      name: IdentityName::Default,
      method_id,
    }
  }

  /// Creates a new IdentityTag with an explicit name.
  pub fn named(method_id: String, name: String) -> Self {
    Self {
      name: IdentityName::Literal(name),
      method_id,
    }
  }

  /// Returns the user-assigned name of the identity.
  pub fn name(&self) -> Option<&str> {
    self.name.as_opt()
  }

  /// Returns the name of the identity, whether user-assigned or default.
  pub fn fullname(&self, id: IdentityId) -> Cow<'_, str> {
    self.name.as_str(id)
  }

  /// Returns the method id of the Identity DID Document.
  pub fn method_id(&self) -> &str {
    &self.method_id
  }
}

impl Debug for IdentityTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_fmt(format_args!("IdentityTag({}, {:?})", self.method_id, self.name))
  }
}

impl PartialEq for IdentityTag {
  fn eq(&self, other: &Self) -> bool {
    self.method_id.eq(&other.method_id)
  }
}

impl Eq for IdentityTag {}

impl Hash for IdentityTag {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.method_id.hash(hasher);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compare() {
    let a: IdentityTag = IdentityTag::new("abcde".into());
    let b: IdentityTag = IdentityTag::named("abcde".into(), "Foo".into());

    assert_eq!(a, b);
    assert_eq!(a.method_id(), b.method_id());
    assert_ne!(a.name(), b.name());
  }
}
