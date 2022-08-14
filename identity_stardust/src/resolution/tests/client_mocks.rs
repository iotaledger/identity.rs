// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::{ResolutionHandler, StardustDID, StardustDocument};
use async_trait::async_trait;
use identity_did::{
  did::{BaseDIDUrl, CoreDID, DIDError, DID},
  document::CoreDocument,
};

/// Mock client capable of resolving DIDs of the following method: StardustDID::METHOD (= "stardust")
pub(super) struct FooClient {
  pub(super) issuer_stardust_doc: StardustDocument,
}

/// Mock client capable of resolving DIDs of the two following methods: TestDID::<0>::method_name() (= "test0") and TestDID::<1>::method_name() (= "test1").
pub(super) struct BarClient {
  pub(super) cache: Vec<CoreDocument>,
}

/// We use const generics in these tests to avoid code repetition. We could also have used a macro_rules macro, but this is arguably more readable.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
pub(super) struct TestDID<const NUMBER: u8>(CoreDID);

impl<const NUMBER: u8> TestDID<NUMBER> {
  pub(super) fn method_name() -> String {
    format!("test{}", NUMBER)
  }
}

impl<const NUMBER: u8> TestDID<NUMBER> {
  fn validate(did: &CoreDID) -> Result<(), DIDError> {
    if did.method() != Self::method_name() {
      Err(DIDError::InvalidMethodName)
    } else {
      Ok(())
    }
  }
}

impl<const NUMBER: u8> AsRef<CoreDID> for TestDID<NUMBER> {
  fn as_ref(&self) -> &CoreDID {
    &self.0
  }
}

impl<const NUMBER: u8> FromStr for TestDID<NUMBER> {
  type Err = DIDError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let did = CoreDID::parse(s)?;
    TestDID::<NUMBER>::validate(&did)?;
    Ok(Self(did))
  }
}

impl<const NUMBER: u8> TryFrom<BaseDIDUrl> for TestDID<NUMBER> {
  type Error = DIDError;

  fn try_from(base_did_url: BaseDIDUrl) -> Result<Self, Self::Error> {
    let did = CoreDID::try_from(base_did_url)?;
    Self::validate(&did)?;
    Ok(Self(did))
  }
}

impl<const NUMBER: u8> TryFrom<&str> for TestDID<NUMBER> {
  type Error = DIDError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    value.parse()
  }
}

impl<const NUMBER: u8> From<TestDID<NUMBER>> for String {
  fn from(value: TestDID<NUMBER>) -> Self {
    value.0.into_string()
  }
}

#[async_trait]
impl ResolutionHandler<StardustDID> for FooClient {
  type Resolved = StardustDocument;
  fn method() -> String {
    StardustDID::METHOD.into()
  }
  async fn resolve(&self, did: &StardustDID) -> crate::Result<StardustDocument> {
    if did == self.issuer_stardust_doc.id() {
      Ok(self.issuer_stardust_doc.clone())
    } else {
      Err(crate::Error::ResolutionProblem("could not resovle did".into()))
    }
  }
}

#[async_trait]
impl<const NUMBER: u8> ResolutionHandler<TestDID<NUMBER>> for BarClient {
  type Resolved = CoreDocument;
  fn method() -> String {
    TestDID::<NUMBER>::method_name()
  }
  async fn resolve(&self, did: &TestDID<NUMBER>) -> crate::Result<CoreDocument> {
    self
      .cache
      .iter()
      .find(|doc| doc.id() == did.as_ref())
      .map(Clone::clone)
      .ok_or(crate::Error::ResolutionProblem("could not resolve document".into()))
  }
}
