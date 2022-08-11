// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use async_trait::async_trait;

use identity_did::{did::DID, document::Document};

#[async_trait]
/// A parameterized trait for handling resolution of DID Documents using a specified DID method.
pub trait ResolutionHandler<D>
where
  D: DID + for<'a> TryFrom<&'a str> + Send + 'static,
{
  type Resolved: Document + 'static + Send;

  /// Fetch the associated DID Document from the given DID.
  async fn resolve(&self, did: &D) -> Result<Self::Resolved>;

  /// The supported did method.
  /// The returned string is expected to match the `did-method-name` when parsing DIDs of the method this handler requires.
  fn method() -> String;
}
