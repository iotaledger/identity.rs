// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account::account::Account;
use identity_account::identity::IdentityCreate;
use identity_iota::did::{IotaDID, IotaDocument};
use serde::{Deserialize, Serialize};

use crate::{traits::ActorRequest, IdentityList};

use super::requests::IdentityResolve;

#[derive(Clone)]
pub struct IdentityStorageHandler {
  account: Arc<Account>,
}

impl IdentityStorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    Ok(Self {
      account: Arc::new(Account::builder().build().await?),
    })
  }

  pub async fn create(self, _input: IdentityCreate) -> IotaDocument {
    todo!()
  }

  pub async fn list(self, _input: IdentityList) -> Vec<IotaDID> {
    vec![]
  }

  pub async fn resolve(self, _input: IdentityResolve) -> Option<IotaDocument> {
    None
  }
}
