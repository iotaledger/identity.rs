// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use crate::types::ToKey;
use crate::types::Index;

// =============================================================================
// Document Chain Metadata Location
// =============================================================================

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MetaLocation {
  chain: Index,
}

impl MetaLocation {
  pub const fn new(chain: Index) -> Self {
    Self { chain }
  }
}

impl ToKey for MetaLocation {
  const TAG: char = 'm';

  fn id(&self) -> String {
    self.chain.to_string()
  }
}

// =============================================================================
// Compiled Document Location
// =============================================================================

pub struct DocLocation {
  chain: Index,
}

impl DocLocation {
  pub const fn new(chain: Index) -> Self {
    Self { chain }
  }
}

impl ToKey for DocLocation {
  const TAG: char = 'c';

  fn id(&self) -> String {
    self.chain.to_string()
  }
}

// =============================================================================
// Auth Chain Location
// =============================================================================

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthLocation {
  chain: Index,
  index: Index,
}

impl AuthLocation {
  pub const fn new(chain: Index) -> Self {
    Self::with_index(chain, Index::new())
  }

  pub const fn with_index(chain: Index, index: Index) -> Self {
    Self { chain, index }
  }
}

impl ToKey for AuthLocation {
  const TAG: char = 'a';

  fn id(&self) -> String {
    format!("{}:{}", self.chain.get(), self.index)
  }
}

// =============================================================================
// Diff Chain Location
// =============================================================================

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct DiffLocation {
  scope: AuthLocation,
  index: Index,
}

impl DiffLocation {
  pub const fn new(scope: AuthLocation) -> Self {
    Self::with_index(scope, Index::new())
  }

  pub const fn with_index(scope: AuthLocation, index: Index) -> Self {
    Self { scope, index }
  }
}

impl ToKey for DiffLocation {
  const TAG: char = 'd';

  fn id(&self) -> String {
    format!("{}:{}", self.scope.id(), self.index)
  }
}

// =============================================================================
// Key Location
// =============================================================================

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyLocation<'a> {
  scope: AuthLocation,
  ident: Cow<'a, str>,
}

impl<'a> KeyLocation<'a> {
  pub fn new<T>(scope: AuthLocation, ident: T) -> Self
  where
    T: Into<Cow<'a, str>>,
  {
    Self {
      scope,
      ident: ident.into(),
    }
  }
}

impl ToKey for KeyLocation<'_> {
  const TAG: char = 'k';

  fn id(&self) -> String {
    format!("{}:{}", self.scope.id(), self.ident)
  }
}
