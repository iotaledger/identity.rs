// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::IdentityList;
use identity_account::account::Account;
use identity_account::identity::IdentityCreate;
use identity_iota::did::{IotaDID, IotaDocument};

use super::requests::IdentityResolve;

#[derive(Clone)]
pub struct StorageHandler {
  account: Arc<Account>,
}

impl StorageHandler {
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

  pub async fn resolve(self, input: IdentityResolve) -> Option<IotaDocument> {
    Some(self.account.resolve_identity(input.0).await.unwrap())
  }
}
