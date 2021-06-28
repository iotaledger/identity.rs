// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::account::Account;
use identity_iota::did::IotaDocument;
use core::convert;
use futures::stream::FuturesOrdered;
use futures::TryStreamExt;

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
      IdentityStorageRequest::List => {
        let list = self
          .account
          .list_identities()
          .await
          .iter()
          .map(|tag| self.account.find_identity(tag.method_id()))
          .collect::<FuturesOrdered<_>>()
          .try_collect::<Vec<_>>()
          .await?
          .into_iter()
          .filter_map(convert::identity)
          .map(|snapshot| snapshot.identity().to_document())
          .collect::<identity_account::Result<Vec<_>>>()?;

        IdentityStorageResponse::List(list)
      }
      _ => todo!(),
    };

    println!("Returning: {:?}", response);

    Ok(response)

    // TODO: PostProcessingHook
  }
}
