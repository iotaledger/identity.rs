// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use std::collections::HashMap;
use std::str::FromStr;

use identity_iota_core::IotaDocument;

use serde;
use serde::Deserialize;
use serde::Serialize;


use super::DummySigner;
use super::Error;
use super::IdentityClient;
use super::IotaClientTrait;
use super::Multicontroller;
use super::Proposal;

use super::rpc_types::{
  IotaObjectData,
  OwnedObjectRef,
};
use super::types::base_types::{IotaAddress, ObjectID};
use super::types::id::UID;

#[derive(Debug, Deserialize, Serialize)]
pub struct OnChainIdentity {
  pub id: UID,
  pub did_doc: Multicontroller<Vec<u8>>,
  #[serde(skip)]
  pub obj_ref: Option<OwnedObjectRef>,
  // used to have something to return a reference for in getter test
  #[serde(skip)]
  pub proposals: HashMap<String, Proposal>,
}

impl OnChainIdentity {
  pub fn is_shared(&self) -> bool {
    true
  }

  pub fn proposals(&self) -> &HashMap<String, Proposal> {
    &self.proposals
  }

  pub fn update_did_document<T>(self, updated_doc: IotaDocument) -> ProposalBuilder
  where
    T: IotaClientTrait<Error = Error>,
  {
    ProposalBuilder::new(self, ProposalAction::UpdateDocument(updated_doc))
  }

  pub fn deactivate_did<T>(self) -> ProposalBuilder
  where
    T: IotaClientTrait<Error = Error>,
  {
    ProposalBuilder::new(self, ProposalAction::Deactivate)
  }

  pub async fn get_history<T>(
    &self,
    _client: &IdentityClient<T>,
    _last_version: Option<&IotaObjectData>,
    _page_size: Option<usize>,
  ) -> Result<Vec<IotaObjectData>, Error>
  where
    T: IotaClientTrait,
  {
    Ok(vec![])
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProposalAction {
  UpdateDocument(IotaDocument),
  Deactivate,
}

#[derive(Debug)]
pub struct ProposalBuilder {}

impl ProposalBuilder {
  pub fn new(_identity: OnChainIdentity, _action: ProposalAction) -> Self {
    Self {}
  }

  pub fn expiration_epoch(self, _exp: u64) -> Self {
    self
  }

  pub fn key(self, _key: String) -> Self {
    self
  }

  pub fn gas_budget(self, _amount: u64) -> Self {
    self
  }

  pub async fn finish<C>(self, _client: &IdentityClient<C>, _signer: &DummySigner) -> Result<Option<Proposal>, Error>
  where
    C: IotaClientTrait,
  {
    Ok(Some(Proposal {}))
  }
}

#[derive(Debug)]
pub struct IdentityBuilder {}

impl IdentityBuilder {
  pub fn new(_did_doc: &[u8], _package_id: ObjectID) -> Self {
    Self {}
  }

  pub fn controller(self, _address: IotaAddress, _voting_power: u64) -> Self {
    self
  }

  pub fn threshold(self, _threshold: u64) -> Self {
    self
  }

  pub fn gas_budget(self, _gas_budget: u64) -> Self {
    self
  }

  pub fn controllers<I>(self, _controllers: I) -> Self
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    self
  }

  pub async fn finish<C>(self, _client: &IdentityClient<C>, _signer: &DummySigner) -> Result<OnChainIdentity, Error>
  where
    C: IotaClientTrait,
  {
    Ok(OnChainIdentity {
      id: UID::new(
        ObjectID::from_str("did:iota:foobar:0x0000000000000000000000000000000000000000000000000000000000000000")
            .map_err(|e| Error::Dummy(e.to_string()) )?
      ),
      did_doc: Multicontroller::new(vec![1u8, 2u8, 3u8]),
      obj_ref: None,
      proposals: HashMap::new(),
    })
  }
}
