// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Formatter;

use std::fmt::Display;

use crate::StardustDID;
use crate::StardustDocument;

use identity_did::did::CoreDID;

use identity_did::document::CoreDocument;
use identity_resolver::Error;
use identity_resolver::Result;

#[derive(Clone)]
pub(super) struct FooClient {
  pub(super) issuer_stardust_doc: StardustDocument,
}

pub(super) struct BarClient {
  pub(super) cache: Vec<CoreDocument>,
}

#[derive(Debug)]
pub(super) struct ResolutionError(String);
impl Display for ResolutionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "could not resolve DID: {}", self.0)
  }
}
impl std::error::Error for ResolutionError {}

impl FooClient {
  pub(super) async fn resolve(&self, did: &StardustDID) -> Result<StardustDocument> {
    if did == self.issuer_stardust_doc.id() {
      Ok(self.issuer_stardust_doc.clone())
    } else {
      Err(Error::HandlerError(Box::new(ResolutionError(did.to_string()))))
    }
  }
}

impl BarClient {
  pub(super) async fn resolve(&self, did: &CoreDID) -> Result<CoreDocument> {
    self
      .cache
      .iter()
      .find(|doc| doc.id() == did.as_ref())
      .map(Clone::clone)
      .ok_or(Error::HandlerError(Box::new(ResolutionError(did.to_string()))))
  }
}
