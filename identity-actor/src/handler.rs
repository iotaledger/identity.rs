// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::account::Account;

use crate::types::{IdentityStorageRequest, IdentityStorageResponse};
use crate::IdentityRequestHandler;

pub struct IdentityStorageHandler {
  account: Account,
}

impl IdentityStorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    Ok(Self {
      account: Account::builder().build().await?,
    })
  }
}

#[async_trait::async_trait]
impl IdentityRequestHandler for IdentityStorageHandler {
  type Request = IdentityStorageRequest;
  type Response = IdentityStorageResponse;

  async fn handle(&mut self, request: Self::Request) -> identity_account::Result<Self::Response> {
    println!("Received {:?}", request);

    // TODO: PreProcessingHook

    let response = match request {
      IdentityStorageRequest::Create(req) => {
        let snapshot = self.account.create_identity(req).await?;

        let did = snapshot.identity().try_did()?;

        let document = self.account.resolve_identity(did).await?;

        IdentityStorageResponse::Create(document)
      }
      _ => todo!(),
    };

    println!("Returning: {:?}", response);

    Ok(response)

    // TODO: PostProcessingHook
  }
}
